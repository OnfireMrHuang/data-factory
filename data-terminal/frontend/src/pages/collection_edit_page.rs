use dioxus::prelude::*;
use crate::routes::Route;
use crate::models::collection::*;
use crate::models::datasource::{DataSource, DataSourceType};
use crate::models::resource::{Resource, ResourceType};
use crate::components::business::collection::{
    TestRunTimeline, TestRunLogViewer, DataPreviewTable
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

/// T054: CollectionEditPage - Edit existing collection tasks
#[component]
pub fn CollectionEditPage(id: String) -> Element {
    let navigator = use_navigator();

    // Tab state
    let mut current_tab = use_signal(|| 1); // 1: Schema, 2: Rules, 3: Test Run

    // Form state
    let mut task_name = use_signal(String::new);
    let mut task_description = use_signal(String::new);
    let mut collection_category = use_signal(|| None::<CollectionCategory>);
    let mut collection_type = use_signal(|| None::<CollectType>);
    let mut datasource_id = use_signal(|| None::<String>);
    let mut datasource_type = use_signal(|| None::<DataSourceType>);
    let mut queue_resource_id = use_signal(|| None::<String>);
    let mut database_resource_id = use_signal(|| None::<String>);
    let mut database_resource_type = use_signal(|| None::<ResourceType>);

    // Schema state
    let mut ddl_sql = use_signal(String::new);
    let mut json_schema = use_signal(String::new);

    // Rule state
    let mut select_sql = use_signal(String::new);
    let mut python_script = use_signal(String::new);
    let mut row_update_rule = use_signal(|| IncrementalRuleForm::default());
    let mut field_update_rules = use_signal(|| Vec::<IncrementalRuleForm>::new());

    // Test run state
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

    let mut loading = use_signal(|| true);
    let mut saving = use_signal(|| false);
    let mut error_msg = use_signal(|| String::new());
    let id_for_submit = id.clone();

    // Load task data on mount
    use_effect(move || {
        let id_clone = id.clone();
        spawn(async move {
            loading.set(true);

            // Fetch task details
            match collections::fetch_collection_task_by_code(&id_clone.clone(), None).await {
                Ok(task) => {
                    // Populate basic info
                    task_name.set(task.name.clone());
                    task_description.set(task.description.clone());
                    collection_category.set(Some(task.category.clone()));
                    collection_type.set(Some(task.collect_type.clone()));
                    datasource_id.set(Some(task.datasource_id.clone()));
                    queue_resource_id.set(Some(task.queue_resource_id.clone()));
                    database_resource_id.set(Some(task.database_resource_id.clone()));

                    // Parse and populate rule data
                    if let Ok(value) = serde_json::from_value::<serde_json::Value>(task.rule.clone()) {
                        // Extract schema
                        if let Some(ddl) = value.get("ddl_sql").and_then(|v| v.as_str()) {
                            ddl_sql.set(ddl.to_string());
                        }
                        if let Some(schema) = value.get("json_schema").and_then(|v| v.as_str()) {
                            json_schema.set(schema.to_string());
                        }

                        // Extract query rules
                        if let Some(sql) = value.get("select_sql").and_then(|v| v.as_str()) {
                            select_sql.set(sql.to_string());
                        }
                        if let Some(script) = value.get("python_script").and_then(|v| v.as_str()) {
                            python_script.set(script.to_string());
                        }

                        // Extract incremental rules
                        if let Some(row_rule) = value.get("row_update_rule") {
                            let rule = IncrementalRuleForm {
                                select_sql: row_rule.get("select_sql").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                                monitor_table: row_rule.get("monitor_table").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                                monitor_sql: row_rule.get("monitor_sql").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                                use_custom_monitor_table: false,
                            };
                            row_update_rule.set(rule);
                        }

                        if let Some(field_rules) = value.get("field_update_rules").and_then(|v| v.as_array()) {
                            let rules: Vec<IncrementalRuleForm> = field_rules
                                .iter()
                                .map(|r| IncrementalRuleForm {
                                    select_sql: r.get("select_sql").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                                    monitor_table: r.get("monitor_table").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                                    monitor_sql: r.get("monitor_sql").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                                    use_custom_monitor_table: false,
                                })
                                .collect();
                            field_update_rules.set(rules);
                        }
                    }
                }
                Err(e) => {
                    error_msg.set(format!("加载任务失败: {:?}", e));
                }
            }

            // Fetch datasources
            match crate::api::datasources::fetch_datasources(1, 1000).await {
                Ok(list) => {
                    datasources.set(list);
                    // Set datasource type
                    if let Some(ds_id) = datasource_id() {
                        if let Some(ds) = datasources().iter().find(|d| d.id == ds_id) {
                            datasource_type.set(Some(ds.datasource_type.clone()));
                        }
                    }
                }
                Err(e) => {
                    error_msg.set(format!("获取数据源失败: {:?}", e));
                }
            }

            // Fetch resources
            match crate::api::resources::fetch_resources().await {
                Ok(list) => {
                    resources.set(list);
                    // Set resource type
                    if let Some(db_id) = database_resource_id() {
                        if let Some(db) = resources().iter().find(|r| r.id == db_id) {
                            database_resource_type.set(Some(db.resource_type.clone()));
                        }
                    }
                }
                Err(e) => {
                    error_msg.set(format!("获取资源失败: {:?}", e));
                }
            }

            loading.set(false);
        });
    });

    // Submit handler
    let submit_handler = move |_| {
        let task_id = id_for_submit.clone();
        spawn(async move {
            saving.set(true);

            // Build the collection rule based on category and mode
            let rule = match collection_category() {
                Some(CollectionCategory::Database) => {
                    if collection_type() == Some(CollectType::Incremental) {
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
                        "mode": match collection_type() {
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
                    saving.set(false);
                    return;
                }
            };

            let request = CreateOrUpdateCollectTaskRequest {
                code: task_id.clone(),
                name: task_name(),
                description: task_description(),
                category: collection_category().unwrap(),
                collect_type: collection_type().unwrap(),
                datasource_id: datasource_id().unwrap(),
                queue_resource_id: queue_resource_id().unwrap(),
                database_resource_id: database_resource_id().unwrap(),
                rule,
            };

            match collections::update_collection_task(request).await {
                Ok(_task) => {
                    saving.set(false);
                    navigator.push(Route::CollectionPage {});
                }
                Err(e) => {
                    error_msg.set(format!("更新任务失败: {:?}", e));
                    saving.set(false);
                }
            }
        });
    };

    // Test run handler (same as create page)
    let test_run_handler = move |_| {
        spawn(async move {
            test_running.set(true);
            test_completed.set(false);
            test_logs.set(Vec::new());
            preview_data.set(None);

            let steps = if collection_type() == Some(CollectType::Incremental) {
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

            let mut logs = test_logs();
            logs.push(LogEntry {
                timestamp: Local::now(),
                level: LogLevel::Info,
                message: "开始执行测试运行...".to_string(),
                details: None,
            });
            test_logs.set(logs);

            for (idx, step) in steps.iter().enumerate() {
                current_test_step.set(idx);

                let mut updated_steps = test_steps();
                updated_steps[idx].status = TestStepStatus::Running;
                updated_steps[idx].start_time = Some(Local::now());
                test_steps.set(updated_steps.clone());

                let mut logs = test_logs();
                logs.push(LogEntry {
                    timestamp: Local::now(),
                    level: LogLevel::Info,
                    message: format!("执行步骤 {}: {}", step.id, step.title),
                    details: None,
                });
                test_logs.set(logs);

                let result = if collection_type() == Some(CollectType::Incremental) {
                    match step.id {
                        1 => TestRunMockApi::check_database_connection(&datasource_id().unwrap_or_default()).await,
                        2 => TestRunMockApi::check_row_update_rule(&row_update_rule().select_sql, &row_update_rule().monitor_table, &row_update_rule().monitor_sql).await,
                        3 => TestRunMockApi::check_field_update_rules(field_update_rules().as_slice()).await,
                        4 => TestRunMockApi::check_full_initial(&ddl_sql()).await,
                        5 => {
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
                        1 => TestRunMockApi::check_database_connection(&datasource_id().unwrap_or_default()).await,
                        2 => {
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
                        3 => TestRunMockApi::check_field_consistency(&select_sql(), &ddl_sql()).await,
                        4 => TestRunMockApi::execute_single_collection(&select_sql()).await,
                        5 => {
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
                        6 => TestRunMockApi::delete_temp_table(&temp_table_name()).await,
                        _ => Ok(crate::api::test_run_mock::TestStepResult {
                            step_id: step.id,
                            success: false,
                            message: "Unknown step".to_string(),
                            error_message: Some("Step not implemented".to_string()),
                            data: None,
                        }),
                    }
                };

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

                        let mut logs = test_logs();
                        logs.push(LogEntry {
                            timestamp: Local::now(),
                            level: if step_result.success { LogLevel::Success } else { LogLevel::Error },
                            message: step_result.message.clone(),
                            details: step_result.data.map(|d| d.to_string()),
                        });
                        test_logs.set(logs);

                        if !step_result.success {
                            test_steps.set(updated_steps);
                            break;
                        }
                    }
                    Err(e) => {
                        updated_steps[idx].status = TestStepStatus::Failed;
                        updated_steps[idx].error_message = Some(e.clone());
                        updated_steps[idx].end_time = Some(Local::now());

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

    // Helper functions to get names
    fn get_datasource_name_by_id(datasource_id: &str, datasources: &Vec<DataSource>) -> String {
        datasources
            .iter()
            .find(|ds| ds.id == datasource_id)
            .map(|ds| ds.name.clone())
            .unwrap_or_else(|| "".to_string())
    }

    fn get_resource_name_by_id(resource_id: &str, resources: &Vec<Resource>) -> String {
        resources
            .iter()
            .find(|r| r.id == resource_id)
            .map(|r| r.name.clone())
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
                h1 { class: "text-2xl font-bold", "编辑采集任务" }
            }

            // Loading state
            if loading() {
                div { class: "flex justify-center py-12",
                    span { class: "loading loading-spinner loading-lg" }
                }
            } else {
                // Error message
                if !error_msg().is_empty() {
                    div { class: "alert alert-error mb-4",
                        "{error_msg()}"
                    }
                }

                // Header Info Section (Read-only)
                div { class: "card bg-base-200 mb-6",
                    div { class: "card-body",
                        // Basic Info (Editable)
                        div { class: "grid grid-cols-2 gap-4 mb-4",
                            div { class: "form-control",
                                label { class: "label",
                                    span { class: "label-text font-semibold", "任务名称" }
                                }
                                input {
                                    r#type: "text",
                                    class: "input input-bordered",
                                    value: "{task_name}",
                                    oninput: move |evt| task_name.set(evt.value())
                                }
                            }
                            div { class: "form-control",
                                label { class: "label",
                                    span { class: "label-text font-semibold", "描述" }
                                }
                                textarea {
                                    class: "textarea textarea-bordered",
                                    value: "{task_description}",
                                    oninput: move |evt| task_description.set(evt.value())
                                }
                            }
                        }

                        div { class: "divider" }

                        // Read-only Info
                        div { class: "grid grid-cols-3 gap-4",
                            div {
                                p { class: "text-sm text-gray-500", "采集来源" }
                                p { class: "font-medium",
                                    {match collection_category() {
                                        Some(CollectionCategory::Database) => "数据库",
                                        Some(CollectionCategory::Api) => "API",
                                        Some(CollectionCategory::Crawler) => "爬虫",
                                        None => "未知",
                                    }}
                                }
                            }
                            div {
                                p { class: "text-sm text-gray-500", "采集模式" }
                                p { class: "font-medium",
                                    {match collection_type() {
                                        Some(CollectType::Full) => "全量",
                                        Some(CollectType::Incremental) => "增量",
                                        None => "未知",
                                    }}
                                }
                            }
                            div {
                                p { class: "text-sm text-gray-500", "数据源" }
                                p { class: "font-medium",
                                    "{get_datasource_name_by_id(&datasource_id().unwrap_or_default(), &datasources())}"
                                }
                            }
                        }

                        div { class: "grid grid-cols-2 gap-4 mt-4",
                            if collection_type() == Some(CollectType::Incremental) {
                                div {
                                    p { class: "text-sm text-gray-500", "队列资源" }
                                    p { class: "font-medium",
                                        "{get_resource_name_by_id(&queue_resource_id().unwrap_or_default(), &resources())}"
                                    }
                                }
                            }
                            div {
                                p { class: "text-sm text-gray-500", "目标资源" }
                                p { class: "font-medium",
                                    "{get_resource_name_by_id(&database_resource_id().unwrap_or_default(), &resources())}"
                                }
                            }
                        }
                    }
                }

                // Tab Navigation
                div { class: "tabs tabs-boxed mb-4",
                    a {
                        class: if current_tab() == 1 { "tab tab-active" } else { "tab" },
                        onclick: move |_| current_tab.set(1),
                        "目标Schema定义"
                    }
                    a {
                        class: if current_tab() == 2 { "tab tab-active" } else { "tab" },
                        onclick: move |_| current_tab.set(2),
                        "取数规则配置"
                    }
                    a {
                        class: if current_tab() == 3 { "tab tab-active" } else { "tab" },
                        onclick: move |_| current_tab.set(3),
                        "测试运行"
                    }
                }

                // Tab Content
                div { class: "card bg-base-200",
                    // Tab 1: Target Schema Definition
                    if current_tab() == 1 {
                        div { class: "card-body",
                            TargetSchemaDefinition {
                                collection_category: collection_category().unwrap_or(CollectionCategory::Database),
                                resource_type: database_resource_type().unwrap_or(ResourceType::Mysql),
                                ddl_sql: ddl_sql,
                                json_schema: json_schema,
                            }
                        }
                    }

                    // Tab 2: Rule Configuration
                    if current_tab() == 2 {
                        if collection_category() == Some(CollectionCategory::Database) && collection_type() == Some(CollectType::Incremental) {
                            IncrementalRuleDefinition {
                                row_update_rule: row_update_rule,
                                field_update_rules: field_update_rules,
                            }
                        } else {
                            QueryDataRuleDefinition {
                                collection_category: collection_category().unwrap_or(CollectionCategory::Database),
                                datasource_type: datasource_type().unwrap_or(DataSourceType::Mysql),
                                query_sql: select_sql,
                            }
                        }
                    }

                    // Tab 3: Test Run
                    if current_tab() == 3 {
                        div { class: "card-body",
                            if !test_running() && !test_completed() {
                                div { class: "space-y-4",
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

                            if test_running() || test_completed() {
                                div { class: "space-y-6",
                                    div { class: "flex flex-row gap-6",
                                        div { style: "width: 320px; min-width: 0; flex-shrink: 0;",
                                            TestRunTimeline {
                                                steps: test_steps(),
                                                current_step: current_test_step()
                                            }
                                        }
                                        div { style: "flex: 1 1 0; min-width: 0;",
                                            TestRunLogViewer {
                                                logs: test_logs(),
                                                max_height: Some("300px".to_string())
                                            }
                                        }
                                    }

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
                }

                // Action Buttons
                div { class: "flex justify-end gap-4 mt-6",
                    button {
                        class: "btn btn-primary",
                        disabled: saving() || test_running(),
                        onclick: submit_handler,
                        if saving() {
                            span { class: "loading loading-spinner" }
                            "保存中..."
                        } else {
                            "保存更改"
                        }
                    }
                }
            }
        }
    }
}
