use dioxus::prelude::*;
use crate::routes::Route;
use crate::models::collection::*;
use crate::components::collection::TaskStageBadge;
use crate::utils::collection_api;
use dioxus_free_icons::{icons::hi_outline_icons::*, Icon};

/// T052: CollectionPage - List all collection tasks
#[component]
pub fn CollectionPage() -> Element {
    let navigator = use_navigator();

    let mut tasks = use_signal(|| Vec::<CollectTask>::new());
    let mut loading = use_signal(|| true);
    let mut error_msg = use_signal(|| String::new());

    // Filters
    let mut status_filter = use_signal(String::new);
    let mut category_filter = use_signal(String::new);

    // Load tasks on mount
    use_effect(move || {
        spawn(async move {
            loading.set(true);
            match collection_api::fetch_collection_tasks().await {
                Ok(data) => {
                    tasks.set(data);
                    error_msg.set(String::new());
                }
                Err(e) => {
                    error_msg.set(format!("Failed to load tasks: {:?}", e));
                }
            }
            loading.set(false);
        });
    });

    // Filtered tasks
    let filtered_tasks = use_memo(move || {
        let mut result = tasks().clone();

        if !status_filter().is_empty() {
            result.retain(|t| format!("{:?}", t.stage).to_lowercase() == status_filter().to_lowercase());
        }

        if !category_filter().is_empty() {
            result.retain(|t| format!("{:?}", t.category).to_lowercase() == category_filter().to_lowercase());
        }

        result
    });

    rsx! {
        div { class: "container mx-auto p-6",
            // Header
            div { class: "flex justify-between items-center mb-6",
                h1 { class: "text-3xl font-bold", "Collection Tasks" }
                button {
                    class: "btn btn-primary gap-2",
                    onclick: move |_| {
                        navigator.push(Route::CollectionCreatePage {});
                    },
                    Icon { icon: HiPlusCircle, class: "w-5 h-5" }
                    "Create Collection Task"
                }
            }

            // Filters
            div { class: "card bg-base-200 mb-4",
                div { class: "card-body",
                    div { class: "flex gap-4",
                        // Status filter
                        select {
                            class: "select select-bordered",
                            value: "{status_filter}",
                            onchange: move |evt| status_filter.set(evt.value()),
                            option { value: "", "All Statuses" }
                            option { value: "draft", "Draft" }
                            option { value: "saved", "Saved" }
                            option { value: "applied", "Applied" }
                            option { value: "running", "Running" }
                            option { value: "failed", "Failed" }
                        }

                        // Category filter
                        select {
                            class: "select select-bordered",
                            value: "{category_filter}",
                            onchange: move |evt| category_filter.set(evt.value()),
                            option { value: "", "All Categories" }
                            option { value: "database", "Database" }
                            option { value: "api", "API" }
                            option { value: "crawler", "Crawler" }
                        }
                    }
                }
            }

            // Error message
            if !error_msg().is_empty() {
                div { class: "alert alert-error mb-4",
                    "{error_msg()}"
                }
            }

            // Loading state
            if loading() {
                div { class: "flex justify-center py-12",
                    span { class: "loading loading-spinner loading-lg" }
                }
            } else if filtered_tasks().is_empty() {
                div { class: "alert alert-info",
                    "No collection tasks found. Click 'Create Collection Task' to get started."
                }
            } else {
                // Tasks table
                div { class: "overflow-x-auto",
                    table { class: "table table-zebra",
                        thead {
                            tr {
                                th { "Name" }
                                th { "Category" }
                                th { "Type" }
                                th { "Status" }
                                th { "Datasource" }
                                th { "Resource" }
                                th { "Created" }
                                th { "Actions" }
                            }
                        }
                        tbody {
                            for task in filtered_tasks() {
                                tr {
                                    key: "{task.id}",
                                    td {
                                        div { class: "font-semibold", "{task.name}" }
                                        div { class: "text-xs opacity-70", "{task.description}" }
                                    }
                                    td {
                                        span { class: "badge badge-outline",
                                            "{task.category:?}"
                                        }
                                    }
                                    td { "{task.collect_type:?}" }
                                    td { TaskStageBadge { stage: task.stage.clone() } }
                                    td {
                                        div { class: "text-sm", "{task.datasource.name}" }
                                        div { class: "text-xs opacity-60", "{task.datasource.datasource_type}" }
                                    }
                                    td {
                                        div { class: "text-sm", "{task.resource.name}" }
                                        div { class: "text-xs opacity-60", "{task.resource.resource_type}" }
                                    }
                                    td {
                                        "{task.created_at.format(\"%Y-%m-%d\")}"
                                    }
                                    td {
                                        div { class: "flex gap-2",
                                            {
                                                let edit_id = task.id.clone();
                                                rsx! {
                                                    button {
                                                        class: "btn btn-sm btn-ghost",
                                                        onclick: move |_| {
                                                            navigator.push(Route::CollectionEditPage { id: edit_id.clone() });
                                                        },
                                                        Icon { icon: HiPencil, class: "w-4 h-4" }
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
}
