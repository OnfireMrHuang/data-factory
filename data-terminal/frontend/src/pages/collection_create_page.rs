use dioxus::prelude::*;
use crate::routes::Route;
use crate::models::collection::*;
use crate::models::datasource::{DataSource, DataSourceType};
use crate::models::resource::{Resource, ResourceType};
use crate::components::business::collection::{DatasourceSelector, ResourceSelector};
use crate::api::collections;




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
    let mut selected_resource_id = use_signal(|| None::<String>);
    let mut selected_resource_type = use_signal(|| None::<ResourceType>);

    // Step 3: Target schema definition
    let mut ddl_sql = use_signal(String::new);        // For Database category
    let mut json_schema = use_signal(String::new);    // For API category

    // Step 4: Collection rule definition
    let mut select_sql = use_signal(String::new);     // For Database category
    let mut python_script = use_signal(String::new);  // For API category

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
                    serde_json::json!({
                        "type": "database",
                        "mode": match selected_mode() {
                            Some(CollectType::Full) => "full",
                            Some(CollectType::Incremental) => "incremental",
                            None => "full"
                        },
                        "ddl_sql": ddl_sql(),
                        "select_sql": select_sql()
                    })
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
                resource_id: selected_resource_id().unwrap_or_default(),
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
                    "采集规则定义"
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
                                    option { value: "数据库", "数据库" }
                                    option { value: "API", "API" }
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

                        ResourceSelector {
                            resources: resources(),
                            selected_resource: selected_resource_id,
                            on_resource_change: move |id: String| {
                                selected_resource_id.set(Some(id.clone()));
                                selected_resource_type.set(
                                    resources()
                                        .iter()
                                        .find(|r| r.id == id)
                                        .map(|r| r.resource_type.clone())
                                );
                            }
                        }

                        div { class: "card-actions justify-between mt-6",
                            button {
                                class: "btn",
                                onclick: move |_| current_step.set(1),
                                "← Back"
                            }
                            button {
                                class: "btn btn-primary",
                                disabled: selected_datasource_id().is_none() || selected_resource_id().is_none(),
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
                            resource_type: selected_resource_type().unwrap(),
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
                        h2 { class: "card-title mb-4", "Step 4: 采集规则定义" }

                        // Database category: SELECT SQL textarea
                        if selected_category() == Some(CollectionCategory::Database) {
                            div { class: "form-control mb-4",
                                label { class: "label",
                                    span { class: "label-text font-semibold", "SELECT SQL" }
                                    span { class: "label-text-alt", "从数据源提取数据的SQL查询" }
                                }
                                textarea {
                                    class: "textarea textarea-bordered font-mono h-64",
                                    placeholder: "SELECT id, name, created_at\nFROM source_table\nWHERE status = 'active'\nORDER BY created_at DESC;",
                                    value: "{select_sql}",
                                    oninput: move |evt| select_sql.set(evt.value())
                                }
                            }
                        }

                        // API category: Python script textarea
                        if selected_category() == Some(CollectionCategory::Api) {
                            div { class: "form-control mb-4",
                                label { class: "label",
                                    span { class: "label-text font-semibold", "Python Script" }
                                    span { class: "label-text-alt", "转换源JSON到目标JSON的Python脚本" }
                                }
                                textarea {
                                    class: "textarea textarea-bordered font-mono h-64",
                                    placeholder: "def transform(source_data):\n    # Transform source JSON to target JSON\n    return {{\n        'id': source_data['id'],\n        'name': source_data['name'],\n        'created_at': source_data['timestamp']\n    }}",
                                    value: "{python_script}",
                                    oninput: move |evt| python_script.set(evt.value())
                                }
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
                                        select_sql().is_empty()
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
                        h2 { class: "card-title mb-4", "Step 5: 测试运行 & 提交" }

                        div { class: "space-y-4",
                            div {
                                h3 { class: "font-semibold", "Task Name" }
                                p { "{task_name()}" }
                            }
                            div {
                                h3 { class: "font-semibold", "Description" }
                                p { "{task_description()}" }
                            }
                            div {
                                h3 { class: "font-semibold", "Category" }
                                p {
                                    {match selected_category() {
                                        Some(CollectionCategory::Database) => "Database",
                                        Some(CollectionCategory::Api) => "API",
                                        Some(CollectionCategory::Crawler) => "Crawler",
                                        None => "Not selected",
                                    }}
                                }
                            }
                            div {
                                h3 { class: "font-semibold", "Collection Mode" }
                                p {
                                    {match selected_mode() {
                                        Some(CollectType::Full) => "Full Collection",
                                        Some(CollectType::Incremental) => "Incremental Collection",
                                        None => "Not selected",
                                    }}
                                }
                            }
                            div {
                                h3 { class: "font-semibold", "Datasource ID" }
                                p { "{selected_datasource_id().unwrap_or_default()}" }
                            }
                            div {
                                h3 { class: "font-semibold", "Resource ID" }
                                p { "{selected_resource_id().unwrap_or_default()}" }
                            }

                            // Show Database-specific fields
                            if selected_category() == Some(CollectionCategory::Database) {
                                div {
                                    h3 { class: "font-semibold", "DDL SQL" }
                                    pre { class: "bg-base-300 p-4 rounded overflow-auto max-h-48",
                                        code { "{ddl_sql()}" }
                                    }
                                }
                                div {
                                    h3 { class: "font-semibold", "SELECT SQL" }
                                    pre { class: "bg-base-300 p-4 rounded overflow-auto max-h-48",
                                        code { "{select_sql()}" }
                                    }
                                }
                            }

                            // Show API-specific fields
                            if selected_category() == Some(CollectionCategory::Api) {
                                div {
                                    h3 { class: "font-semibold", "JSON Schema" }
                                    pre { class: "bg-base-300 p-4 rounded overflow-auto max-h-48",
                                        code { "{json_schema()}" }
                                    }
                                }
                                div {
                                    h3 { class: "font-semibold", "Python Script" }
                                    pre { class: "bg-base-300 p-4 rounded overflow-auto max-h-48",
                                        code { "{python_script()}" }
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
                                disabled: loading(),
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


