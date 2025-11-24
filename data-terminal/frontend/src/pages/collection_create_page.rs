use dioxus::prelude::*;
use crate::routes::Route;
use crate::models::collection::*;
use crate::models::datasource::{DataSource, DataSourceType};
use crate::models::resource::{Resource, ResourceType};
use crate::components::business::collection::{
    DatasourceSelector, ResourceSelectorForDatabaseFull, ResourceSelectorForDatabaseIncremental,
    TestRunTimeline, TestRunLogViewer, DataPreviewTable, TestStep, TestStepStatus, LogEntry, LogLevel
};
use crate::api::collections;
use crate::api::test_run_mock::{TestRunMockApi, PreviewDataResponse};
use chrono::{Utc, Local};

const MONITOR_TABLE_OPTIONS: [&str; 4] = [
    "public.cdc_table_tracker",
    "public.order_monitor",
    "public.inventory_guard",
    "__custom__",
];


#[component]
fn TargetSchemaDefinition(
    collection_category: CollectionCategory,
    resource_type: ResourceType,
    ddl_sql: Signal<String>,
    json_schema: Signal<String>,
) -> Element {
    // todo: 目前仅支持数据库
    let resource_schema_title = match resource_type.clone() {
        ResourceType::Mysql => "Mysql建表语句",
        ResourceType::Postgres => "Postgresql建表语句",
        ResourceType::Doris => "doris建表语句",
        _ => "",
    };
    let resource_placeholder = match resource_type.clone() {
        ResourceType::Mysql => "CREATE TABLE target_table (\n  id INT PRIMARY KEY,\n  name VARCHAR(255),\n  created_at TIMESTAMP\n);",
        ResourceType::Postgres => "CREATE TABLE target_table (\n  id SERIAL PRIMARY KEY,\n  name VARCHAR(255),\n  created_at TIMESTAMP\n);",
        ResourceType::Doris => "CREATE TABLE target_table (\n  id INT PRIMARY KEY,\n  name VARCHAR(255),\n  created_at TIMESTAMP\n);",
        _ => "",
    };


    rsx! {
        div { class: "form-control mb-4 flex flex-col",
            label { class: "label mb-4",
                span { class: "label-text font-semibold", "{resource_schema_title}" }
            }
            if collection_category.clone() == CollectionCategory::Database {
                textarea {
                    class: "textarea textarea-primary font-mono w-full h-96",
                    placeholder: "{resource_placeholder}",
                    value: "{ddl_sql}",
                    oninput: move |evt| ddl_sql.set(evt.value())
                }
            } else {
                textarea {
                    class: "textarea textarea-primary font-mono w-full h-96",
                    placeholder: "{resource_placeholder}",
                    value: "{json_schema}",
                    oninput: move |evt| json_schema.set(evt.value())
                }
            }
        }
    }
}


#[component]
fn QueryDataRuleDefinition(
    collection_category: CollectionCategory,
    datasource_type: DataSourceType,
    query_sql: Signal<String>,
) -> Element {
    rsx!{
        div { class: "card-body",
            // Database category: SELECT SQL textarea
            if collection_category.clone() == CollectionCategory::Database {
                div { class: "form-control mb-4 flex flex-col",
                    label { class: "label mb-4",
                        span { class: "label-text font-semibold", "SELECT SQL" }
                        span { class: "label-text-alt", "从数据源提取数据的SQL查询" }
                    }
                    textarea {
                        class: "textarea textarea-bordered font-mono w-full h-96",
                        placeholder: "SELECT id, name, created_at\nFROM source_table\nWHERE status = 'active'\nORDER BY created_at DESC;",
                        value: "{query_sql}",
                        oninput: move |evt| query_sql.set(evt.value())
                    }
                }
            }
        }
    }
}


#[component]
fn IncrementalRuleDefinition(
    row_update_rule: Signal<IncrementalRuleForm>,
    field_update_rules: Signal<Vec<IncrementalRuleForm>>,
) -> Element {
    rsx! {
        div { class: "card-body space-y-6",
            // Row Update Rule Section
            div { class: "border border-base-300 rounded-lg p-4",
                h3 { class: "font-semibold text-lg mb-4", "行新增规则 (Row Update Rule)" }
                p { class: "text-sm text-gray-500 mb-4", "配置行新增的数据查询SQL和监控规则" }

                div { class: "form-control mb-4",
                    label { class: "label",
                        span { class: "label-text font-semibold", "SELECT SQL" }
                        span { class: "label-text-alt text-gray-500", "所有字段别名必须匹配目标表的所有字段名" }
                    }
                    textarea {
                        class: "textarea textarea-bordered font-mono w-full h-48",
                        placeholder: "SELECT \n  id, \n  name, \n  created_at, \n  updated_at\nFROM source_table\nWHERE status = 'active';",
                        value: "{row_update_rule().select_sql}",
                        oninput: move |evt| {
                            let mut rule = row_update_rule();
                            rule.select_sql = evt.value();
                            row_update_rule.set(rule);
                        }
                    }
                }

                div { class: "form-control mb-4",
                    label { class: "label",
                        span { class: "label-text font-semibold", "监控表 (Monitor Table)" }
                    }
                    select {
                        class: "select select-bordered w-full",
                        value: "{row_update_rule().monitor_table}",
                        onchange: move |evt| {
                            let mut rule = row_update_rule();
                            let value = evt.value();
                            if value == "__custom__" {
                                rule.use_custom_monitor_table = true;
                                rule.monitor_table = String::new();
                            } else {
                                rule.use_custom_monitor_table = false;
                                rule.monitor_table = value;
                            }
                            row_update_rule.set(rule);
                        },
                        option { value: "", disabled: true, selected: row_update_rule().monitor_table.is_empty(), "选择监控表..." }
                        for table_name in MONITOR_TABLE_OPTIONS.iter() {
                            option {
                                value: "{table_name}",
                                selected: row_update_rule().monitor_table == *table_name,
                                "{table_name}"
                            }
                        }
                    }
                }

                if row_update_rule().use_custom_monitor_table {
                    div { class: "form-control mb-4",
                        label { class: "label",
                            span { class: "label-text font-semibold", "自定义监控表名" }
                        }
                        input {
                            r#type: "text",
                            class: "input input-bordered w-full",
                            placeholder: "custom.monitor_table",
                            value: "{row_update_rule().monitor_table}",
                            oninput: move |evt| {
                                let mut rule = row_update_rule();
                                rule.monitor_table = evt.value();
                                row_update_rule.set(rule);
                            }
                        }
                    }
                }

                div { class: "form-control mb-4",
                    label { class: "label",
                        span { class: "label-text font-semibold", "监控SQL (Monitor SQL)" }
                        span { class: "label-text-alt text-gray-500", "用于检测数据变化的SQL查询" }
                    }
                    textarea {
                        class: "textarea textarea-bordered font-mono w-full h-32",
                        placeholder: "SELECT MAX(updated_at) FROM source_table;",
                        value: "{row_update_rule().monitor_sql}",
                        oninput: move |evt| {
                            let mut rule = row_update_rule();
                            rule.monitor_sql = evt.value();
                            row_update_rule.set(rule);
                        }
                    }
                }
            }

            // Field Update Rules Section
            div { class: "border border-base-300 rounded-lg p-4",
                div { class: "flex items-center justify-between mb-4",
                    div {
                        h3 { class: "font-semibold text-lg", "字段更新规则 (Field Update Rules)" }
                        p { class: "text-sm text-gray-500", "配置字段级别的增量更新规则" }
                    }
                    button {
                        class: "btn btn-sm btn-primary",
                        onclick: move |_| {
                            let mut rules = field_update_rules();
                            rules.push(IncrementalRuleForm::default());
                            field_update_rules.set(rules);
                        },
                        "+ 添加字段更新规则"
                    }
                }

                if field_update_rules().is_empty() {
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
                        span { "暂无字段更新规则，点击右上角按钮添加" }
                    }
                }

                for (idx , rule) in field_update_rules().iter().enumerate() {
                    div {
                        key: "{idx}",
                        class: "border border-base-200 rounded-lg p-4 mb-4 bg-base-100",

                        div { class: "flex items-center justify-between mb-3",
                            h4 { class: "font-medium", "字段更新规则 #{idx + 1}" }
                            button {
                                class: "btn btn-sm btn-ghost btn-circle",
                                onclick: move |_| {
                                    let mut rules = field_update_rules();
                                    rules.remove(idx);
                                    field_update_rules.set(rules);
                                },
                                "×"
                            }
                        }

                        div { class: "form-control mb-3",
                            label { class: "label",
                                span { class: "label-text", "SELECT SQL" }
                                span { class: "label-text-alt text-gray-500", "只需包含部分字段" }
                            }
                            textarea {
                                class: "textarea textarea-bordered textarea-sm font-mono w-full h-32",
                                placeholder: "SELECT id, status FROM source_table WHERE ...;",
                                value: "{rule.select_sql}",
                                oninput: move |evt| {
                                    let mut rules = field_update_rules();
                                    rules[idx].select_sql = evt.value();
                                    field_update_rules.set(rules);
                                }
                            }
                        }

                        div { class: "form-control mb-3",
                            label { class: "label",
                                span { class: "label-text", "监控表" }
                            }
                            select {
                                class: "select select-bordered select-sm w-full",
                                value: "{rule.monitor_table}",
                                onchange: move |evt| {
                                    let mut rules = field_update_rules();
                                    let value = evt.value();
                                    if value == "__custom__" {
                                        rules[idx].use_custom_monitor_table = true;
                                        rules[idx].monitor_table = String::new();
                                    } else {
                                        rules[idx].use_custom_monitor_table = false;
                                        rules[idx].monitor_table = value;
                                    }
                                    field_update_rules.set(rules);
                                },
                                option { value: "", disabled: true, selected: rule.monitor_table.is_empty(), "选择监控表..." }
                                for table_name in MONITOR_TABLE_OPTIONS.iter() {
                                    option {
                                        value: "{table_name}",
                                        selected: rule.monitor_table == *table_name,
                                        "{table_name}"
                                    }
                                }
                            }
                        }

                        if rule.use_custom_monitor_table {
                            div { class: "form-control mb-3",
                                label { class: "label",
                                    span { class: "label-text", "自定义监控表名" }
                                }
                                input {
                                    r#type: "text",
                                    class: "input input-bordered input-sm w-full",
                                    placeholder: "custom.monitor_table",
                                    value: "{rule.monitor_table}",
                                    oninput: move |evt| {
                                        let mut rules = field_update_rules();
                                        rules[idx].monitor_table = evt.value();
                                        field_update_rules.set(rules);
                                    }
                                }
                            }
                        }

                        div { class: "form-control mb-3",
                            label { class: "label",
                                span { class: "label-text", "监控SQL" }
                            }
                            textarea {
                                class: "textarea textarea-bordered textarea-sm font-mono w-full h-24",
                                placeholder: "SELECT COUNT(*) FROM source_table WHERE status = 'changed';",
                                value: "{rule.monitor_sql}",
                                oninput: move |evt| {
                                    let mut rules = field_update_rules();
                                    rules[idx].monitor_sql = evt.value();
                                    field_update_rules.set(rules);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}


/// T053: CollectionCreatePage - Multi-step wizard for creating collection tasks
#[component]
pub fn CollectionCreatePage() -> Element {
    let navigator = use_navigator();

    // Wizard state
    let mut current_step = use_signal(|| 1);

    // Form state
    let mut task_name = use_signal(String::new);
    let mut task_description = use_signal(String::new);
    let mut selected_mode = use_signal(|| None::<CollectType>);
    let mut selected_category = use_signal(|| None::<CollectionCategory>);
    let mut selected_datasource_id = use_signal(|| None::<String>);
    let mut selected_datasource_type = use_signal(|| None::<DataSourceType>);
    let mut selected_queue_resource_id = use_signal(|| None::<String>);
    let mut selected_database_resource_id = use_signal(|| None::<String>);
    let mut selected_database_resource_type = use_signal(|| None::<ResourceType>);

    // Step 3: Target schema definition
    let mut ddl_sql = use_signal(String::new);        // For Database category
    let mut json_schema = use_signal(String::new);    // For API category

    // Step 4: Collection rule definition
    let mut select_sql = use_signal(String::new);     // For Database category
    let mut python_script = use_signal(String::new);  // For API category

    // Step 4: Incremental rule definition
    let mut row_update_rule = use_signal(|| IncrementalRuleForm::default());
    let mut field_update_rules = use_signal(|| Vec::<IncrementalRuleForm>::new());

    // Step 5: Test run state
    let mut test_running = use_signal(|| false);
    let mut test_steps = use_signal(|| Vec::<TestStep>::new());
    let mut test_logs = use_signal(|| Vec::<LogEntry>::new());
    let mut preview_data = use_signal(|| None::<PreviewDataResponse>);
    let mut current_test_step = use_signal(|| 0);
    let mut temp_table_name = use_signal(|| String::new());
    let mut test_completed = use_signal(|| false);

    // Data
    let mut datasources = use_signal(|| Vec::<DataSource>::new());
    let mut resources = use_signal(|| Vec::<Resource>::new());

    let mut loading = use_signal(|| false);
    let mut error_msg = use_signal(|| String::new());

    // Load datasources and resources on mount
    use_effect(move || {
        spawn(async move {
            // fetch datasources
            match crate::api::datasources::fetch_datasources(1, 1000).await {
                Ok(list) => {
                    if let Some(category) = selected_category() {
                        datasources.set(
                            list.into_iter()
                                .filter(|item| category.matches_datasource_category(&item.category))
                                .collect()
                        );
                    } else {
                        datasources.set(list);
                    }
                }
                Err(e) => {
                    error_msg.set(format!("获取数据源失败: {:?}", e));
                }
            }
            // fetch resources
            match crate::api::resources::fetch_resources().await {
                Ok(list) => {
                    if let Some(category) = selected_mode() {
                        resources.set(
                            list.into_iter()
                                .filter(|item| category.matches_resource_category(&item.category))
                                .collect()
                        );
                    } else {
                        resources.set(list);
                    }
                }
                Err(e) => {
                    error_msg.set(format!("获取资源失败: {:?}", e));
                }
            }
        });
    });

    // Submit handler
    let submit_handler = move |_| {
        spawn(async move {
            loading.set(true);

            // Build the collection rule based on category and mode
            let rule = match selected_category() {
                Some(CollectionCategory::Database) => {
                    if selected_mode() == Some(CollectType::Incremental) {
                        // Build incremental rule with row_update_rule and field_update_rules
                        let field_rules: Vec<serde_json::Value> = field_update_rules()
                            .iter()
                            .map(|rule| {
                                serde_json::json!({
                                    "select_sql": rule.select_sql,
                                    "monitor_table": rule.monitor_table,
                                    "monitor_sql": rule.monitor_sql
                                })
                            })
                            .collect();

                        serde_json::json!({
                            "type": "database",
                            "mode": "incremental",
                            "ddl_sql": ddl_sql(),
                            "row_update_rule": {
                                "select_sql": row_update_rule().select_sql,
                                "monitor_table": row_update_rule().monitor_table,
                                "monitor_sql": row_update_rule().monitor_sql
                            },
                            "field_update_rules": field_rules
                        })
                    } else {
                        serde_json::json!({
                            "type": "database",
                            "mode": "full",
                            "ddl_sql": ddl_sql(),
                            "select_sql": select_sql()
                        })
                    }
                }
                Some(CollectionCategory::Api) => {
                    serde_json::json!({
                        "type": "api",
                        "mode": match selected_mode() {
                            Some(CollectType::Full) => "full",
                            Some(CollectType::Incremental) => "incremental",
                            None => "full"
                        },
                        "json_schema": json_schema(),
                        "python_script": python_script()
                    })
                }
                _ => {
                    error_msg.set("Invalid category selected".to_string());
                    loading.set(false);
                    return;
                }
            };

            let request = CreateCollectTaskRequest {
                name: task_name(),
                description: if task_description().is_empty() {
                    None
                } else {
                    Some(task_description())
                },
                category: selected_category().unwrap_or(CollectionCategory::Database),
                collect_type: selected_mode().unwrap_or(CollectType::Full),
                datasource_id: selected_datasource_id().unwrap_or_default(),
                queue_resource_id: selected_queue_resource_id().unwrap_or_default(),
                database_resource_id: selected_database_resource_id().unwrap_or_default(),
                rule,
            };

            match collections::create_collection_task(request).await {
                Ok(_task) => {
                    loading.set(false);
                    navigator.push(Route::CollectionPage {});
                }
                Err(e) => {
                    error_msg.set(format!("Failed to create task: {:?}", e));
                    loading.set(false);
                }
            }
        });
    };

    // Test run handler
    let test_run_handler = move |_| {
        spawn(async move {
            test_running.set(true);
            test_completed.set(false);
            test_logs.set(Vec::new());
            preview_data.set(None);

            // Initialize test steps based on collection mode
            let steps = if selected_mode() == Some(CollectType::Incremental) {
                vec![
                    TestStep {
                        id: 1,
                        title: "数据库连接检查".to_string(),
                        description: "验证数据源连接是否正常".to_string(),
                        status: TestStepStatus::Pending,
                        error_message: None,
                        start_time: None,
                        end_time: None,
                    },
                    TestStep {
                        id: 2,
                        title: "行新增规则检查".to_string(),
                        description: "验证行新增规则配置是否正确".to_string(),
                        status: TestStepStatus::Pending,
                        error_message: None,
                        start_time: None,
                        end_time: None,
                    },
                    TestStep {
                        id: 3,
                        title: "字段更新规则检查".to_string(),
                        description: "验证字段更新规则配置是否正确".to_string(),
                        status: TestStepStatus::Pending,
                        error_message: None,
                        start_time: None,
                        end_time: None,
                    },
                    TestStep {
                        id: 4,
                        title: "全量初始检查".to_string(),
                        description: "检查全量初始化配置".to_string(),
                        status: TestStepStatus::Pending,
                        error_message: None,
                        start_time: None,
                        end_time: None,
                    },
                    TestStep {
                        id: 5,
                        title: "全量初始化预览".to_string(),
                        description: "预览初始化数据".to_string(),
                        status: TestStepStatus::Pending,
                        error_message: None,
                        start_time: None,
                        end_time: None,
                    },
                ]
            } else {
                vec![
                    TestStep {
                        id: 1,
                        title: "数据库连接检查".to_string(),
                        description: "验证数据源连接是否正常".to_string(),
                        status: TestStepStatus::Pending,
                        error_message: None,
                        start_time: None,
                        end_time: None,
                    },
                    TestStep {
                        id: 2,
                        title: "创建临时目标表".to_string(),
                        description: "根据DDL创建临时测试表".to_string(),
                        status: TestStepStatus::Pending,
                        error_message: None,
                        start_time: None,
                        end_time: None,
                    },
                    TestStep {
                        id: 3,
                        title: "字段一致性检查".to_string(),
                        description: "检查取数字段与目标表字段是否匹配".to_string(),
                        status: TestStepStatus::Pending,
                        error_message: None,
                        start_time: None,
                        end_time: None,
                    },
                    TestStep {
                        id: 4,
                        title: "执行单次采集".to_string(),
                        description: "执行一次采集任务(最大20行)".to_string(),
                        status: TestStepStatus::Pending,
                        error_message: None,
                        start_time: None,
                        end_time: None,
                    },
                    TestStep {
                        id: 5,
                        title: "预览临时目标表".to_string(),
                        description: "查看采集到的数据".to_string(),
                        status: TestStepStatus::Pending,
                        error_message: None,
                        start_time: None,
                        end_time: None,
                    },
                    TestStep {
                        id: 6,
                        title: "删除临时目标表".to_string(),
                        description: "清理测试数据".to_string(),
                        status: TestStepStatus::Pending,
                        error_message: None,
                        start_time: None,
                        end_time: None,
                    },
                ]
            };
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
                let result = if selected_mode() == Some(CollectType::Incremental) {
                    match step.id {
                        1 => {
                            // Check database connection
                            TestRunMockApi::check_database_connection(
                                &selected_datasource_id().unwrap_or_default()
                            ).await
                        }
                        2 => {
                            // Check row update rule
                            TestRunMockApi::check_row_update_rule(
                                &row_update_rule().select_sql,
                                &row_update_rule().monitor_table,
                                &row_update_rule().monitor_sql
                            ).await
                        }
                        3 => {
                            // Check field update rules
                            TestRunMockApi::check_field_update_rules(&field_update_rules()).await
                        }
                        4 => {
                            // Full initial check
                            TestRunMockApi::check_full_initial(&ddl_sql()).await
                        }
                        5 => {
                            // Full initialization preview
                            match TestRunMockApi::preview_full_initialization(&row_update_rule().select_sql).await {
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
                                &selected_datasource_id().unwrap_or_default()
                            ).await
                        }
                        2 => {
                            // Create temporary table
                            let result = TestRunMockApi::create_temp_table(&ddl_sql()).await;
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
                            TestRunMockApi::check_field_consistency(&select_sql(), &ddl_sql()).await
                        }
                        4 => {
                            // Execute single collection
                            TestRunMockApi::execute_single_collection(&select_sql()).await
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


    /// Returns the name of the databse with the given id, or an empty string if not found.
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
        div { class: "container mx-auto p-6 max-w-7xl h-full",
            // Header
            div { class: "flex items-center gap-4 mb-6",
                button {
                    class: "btn btn-soft btn-primary",
                    onclick: move |_| { navigator.push(Route::CollectionPage {}); },
                    "<- 返回"
                }
            }

            // Progress steps
            div { class: "steps steps-horizontal w-full mb-8",
                div {
                    class: if current_step() >= 1 { "step step-primary" } else { "step" },
                    "基础信息"
                }
                div {
                    class: if current_step() >= 2 { "step step-primary" } else { "step" },
                    "选择数据来源、目标资源"
                }
                div {
                    class: if current_step() >= 3 { "step step-primary" } else { "step" },
                    "目标schema定义"
                }
                div {
                    class: if current_step() >= 4 { "step step-primary" } else { "step" },
                    "取数规则定义"
                }
                div {
                    class: if current_step() >= 5 { "step step-primary" } else { "step" },
                    "测试运行"
                }
            }

            // Error message
            if !error_msg().is_empty() {
                div { class: "alert alert-error mb-4",
                    "{error_msg()}"
                }
            }

            // Step 1: Basic Info
            if current_step() == 1 {
                div { class: "card bg-base-200",
                    div { class: "card-body",

                        div { class: "form-control mb-4",
                            div { class: "flex items-center gap-4",
                                label { class: "label mb-2 m-0",
                                    span { class: "label-text font-semibold", "名称" }
                                }
                                input {
                                    r#type: "text",
                                    class: "input input-bordered",
                                    placeholder: "输入任务名称",
                                    value: "{task_name}",
                                    oninput: move |evt| task_name.set(evt.value())
                                }
                            }
                        }

                        div { class: "form-control mb-4",
                            div { class: "flex items-center gap-4", 
                                label { class: "label",
                                    span { class: "label-text font-semibold", "描述" }
                                }
                                textarea {
                                    class: "textarea textarea-bordered",
                                    placeholder: "输入任务描述",
                                    value: "{task_description}",
                                    oninput: move |evt| task_description.set(evt.value())
                                }
                            }
                        }

                        div { class: "form-control mb-4",
                            div { class: "flex items-center gap-4", 
                                label { class: "label",
                                    span { class: "label-text font-semibold", "采集来源" }
                                }
                                select {
                                    class: "select select-bordered",
                                    value: match selected_category() {
                                        Some(CollectionCategory::Database) => "数据库",
                                        Some(CollectionCategory::Api) => "API",
                                        Some(CollectionCategory::Crawler) => "爬虫",
                                        _ => "",
                                    },
                                    onchange: move |evt| {
                                        let category = match evt.value().as_str() {
                                            "数据库" => Some(CollectionCategory::Database),
                                            "API" => Some(CollectionCategory::Api),
                                            "爬虫" => Some(CollectionCategory::Crawler),
                                            _ => None,
                                        };
                                        selected_category.set(category);
                                    },
                                    option { value: "", disabled: true, selected: selected_category().is_none(), "选择采集来源..." }
                                    option { value: "数据库", "数据库" } // 仅先支持数据库采集
                                    // option { value: "API", "API" }
                                    // option { value: "爬虫", "爬虫" } // 先不支持爬虫
                                }
                            }
                        }

                        div { class: "form-control mb-4",
                            div {class: "flex items-center gap-4",
                                label { class: "label",
                                    span { class: "label-text font-semibold", "选择采集模式" }
                                }
                                select {
                                    class: "select select-bordered",
                                    value: match selected_mode() {
                                        Some(CollectType::Full) => "full",
                                        Some(CollectType::Incremental) => "incremental",
                                        None => "",
                                    },
                                    onchange: move |evt| {
                                        let mode = match evt.value().as_str() {
                                            "full" => Some(CollectType::Full),
                                            "incremental" => Some(CollectType::Incremental),
                                            _ => None,
                                        };
                                        selected_mode.set(mode);
                                    },
                                    option { value: "", disabled: true, selected: selected_mode().is_none(), "选择采集模式..." }
                                    option { value: "full", "全量" }
                                    option { value: "incremental", "增量" }
                                }
                            }
                        }

                        div { class: "card-actions justify-end mt-6",
                            button {
                                class: "btn btn-primary",
                                disabled: task_name().is_empty() || selected_mode().is_none(),
                                onclick: move |_| current_step.set(2),
                                "Next →"
                            }
                        }
                    }
                }
            }

            // Step 2: Select Source & Target
            if current_step() == 2 {
                div { class: "card bg-base-200",
                    div { class: "card-body",
                        // 非爬虫形式则需要选定数据源
                        if selected_category() != Some(CollectionCategory::Crawler) {
                            DatasourceSelector {
                                datasources: datasources(),
                                selected_datasource: selected_datasource_id,
                                on_datasource_change: move |id: String| {
                                    selected_datasource_id.set(Some(id.clone()));
                                    selected_datasource_type.set(
                                        datasources()
                                            .iter()
                                            .find(|ds| ds.id == id)
                                            .map(|ds| ds.datasource_type.clone())
                                    );
                                }
                            }
                            div { class: "divider" }
                        }

                        match selected_category() {
                            Some(CollectionCategory::Database) => {
                                if selected_mode() == Some(CollectType::Full) {
                                    rsx! {
                                        ResourceSelectorForDatabaseFull {
                                            resources: resources(),
                                            selected_database_resource: selected_database_resource_id,
                                            on_database_resource_change: move |id: String| {
                                                selected_database_resource_id.set(Some(id.clone()));
                                                selected_database_resource_type.set(
                                                    resources()
                                                        .iter()
                                                        .find(|r| r.id == id)
                                                        .map(|r| r.resource_type.clone())
                                                );
                                            },
                                        }
                                    }
                                } else if selected_mode() == Some(CollectType::Incremental) {
                                    rsx! {
                                        ResourceSelectorForDatabaseIncremental {
                                            resources: resources(),
                                            selected_queue_resource: selected_queue_resource_id,
                                            selected_database_resource: selected_database_resource_id,
                                            on_queue_resource_change: move |id: String| {
                                                selected_queue_resource_id.set(Some(id));
                                            },
                                            on_database_resource_change: move |id: String| {
                                                selected_database_resource_id.set(Some(id.clone()));
                                                selected_database_resource_type.set(
                                                    resources()
                                                        .iter()
                                                        .find(|r| r.id == id)
                                                        .map(|r| r.resource_type.clone())
                                                );
                                            },
                                        }
                                    }
                                } else {
                                    rsx!{}
                                }
                            }
                            _ => rsx!{}
                        }


                        div { class: "card-actions justify-between mt-6",
                            button {
                                class: "btn",
                                onclick: move |_| current_step.set(1),
                                "← Back"
                            }
                            button {
                                class: "btn btn-primary",
                                disabled: selected_datasource_id().is_none() ||
                                    (selected_mode() == Some(CollectType::Incremental) && selected_queue_resource_id().is_none()) ||
                                    (selected_mode() == Some(CollectType::Full) && selected_category() == Some(CollectionCategory::Database) && selected_database_resource_id().is_none()),
                                onclick: move |_| current_step.set(3),
                                "Next →"
                            }
                        }
                    }
                }
            }

            // Step 3: Target Schema Definition
            if current_step() == 3 {
                div { class: "card bg-base-200",
                    div { class: "card-body",
                        TargetSchemaDefinition{
                            collection_category: selected_category().unwrap(),
                            resource_type: selected_database_resource_type().unwrap(),
                            ddl_sql: ddl_sql,
                            json_schema: json_schema,
                        }

                        div { class: "card-actions justify-between mt-6",
                            button {
                                class: "btn",
                                onclick: move |_| current_step.set(2),
                                "← Back"
                            }
                            button {
                                class: "btn btn-primary",
                                disabled: {
                                    if selected_category() == Some(CollectionCategory::Database) {
                                        ddl_sql().is_empty()
                                    } else if selected_category() == Some(CollectionCategory::Api) {
                                        json_schema().is_empty()
                                    } else {
                                        true
                                    }
                                },
                                onclick: move |_| current_step.set(4),
                                "Next →"
                            }
                        }
                    }
                }
            }

            // Step 4: Collection Rule Definition
            if current_step() == 4 {
                div { class: "card bg-base-200",
                    div { class: "card-body",

                        // Show different components based on category and mode
                        if selected_category() == Some(CollectionCategory::Database) && selected_mode() == Some(CollectType::Incremental) {
                            IncrementalRuleDefinition {
                                row_update_rule: row_update_rule,
                                field_update_rules: field_update_rules,
                            }
                        } else {
                            QueryDataRuleDefinition {
                                collection_category: selected_category().unwrap(),
                                datasource_type: selected_datasource_type().unwrap(),
                                query_sql: select_sql,
                            }
                        }

                        div { class: "card-actions justify-between mt-6",
                            button {
                                class: "btn",
                                onclick: move |_| current_step.set(3),
                                "← Back"
                            }
                            button {
                                class: "btn btn-primary",
                                disabled: {
                                    if selected_category() == Some(CollectionCategory::Database) {
                                        if selected_mode() == Some(CollectType::Incremental) {
                                            row_update_rule().select_sql.is_empty()
                                        } else {
                                            select_sql().is_empty()
                                        }
                                    } else if selected_category() == Some(CollectionCategory::Api) {
                                        python_script().is_empty()
                                    } else {
                                        true
                                    }
                                },
                                onclick: move |_| current_step.set(5),
                                "Next →"
                            }
                        }
                    }
                }
            }

            // Step 5: Review & Submit
            if current_step() == 5 {
                div { class: "card bg-base-200",
                    div { class: "card-body",

                        // Only show summary and test button when database category is selected
                        if selected_category() == Some(CollectionCategory::Database) {
                            // Test run section
                            if !test_running() && !test_completed() {
                                div { class: "space-y-4 mb-6",
                                    h3 { class: "font-semibold text-lg", "配置概览" }
                                    div { class: "grid grid-cols-2 gap-4",
                                        div {
                                            p { class: "text-sm text-gray-500", "任务名称" }
                                            p { class: "font-medium", "{task_name()}" }
                                        }
                                        div {
                                            p { class: "text-sm text-gray-500", "采集模式" }
                                            p { class: "font-medium",
                                                {match selected_mode() {
                                                    Some(CollectType::Full) => "数据库全量采集",
                                                    Some(CollectType::Incremental) => "数据库增量采集",
                                                    None => "未选择",
                                                }}
                                            }
                                        }
                                        div {
                                            p { class: "text-sm text-gray-500", "数据源" }
                                            p { class: "font-medium", "{get_datasource_name_by_id(&selected_datasource_id().unwrap_or_default(), &datasources())}" }
                                        }
                                        {if selected_mode() == Some(CollectType::Incremental) {
                                            rsx! {
                                                div {
                                                    p { class: "text-sm text-gray-500", "队列资源" }
                                                    p { class: "font-medium", "{get_queue_name_by_id(&selected_queue_resource_id().unwrap_or_default(), &resources())}" }
                                                }
                                            }
                                        } else {
                                            rsx! {}
                                        }}
                                        div {
                                            p { class: "text-sm text-gray-500", "目标资源" }
                                            p { class: "font-medium", "{get_database_name_by_id(&selected_database_resource_id().unwrap_or_default(), &resources())}" }  
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
                                        span { "点击下方按钮开始测试运行，验证采集配置是否正确" }
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
                                    // 并排显示 执行步骤与日志，Timeline宽度固定320px，日志宽度更大
                                    div { class: "flex flex-row gap-6",
                                        // Timeline (左侧，宽度320px)
                                        div { style: "width: 320px; min-width: 0; flex-shrink: 0;",
                                            TestRunTimeline {
                                                steps: test_steps(),
                                                current_step: current_test_step()
                                            }
                                        }
                                        // Log viewer (右侧，占据剩余空间)
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
                        div { class: "card-actions justify-between mt-6",
                            button {
                                class: "btn",
                                onclick: move |_| current_step.set(4),
                                "← Back"
                            }
                            button {
                                class: "btn btn-primary",
                                disabled: loading() || test_running() || (!test_completed() && selected_category() == Some(CollectionCategory::Database)),
                                onclick: submit_handler,
                                if loading() {
                                    span { class: "loading loading-spinner" }
                                    "Creating..."
                                } else {
                                    "Create Task"
                                }
                            }
                        }
                    }
                }
            }
        }
    }

}


