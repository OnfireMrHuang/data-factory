use dioxus::prelude::*;
use crate::models::collection::{
    TestStep, TestStepStatus, LogLevel, LogEntry, CollectionCategory, CollectType,
    IncrementalRuleForm, collect_test_run_step
};
use crate::models::datasource::DataSource;
use crate::models::resource::Resource;
use crate::api::test_run_mock::{TestRunMockApi, PreviewDataResponse};
use chrono::Local;

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
                                    TestStepStatus::Success => "bg-success",
                                    TestStepStatus::Failed => "bg-error",
                                    TestStepStatus::Running => "bg-primary",
                                    _ => ""
                                }
                            }
                        }

                        div { class: "timeline-start timeline-box",
                            div { class: "flex items-center gap-2",
                                // Status icon
                                match step.status {
                                    TestStepStatus::Pending => rsx! {
                                        span { class: "loading loading-ring loading-sm text-gray-400" }
                                    },
                                    TestStepStatus::Running => rsx! {
                                        span { class: "loading loading-spinner loading-sm text-primary" }
                                    },
                                    TestStepStatus::Success => rsx! {
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
                                    TestStepStatus::Failed => rsx! {
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
                                    TestStepStatus::Skipped => rsx! {
                                        svg { class: "w-5 h-5 text-gray-400",
                                            xmlns: "http://www.w3.org/2000/svg",
                                            fill: "none",
                                            view_box: "0 0 24 24",
                                            stroke_width: "2",
                                            stroke: "currentColor",
                                            path {
                                                stroke_linecap: "round",
                                                stroke_linejoin: "round",
                                                d: "M8 7h8m0 0v8m0-8l-8 8"
                                            }
                                        }
                                    }
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
                                    Some(TestStepStatus::Success) => "bg-success",
                                    Some(TestStepStatus::Failed) => "bg-error",
                                    Some(TestStepStatus::Running) => "bg-primary",
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
    let mut current_test_step = use_signal(|| 0);
    let mut temp_table_name = use_signal(|| String::new());
    let mut test_completed = use_signal(|| false);

    // Clone values for use in both closure and rsx
    let task_name_clone = task_name.clone();
    let selected_mode_for_rsx = selected_mode.clone();
    let selected_datasource_id_for_rsx = selected_datasource_id.clone();
    let selected_queue_resource_id_for_rsx = selected_queue_resource_id.clone();
    let selected_database_resource_id_for_rsx = selected_database_resource_id.clone();
    let datasources_for_rsx = datasources.clone();
    let resources_for_rsx = resources.clone();

    // Test run handler
    let test_run_handler = move |_| {
        let selected_mode_clone = selected_mode.clone();
        let selected_category_clone = selected_category.clone();
        let selected_datasource_id_clone = selected_datasource_id.clone();
        let row_update_rule_clone = row_update_rule.clone();
        let field_update_rules_clone = field_update_rules.clone();
        let ddl_sql_clone = ddl_sql.clone();
        let select_sql_clone = select_sql.clone();

        spawn(async move {
            test_running.set(true);
            test_completed.set(false);
            test_logs.set(Vec::new());
            preview_data.set(None);

            // Initialize test steps based on collection mode
            let steps = collect_test_run_step(Some(selected_category_clone.clone()), Some(selected_mode_clone.clone())).await;
            test_steps.set(steps.clone());

            // Add initial log
            let mut logs = test_logs();
            logs.push(LogEntry {
                timestamp: Local::now(),
                level: LogLevel::Info,
                message: "开始执行测试运行...".to_string(),
                details: None,
            });
            test_logs.set(logs);

            // Execute test steps
            for (idx, step) in steps.iter().enumerate() {
                current_test_step.set(idx);

                // Update step status to running
                let mut updated_steps = test_steps();
                updated_steps[idx].status = TestStepStatus::Running;
                updated_steps[idx].start_time = Some(Local::now());
                test_steps.set(updated_steps.clone());

                // Add log for step start
                let mut logs = test_logs();
                logs.push(LogEntry {
                    timestamp: Local::now(),
                    level: LogLevel::Info,
                    message: format!("执行步骤 {}: {}", step.id, step.title),
                    details: None,
                });
                test_logs.set(logs);

                // Execute step based on ID and mode
                let result = if selected_mode_clone == CollectType::Incremental {
                    match step.id {
                        1 => {
                            // Check database connection
                            TestRunMockApi::check_database_connection(
                                &selected_datasource_id_clone
                            ).await
                        }
                        2 => {
                            // Check row update rule
                            TestRunMockApi::check_row_update_rule(
                                &row_update_rule_clone.select_sql,
                                &row_update_rule_clone.monitor_table,
                                &row_update_rule_clone.monitor_sql
                            ).await
                        }
                        3 => {
                            // Check field update rules
                            TestRunMockApi::check_field_update_rules(&field_update_rules_clone).await
                        }
                        4 => {
                            // Full initial check
                            TestRunMockApi::check_full_initial(&ddl_sql_clone).await
                        }
                        5 => {
                            // Full initialization preview
                            match TestRunMockApi::preview_full_initialization(&row_update_rule_clone.select_sql).await {
                                Ok(data) => {
                                    preview_data.set(Some(data.clone()));
                                    Ok(crate::api::test_run_mock::TestStepResult {
                                        step_id: 5,
                                        success: true,
                                        message: format!("成功预览 {} 行数据", data.total_rows),
                                        error_message: None,
                                        data: None,
                                    })
                                }
                                Err(e) => Err(e),
                            }
                        }
                        _ => Ok(crate::api::test_run_mock::TestStepResult {
                            step_id: step.id,
                            success: false,
                            message: "Unknown step".to_string(),
                            error_message: Some("Step not implemented".to_string()),
                            data: None,
                        }),
                    }
                } else {
                    match step.id {
                        1 => {
                            // Check database connection
                            TestRunMockApi::check_database_connection(
                                &selected_datasource_id_clone
                            ).await
                        }
                        2 => {
                            // Create temporary table
                            let result = TestRunMockApi::create_temp_table(&ddl_sql_clone).await;
                            if let Ok(ref step_result) = result {
                                if let Some(data) = &step_result.data {
                                    if let Some(table_name) = data.get("table_name") {
                                        if let Some(name) = table_name.as_str() {
                                            temp_table_name.set(name.to_string());
                                        }
                                    }
                                }
                            }
                            result
                        }
                        3 => {
                            // Check field consistency
                            TestRunMockApi::check_field_consistency(&select_sql_clone, &ddl_sql_clone).await
                        }
                        4 => {
                            // Execute single collection
                            TestRunMockApi::execute_single_collection(&select_sql_clone).await
                        }
                        5 => {
                            // Preview temp table
                            match TestRunMockApi::preview_temp_table(&temp_table_name()).await {
                                Ok(data) => {
                                    preview_data.set(Some(data.clone()));
                                    Ok(crate::api::test_run_mock::TestStepResult {
                                        step_id: 5,
                                        success: true,
                                        message: format!("成功预览 {} 行数据", data.total_rows),
                                        error_message: None,
                                        data: None,
                                    })
                                }
                                Err(e) => Err(e),
                            }
                        }
                        6 => {
                            // Delete temp table
                            TestRunMockApi::delete_temp_table(&temp_table_name()).await
                        }
                        _ => Ok(crate::api::test_run_mock::TestStepResult {
                            step_id: step.id,
                            success: false,
                            message: "Unknown step".to_string(),
                            error_message: Some("Step not implemented".to_string()),
                            data: None,
                        }),
                    }
                };

                // Update step based on result
                let mut updated_steps = test_steps();
                match result {
                    Ok(step_result) => {
                        updated_steps[idx].status = if step_result.success {
                            TestStepStatus::Success
                        } else {
                            TestStepStatus::Failed
                        };
                        updated_steps[idx].error_message = step_result.error_message;
                        updated_steps[idx].end_time = Some(Local::now());

                        // Add log for step result
                        let mut logs = test_logs();
                        logs.push(LogEntry {
                            timestamp: Local::now(),
                            level: if step_result.success { LogLevel::Success } else { LogLevel::Error },
                            message: step_result.message.clone(),
                            details: step_result.data.map(|d| d.to_string()),
                        });
                        test_logs.set(logs);

                        // Stop if step failed
                        if !step_result.success {
                            test_steps.set(updated_steps);
                            break;
                        }
                    }
                    Err(e) => {
                        updated_steps[idx].status = TestStepStatus::Failed;
                        updated_steps[idx].error_message = Some(e.clone());
                        updated_steps[idx].end_time = Some(Local::now());

                        // Add error log
                        let mut logs = test_logs();
                        logs.push(LogEntry {
                            timestamp: Local::now(),
                            level: LogLevel::Error,
                            message: format!("步骤执行失败: {}", e),
                            details: None,
                        });
                        test_logs.set(logs);

                        test_steps.set(updated_steps);
                        break;
                    }
                }
                test_steps.set(updated_steps);
            }

            // Add completion log
            let mut logs = test_logs();
            logs.push(LogEntry {
                timestamp: Local::now(),
                level: LogLevel::Info,
                message: "测试运行完成".to_string(),
                details: None,
            });
            test_logs.set(logs);

            test_running.set(false);
            test_completed.set(true);
            on_test_completed.call(true);
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
                            current_step: current_test_step()
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
                        table_name: temp_table_name()
                    }
                }
            }
        }
    }
}
