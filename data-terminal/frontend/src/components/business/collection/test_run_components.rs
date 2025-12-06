use dioxus::prelude::*;
use crate::models::collection::{
    TestStep, TestRunStatus, LogLevel, LogEntry, CollectionCategory, CollectType,
    IncrementalRuleForm
};
use crate::models::test_run::{TestStepInfo, TestLogItem, LogLevel as ApiLogLevel};
use crate::models::datasource::DataSource;
use crate::models::resource::Resource;
use crate::api::collections;
use chrono::{Local, DateTime};
use gloo::timers::future::TimeoutFuture;

/// Preview data response (local type for UI)
#[derive(Debug, Clone)]
pub struct PreviewDataResponse {
    pub columns: Vec<String>,
    pub rows: Vec<Vec<String>>,
    pub total_rows: usize,
}

impl From<crate::models::test_run::TablePreviewResponse> for PreviewDataResponse {
    fn from(value: crate::models::test_run::TablePreviewResponse) -> Self {
        let total_rows = value.rows.len();
        Self {
            columns: value.columns,
            rows: value.rows,
            total_rows,
        }
    }
}

/// Convert API TestStepInfo to UI TestStep
fn convert_step_info(step_info: TestStepInfo) -> TestStep {
    // Convert API TestRunStatus to UI TestRunStatus
    let status = match step_info.status {
        crate::models::test_run::TestRunStatus::Pending => TestRunStatus::Pending,
        crate::models::test_run::TestRunStatus::Running => TestRunStatus::Running,
        crate::models::test_run::TestRunStatus::Success => TestRunStatus::Success,
        crate::models::test_run::TestRunStatus::Failed => TestRunStatus::Failed,
    };

    TestStep {
        id: step_info.id as usize,
        title: step_info.title,
        description: step_info.description,
        status,
        error_message: step_info.error_message,
        start_time: step_info.start_time.and_then(|s| DateTime::parse_from_rfc3339(&s).ok().map(|dt| dt.with_timezone(&Local))),
        end_time: step_info.end_time.and_then(|s| DateTime::parse_from_rfc3339(&s).ok().map(|dt| dt.with_timezone(&Local))),
    }
}

/// Convert API TestLogItem to UI LogEntry
fn convert_log_item(log_item: TestLogItem) -> LogEntry {
    let level = match log_item.level {
        ApiLogLevel::Info => LogLevel::Info,
        ApiLogLevel::Success => LogLevel::Success,
        ApiLogLevel::Warning => LogLevel::Warning,
        ApiLogLevel::Error => LogLevel::Error,
    };

    let timestamp = DateTime::parse_from_rfc3339(&log_item.timestamp)
        .ok()
        .map(|dt| dt.with_timezone(&Local))
        .unwrap_or_else(|| Local::now());

    LogEntry {
        timestamp,
        level,
        message: log_item.message,
        details: log_item.details,
    }
}


/// Test run timeline component
#[component]
pub fn TestRunTimeline(
    steps: Vec<TestStep>,
    current_step: usize,
) -> Element {
    rsx! {
        div { class: "w-full",
            ul { class: "timeline timeline-vertical",
                for (index, step) in steps.iter().enumerate() {
                    li { key: "{step.id}",
                        if index > 0 {
                            hr {
                                class: match step.status {
                                    TestRunStatus::Success => "bg-success",
                                    TestRunStatus::Failed => "bg-error",
                                    TestRunStatus::Running => "bg-primary",
                                    _ => ""
                                }
                            }
                        }

                        div { class: "timeline-start timeline-box",
                            div { class: "flex items-center gap-2",
                                // Status icon
                                match step.status {
                                    TestRunStatus::Pending => rsx! {
                                        span { class: "loading loading-ring loading-sm text-gray-400" }
                                    },
                                    TestRunStatus::Running => rsx! {
                                        span { class: "loading loading-spinner loading-sm text-primary" }
                                    },
                                    TestRunStatus::Success => rsx! {
                                        svg { class: "w-5 h-5 text-success",
                                            xmlns: "http://www.w3.org/2000/svg",
                                            fill: "none",
                                            view_box: "0 0 24 24",
                                            stroke_width: "2",
                                            stroke: "currentColor",
                                            path {
                                                stroke_linecap: "round",
                                                stroke_linejoin: "round",
                                                d: "M5 13l4 4L19 7"
                                            }
                                        }
                                    },
                                    TestRunStatus::Failed => rsx! {
                                        svg { class: "w-5 h-5 text-error",
                                            xmlns: "http://www.w3.org/2000/svg",
                                            fill: "none",
                                            view_box: "0 0 24 24",
                                            stroke_width: "2",
                                            stroke: "currentColor",
                                            path {
                                                stroke_linecap: "round",
                                                stroke_linejoin: "round",
                                                d: "M6 18L18 6M6 6l12 12"
                                            }
                                        }
                                    },
                                }

                                div {
                                    div { class: "font-semibold", "{step.title}" }
                                    div { class: "text-sm text-gray-500", "{step.description}" }
                                    if let Some(error_msg) = &step.error_message {
                                        div { class: "text-sm text-error mt-1", "{error_msg}" }
                                    }
                                    if let Some(start_time) = step.start_time {
                                        div { class: "text-xs text-gray-400 mt-1",
                                            {
                                                let time_str = start_time.format("%H:%M:%S").to_string();
                                                if let Some(end_time) = step.end_time {
                                                    format!("{} - {}", time_str, end_time.format("%H:%M:%S"))
                                                } else {
                                                    time_str
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        div {
                            class: format!("timeline-middle {}",
                                if index == current_step { "text-primary" } else { "" }
                            ),
                            svg {
                                xmlns: "http://www.w3.org/2000/svg",
                                view_box: "0 0 20 20",
                                fill: "currentColor",
                                class: "w-5 h-5",
                                path {
                                    fill_rule: "evenodd",
                                    d: "M10 18a8 8 0 100-16 8 8 0 000 16zm3.857-9.809a.75.75 0 00-1.214-.882l-3.483 4.79-1.88-1.88a.75.75 0 10-1.06 1.061l2.5 2.5a.75.75 0 001.137-.089l4-5.5z",
                                    clip_rule: "evenodd"
                                }
                            }
                        }

                        if index < steps.len() - 1 {
                            hr {
                                class: match steps.get(index + 1).map(|s| &s.status) {
                                    Some(TestRunStatus::Success) => "bg-success",
                                    Some(TestRunStatus::Failed) => "bg-error",
                                    Some(TestRunStatus::Running) => "bg-primary",
                                    _ => ""
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Test run log viewer component
#[component]
pub fn TestRunLogViewer(
    logs: Vec<LogEntry>,
    max_height: Option<String>,
) -> Element {
    let height = max_height.unwrap_or_else(|| "400px".to_string());

    rsx! {
        div { class: "w-full",
            div { class: "card bg-base-200",
                div { class: "card-body p-4",
                    h3 { class: "font-semibold mb-3", "执行日志" }
                    div {
                        class: "overflow-auto font-mono text-sm bg-base-300 rounded p-3",
                        style: "max-height: {height}; min-height: 200px;",
                        if logs.is_empty() {
                            div { class: "text-gray-500", "等待执行..." }
                        } else {
                            for log in logs.iter() {
                                div { class: "mb-2",
                                    div { class: "flex items-start gap-2",
                                        span { class: "text-gray-400 text-xs whitespace-nowrap",
                                            {log.timestamp.format("%H:%M:%S%.3f").to_string()}
                                        }
                                        span {
                                            class: match log.level {
                                                LogLevel::Info => "text-info",
                                                LogLevel::Success => "text-success",
                                                LogLevel::Warning => "text-warning",
                                                LogLevel::Error => "text-error",
                                            },
                                            match log.level {
                                                LogLevel::Info => "[INFO]",
                                                LogLevel::Success => "[SUCCESS]",
                                                LogLevel::Warning => "[WARN]",
                                                LogLevel::Error => "[ERROR]",
                                            }
                                        }
                                        span { class: "flex-1", "{log.message}" }
                                    }
                                    if let Some(details) = &log.details {
                                        div { class: "ml-16 text-gray-500 text-xs mt-1 whitespace-pre-wrap",
                                            "{details}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Data preview table component
#[component]
pub fn DataPreviewTable(
    columns: Vec<String>,
    rows: Vec<Vec<String>>,
    table_name: String,
) -> Element {
    rsx! {
        div { class: "w-full",
            div { class: "card bg-base-200",
                div { class: "card-body",
                    h3 { class: "font-semibold mb-3",
                        "数据预览 - {table_name}"
                        span { class: "text-sm text-gray-500 ml-2",
                            "({rows.len()} 行)"
                        }
                    }
                    div { class: "overflow-x-auto",
                        table { class: "table table-zebra",
                            thead {
                                tr {
                                    for col in columns.iter() {
                                        th { "{col}" }
                                    }
                                }
                            }
                            tbody {
                                if rows.is_empty() {
                                    tr {
                                        td {
                                            colspan: "{columns.len()}",
                                            class: "text-center text-gray-500",
                                            "暂无数据"
                                        }
                                    }
                                } else {
                                    for row in rows.iter() {
                                        tr {
                                            for cell in row.iter() {
                                                td {
                                                    class: "max-w-xs truncate",
                                                    title: "{cell}",
                                                    "{cell}"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
/// Database test run component - handles the full test run workflow
#[component]
pub fn DatabaseTestRun(
    // Form data
    task_code: String,
    task_name: String,
    selected_mode: CollectType,
    selected_category: CollectionCategory,
    selected_datasource_id: String,
    selected_queue_resource_id: Option<String>,
    selected_database_resource_id: String,
    ddl_sql: String,
    select_sql: String,
    row_update_rule: IncrementalRuleForm,
    field_update_rules: Vec<IncrementalRuleForm>,
    // Resources
    datasources: Vec<DataSource>,
    resources: Vec<Resource>,
    // Callbacks
    on_test_completed: EventHandler<bool>,
) -> Element {
    // Test run state
    let mut test_running = use_signal(|| false);
    let mut test_steps = use_signal(|| Vec::<TestStep>::new());
    let mut test_logs = use_signal(|| Vec::<LogEntry>::new());
    let mut preview_data = use_signal(|| None::<PreviewDataResponse>);
    let mut test_completed = use_signal(|| false);

    // Clone values for use in both closure and rsx
    let task_code_clone = task_code.clone();
    let task_name_clone = task_name.clone();
    let selected_mode_for_rsx = selected_mode.clone();
    let selected_datasource_id_for_rsx = selected_datasource_id.clone();
    let selected_queue_resource_id_for_rsx = selected_queue_resource_id.clone();
    let selected_database_resource_id_for_rsx = selected_database_resource_id.clone();
    let datasources_for_rsx = datasources.clone();
    let resources_for_rsx = resources.clone();

    // Test run handler
    let test_run_handler = move |_| {
        let collection_code = task_code_clone.clone();

        spawn(async move {
            test_running.set(true);
            test_completed.set(false);
            test_logs.set(Vec::new());
            preview_data.set(None);
            test_steps.set(Vec::new());

            // Add initial log
            let mut logs = Vec::new();
            logs.push(LogEntry {
                timestamp: Local::now(),
                level: LogLevel::Info,
                message: "正在启动测试运行...".to_string(),
                details: None,
            });
            test_logs.set(logs.clone());

            // Step 1: Execute test run to get task_id
            let task_id = match collections::execute_test_run(&collection_code).await {
                Ok(response) => {
                    logs.push(LogEntry {
                        timestamp: Local::now(),
                        level: LogLevel::Success,
                        message: format!("测试运行已启动，任务ID: {}", response.task_id),
                        details: None,
                    });
                    test_logs.set(logs.clone());
                    response.task_id
                }
                Err(e) => {
                    logs.push(LogEntry {
                        timestamp: Local::now(),
                        level: LogLevel::Error,
                        message: format!("启动测试运行失败: {}", e),
                        details: None,
                    });
                    test_logs.set(logs.clone());
                    test_running.set(false);
                    test_completed.set(true);
                    on_test_completed.call(false);
                    return;
                }
            };

            // Step 2: Poll for status until completion
            let mut last_log_timestamp: Option<i64> = None;
            let mut test_success = true;

            loop {
                // Wait 500ms before polling
                TimeoutFuture::new(500).await;

                // Get test run status
                match collections::get_test_run_status(&collection_code, &task_id).await {
                    Ok(status_response) => {
                        // Update steps
                        let steps: Vec<TestStep> = status_response.steps
                            .into_iter()
                            .map(convert_step_info)
                            .collect();
                        test_steps.set(steps.clone());

                        // Check if test is complete
                        let is_complete = matches!(
                            status_response.status,
                            crate::models::test_run::TestRunStatus::Success | crate::models::test_run::TestRunStatus::Failed
                        );

                        if matches!(status_response.status, crate::models::test_run::TestRunStatus::Failed) {
                            test_success = false;
                        }

                        // Get logs
                        if let Ok(logs_response) = collections::get_test_run_logs(
                            &collection_code,
                            &task_id,
                            last_log_timestamp,
                        ).await {
                            // Convert and append new logs
                            let new_logs: Vec<LogEntry> = logs_response.items
                                .into_iter()
                                .map(convert_log_item)
                                .collect();

                            if !new_logs.is_empty() {
                                let mut current_logs = test_logs();
                                current_logs.extend(new_logs);
                                test_logs.set(current_logs);

                                // Update last timestamp
                                if let Some(last_log) = test_logs().last() {
                                    last_log_timestamp = Some(last_log.timestamp.timestamp_millis());
                                }
                            }
                        }

                        // If complete, get preview data and exit loop
                        if is_complete {
                            if test_success {
                                match collections::get_test_run_preview(&collection_code, &task_id).await {
                                    Ok(preview_response) => {
                                        preview_data.set(Some(PreviewDataResponse {
                                            columns: preview_response.columns,
                                            rows: preview_response.rows.clone(),
                                            total_rows: preview_response.rows.len(),
                                        }));
                                        logs.push(LogEntry {
                                            timestamp: Local::now(),
                                            level: LogLevel::Success,
                                            message: "测试运行成功完成".to_string(),
                                            details: None,
                                        });
                                    }
                                    Err(e) => {
                                        logs.push(LogEntry {
                                            timestamp: Local::now(),
                                            level: LogLevel::Warning,
                                            message: format!("无法获取预览数据: {}", e),
                                            details: None,
                                        });
                                    }
                                }
                            } else {
                                logs.push(LogEntry {
                                    timestamp: Local::now(),
                                    level: LogLevel::Error,
                                    message: "测试运行失败".to_string(),
                                    details: None,
                                });
                            }
                            test_logs.set(logs.clone());
                            break;
                        }
                    }
                    Err(e) => {
                        logs.push(LogEntry {
                            timestamp: Local::now(),
                            level: LogLevel::Error,
                            message: format!("获取测试状态失败: {}", e),
                            details: None,
                        });
                        test_logs.set(logs.clone());
                        test_success = false;
                        break;
                    }
                }
            }

            test_running.set(false);
            test_completed.set(true);
            on_test_completed.call(test_success);
        });
    };

    /// Returns the name of the datasource with the given id, or an empty string if not found.
    fn get_datasource_name_by_id(datasource_id: &str, datasources: &Vec<DataSource>) -> String {
        datasources
            .iter()
            .find(|ds| ds.id == datasource_id)
            .map(|ds| ds.name.clone())
            .unwrap_or_else(|| "".to_string())
    }

    /// Returns the name of the database with the given id, or an empty string if not found.
    fn get_database_name_by_id(database_id: &str, databases: &Vec<Resource>) -> String {
        databases
            .iter()
            .find(|db| db.id == database_id)
            .map(|db| db.name.clone())
            .unwrap_or_else(|| "".to_string())
    }

    /// Returns the name of the queue with the given id, or an empty string if not found.
    fn get_queue_name_by_id(queue_id: &str, queues: &Vec<Resource>) -> String {
        queues
            .iter()
            .find(|q| q.id == queue_id)
            .map(|q| q.name.clone())
            .unwrap_or_else(|| "".to_string())
    }

    rsx! {
        // Only show summary and test button when test not started
        if !test_running() && !test_completed() {
            div { class: "space-y-4 mb-6",
                h3 { class: "font-semibold text-lg", "配置概览" }
                div { class: "grid grid-cols-2 gap-4",
                    div {
                        p { class: "text-sm text-gray-500", "任务名称" }
                        p { class: "font-medium", "{task_name_clone}" }
                    }
                    div {
                        p { class: "text-sm text-gray-500", "采集模式" }
                        p { class: "font-medium",
                            {match selected_mode_for_rsx {
                                CollectType::Full => "数据库全量采集",
                                CollectType::Incremental => "数据库增量采集",
                            }}
                        }
                    }
                    div {
                        p { class: "text-sm text-gray-500", "数据源" }
                        p { class: "font-medium", "{get_datasource_name_by_id(&selected_datasource_id_for_rsx, &datasources_for_rsx)}" }
                    }
                    {if selected_mode_for_rsx == CollectType::Incremental {
                        rsx! {
                            div {
                                p { class: "text-sm text-gray-500", "队列资源" }
                                p { class: "font-medium",
                                    "{get_queue_name_by_id(&selected_queue_resource_id_for_rsx.clone().unwrap_or_default(), &resources_for_rsx)}"
                                }
                            }
                        }
                    } else {
                        rsx! {}
                    }}
                    div {
                        p { class: "text-sm text-gray-500", "目标资源" }
                        p { class: "font-medium", "{get_database_name_by_id(&selected_database_resource_id_for_rsx, &resources_for_rsx)}" }
                    }
                }

                div { class: "divider" }

                div { class: "alert alert-info",
                    svg { class: "stroke-current shrink-0 w-6 h-6",
                        xmlns: "http://www.w3.org/2000/svg",
                        fill: "none",
                        view_box: "0 0 24 24",
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            stroke_width: "2",
                            d: "M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
                        }
                    }
                    span { "点击下方按钮开始测试运行,验证采集配置是否正确" }
                }

                div { class: "flex justify-center",
                    button {
                        class: "btn btn-primary btn-lg",
                        onclick: test_run_handler,
                        "开始测试运行"
                    }
                }
            }
        }

        // Show test running/completed UI
        if test_running() || test_completed() {
            div { class: "space-y-6",
                // Timeline and logs side by side
                div { class: "flex flex-row gap-6",
                    // Timeline (left side, fixed width 320px)
                    div { style: "width: 320px; min-width: 0; flex-shrink: 0;",
                        TestRunTimeline {
                            steps: test_steps(),
                            current_step: 0
                        }
                    }
                    // Log viewer (right side, takes remaining space)
                    div { style: "flex: 1 1 0; min-width: 0;",
                        TestRunLogViewer {
                            logs: test_logs(),
                            max_height: Some("300px".to_string())
                        }
                    }
                }

                // Data preview (show when available)
                if let Some(preview) = preview_data() {
                    DataPreviewTable {
                        columns: preview.columns,
                        rows: preview.rows,
                        table_name: task_name_clone.clone()
                    }
                }
            }
        }
    }
}
