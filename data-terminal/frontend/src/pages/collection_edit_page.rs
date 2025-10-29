use dioxus::prelude::*;
use crate::routes::Route;
use crate::models::collection::*;
use crate::models::datasource::DataSource;
use crate::models::resource::Resource;
use crate::components::collection::*;
use crate::utils::collection_api;

/// T054: CollectionEditPage - Edit existing collection tasks
#[component]
pub fn CollectionEditPage(id: String) -> Element {
    let navigator = use_navigator();

    // Clone id for use in multiple closures
    let id_for_load = id.clone();
    let id_for_submit = id.clone();

    // Wizard state
    let mut current_step = use_signal(|| 1);

    // Form state
    let mut task_name = use_signal(String::new);
    let mut task_description = use_signal(String::new);
    let mut selected_mode = use_signal(|| None::<String>);
    let mut selected_category = use_signal(|| CollectionCategory::Database);
    let mut selected_datasource_id = use_signal(|| None::<String>);
    let mut selected_resource_id = use_signal(|| None::<String>);
    let mut selected_tables = use_signal(|| Vec::<String>::new());
    let mut transform_sql = use_signal(String::new);
    let mut target_schema = use_signal(|| None::<TableSchema>);

    // Current task
    let mut current_task = use_signal(|| None::<CollectTask>);

    // Data
    let mut datasources = use_signal(|| Vec::<DataSource>::new());
    let mut resources = use_signal(|| Vec::<Resource>::new());
    let mut tables = use_signal(|| Vec::<TableMetadata>::new());

    let mut loading = use_signal(|| true);
    let mut saving = use_signal(|| false);
    let mut error_msg = use_signal(|| String::new());

    // Load task on mount
    use_effect(move || {
        let task_id = id_for_load.clone();
        spawn(async move {
            loading.set(true);
            match collection_api::fetch_collection_task_by_id(&task_id).await {
                Ok(task) => {
                    // Check if task can be edited
                    if !matches!(task.stage, TaskStage::Draft) {
                        error_msg.set(format!(
                            "Cannot edit task with status '{:?}'. Only Draft or Saved tasks can be edited.",
                            task.stage
                        ));
                        loading.set(false);
                        return;
                    }

                    // Pre-populate form fields
                    task_name.set(task.name.clone());
                    task_description.set(task.description.clone());
                    selected_category.set(task.category.clone());
                    selected_datasource_id.set(Some(task.datasource.id.clone()));
                    selected_resource_id.set(Some(task.resource.id.clone()));

                    // Set mode based on collect_type
                    selected_mode.set(Some(match task.collect_type {
                        CollectType::Full => "full".to_string(),
                        CollectType::Incremental => "incremental".to_string(),
                    }));

                    // Extract rule-specific data
                    match &task.rule {
                        CollectionRule::FullDatabase(rule) => {
                            let table_names: Vec<String> = rule.selected_tables
                                .iter()
                                .map(|t| t.table_name.clone())
                                .collect();
                            selected_tables.set(table_names);
                            transform_sql.set(rule.transformation_sql.clone().unwrap_or_default());
                            target_schema.set(Some(rule.target_schema.clone()));
                        }
                        CollectionRule::IncrementalDatabase(rule) => {
                            selected_tables.set(rule.cdc_config.source_tables.clone());
                            // Incremental mode doesn't have transformation_sql or target_schema in the same way
                        }
                        _ => {
                            // API and Crawler modes not implemented yet
                        }
                    }

                    current_task.set(Some(task));
                    error_msg.set(String::new());

                    // TODO: Fetch datasources and resources from API
                }
                Err(e) => {
                    error_msg.set(format!("Failed to load task: {:?}", e));
                }
            }
            loading.set(false);
        });
    });

    // Generate schema when tables are selected
    let generate_schema_handler = move |_| {
        spawn(async move {
            if let (Some(ds_id), Some(res_id)) = (selected_datasource_id(), selected_resource_id()) {
                let table_selections: Vec<TableSelection> = selected_tables()
                    .iter()
                    .map(|name| TableSelection {
                        table_name: name.clone(),
                        selected_fields: vec![], // Empty = all fields
                    })
                    .collect();

                match collection_api::generate_target_schema(&ds_id, &res_id, table_selections).await {
                    Ok(schema) => {
                        target_schema.set(Some(schema));
                    }
                    Err(e) => {
                        error_msg.set(format!("Failed to generate schema: {:?}", e));
                    }
                }
            }
        });
    };

    // Submit handler
    let submit_handler = move |_| {
        let task_id = id_for_submit.clone();
        spawn(async move {
            saving.set(true);

            // Build the collection rule based on mode
            let rule = if selected_mode() == Some("full".to_string()) {
                let table_selections: Vec<TableSelection> = selected_tables()
                    .iter()
                    .map(|name| TableSelection {
                        table_name: name.clone(),
                        selected_fields: vec![],
                    })
                    .collect();

                CollectionRule::FullDatabase(FullDatabaseRule {
                    selected_tables: table_selections,
                    transformation_sql: if transform_sql().is_empty() {
                        None
                    } else {
                        Some(transform_sql())
                    },
                    target_schema: target_schema().unwrap_or_else(|| TableSchema {
                        table_name: "default".to_string(),
                        fields: vec![],
                    }),
                })
            } else {
                // TODO: Support incremental mode
                return;
            };

            let request = UpdateCollectTaskRequest {
                name: Some(task_name()),
                description: Some(task_description()),
                rule: Some(rule),
            };

            match collection_api::update_collection_task(&task_id, request).await {
                Ok(_task) => {
                    saving.set(false);
                    navigator.push(Route::CollectionPage {});
                }
                Err(e) => {
                    error_msg.set(format!("Failed to update task: {:?}", e));
                    saving.set(false);
                }
            }
        });
    };

    rsx! {
        div { class: "container mx-auto p-6 max-w-4xl",
            // Header
            div { class: "flex items-center gap-4 mb-6",
                button {
                    class: "btn btn-ghost btn-sm",
                    onclick: move |_| { navigator.push(Route::CollectionPage {}); },
                    "← Back"
                }
                h1 { class: "text-3xl font-bold", "Edit Collection Task" }
            }

            // Loading state
            if loading() {
                div { class: "flex justify-center py-12",
                    span { class: "loading loading-spinner loading-lg" }
                }
            } else {
                // Progress steps
                div { class: "steps steps-horizontal w-full mb-8",
                    div {
                        class: if current_step() >= 1 { "step step-primary" } else { "step" },
                        "Basic Info"
                    }
                    div {
                        class: if current_step() >= 2 { "step step-primary" } else { "step" },
                        "Select Source"
                    }
                    div {
                        class: if current_step() >= 3 { "step step-primary" } else { "step" },
                        "Configure"
                    }
                    div {
                        class: if current_step() >= 4 { "step step-primary" } else { "step" },
                        "Review"
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
                            h2 { class: "card-title mb-4", "Step 1: Basic Information" }

                            div { class: "form-control mb-4",
                                label { class: "label",
                                    span { class: "label-text font-semibold", "Task Name" }
                                }
                                input {
                                    r#type: "text",
                                    class: "input input-bordered",
                                    placeholder: "Enter task name...",
                                    value: "{task_name}",
                                    oninput: move |evt| task_name.set(evt.value())
                                }
                            }

                            div { class: "form-control mb-4",
                                label { class: "label",
                                    span { class: "label-text font-semibold", "Description" }
                                }
                                textarea {
                                    class: "textarea textarea-bordered",
                                    placeholder: "Enter task description...",
                                    value: "{task_description}",
                                    oninput: move |evt| task_description.set(evt.value())
                                }
                            }

                            ModeSelector {
                                selected_mode,
                                on_mode_change: move |mode: String| selected_mode.set(Some(mode))
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
                            h2 { class: "card-title mb-4", "Step 2: Select Source & Target" }

                            DatasourceSelector {
                                datasources: datasources(),
                                selected_datasource: selected_datasource_id,
                                on_datasource_change: move |id: String| {
                                    selected_datasource_id.set(Some(id.clone()));
                                    // TODO: Fetch tables for this datasource
                                }
                            }

                            div { class: "divider" }

                            ResourceSelector {
                                resources: resources(),
                                selected_resource: selected_resource_id,
                                on_resource_change: move |id: String| selected_resource_id.set(Some(id))
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

                // Step 3: Configure Collection
                if current_step() == 3 {
                    div { class: "space-y-4",
                        DbConfigPanel {
                            tables: tables(),
                            selected_tables,
                            on_table_toggle: move |table: String| {
                                let mut current = selected_tables();
                                if current.contains(&table) {
                                    current.retain(|t| t != &table);
                                } else {
                                    current.push(table);
                                }
                                selected_tables.set(current);
                            }
                        }

                        div { class: "card bg-base-200",
                            div { class: "card-body",
                                TransformEditor {
                                    transform_sql,
                                    on_sql_change: move |sql: String| transform_sql.set(sql)
                                }

                                button {
                                    class: "btn btn-secondary mt-4",
                                    disabled: selected_tables().is_empty(),
                                    onclick: generate_schema_handler,
                                    "Generate Target Schema"
                                }
                            }
                        }

                        TargetSchemaEditor {
                            schema: target_schema,
                            on_schema_change: move |schema: TableSchema| target_schema.set(Some(schema))
                        }

                        div { class: "flex justify-between",
                            button {
                                class: "btn",
                                onclick: move |_| current_step.set(2),
                                "← Back"
                            }
                            button {
                                class: "btn btn-primary",
                                disabled: selected_tables().is_empty() || target_schema().is_none(),
                                onclick: move |_| current_step.set(4),
                                "Next →"
                            }
                        }
                    }
                }

                // Step 4: Review & Submit
                if current_step() == 4 {
                    div { class: "card bg-base-200",
                        div { class: "card-body",
                            h2 { class: "card-title mb-4", "Step 4: Review & Submit" }

                            div { class: "space-y-4",
                                div {
                                    h3 { class: "font-semibold", "Task Name" }
                                    p { "{task_name()}" }
                                }
                                div {
                                    h3 { class: "font-semibold", "Mode" }
                                    p { "{selected_mode().unwrap_or_default()}" }
                                }
                                div {
                                    h3 { class: "font-semibold", "Selected Tables" }
                                    p { "{selected_tables().join(\", \")}" }
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
                                    disabled: saving(),
                                    onclick: submit_handler,
                                    if saving() {
                                        span { class: "loading loading-spinner" }
                                        "Updating..."
                                    } else {
                                        "Update Task"
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
