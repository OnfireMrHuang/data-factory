use dioxus::prelude::*;
use crate::models::resource::Resource;
use crate::models::resource::Category;

/// T046: ResourceSelector component - Select a target resource
#[component]
pub fn ResourceSelectorForDatabaseIncremental(
    resources: Vec<Resource>,
    selected_queue_resource: Signal<Option<String>>,
    selected_database_resource: Signal<Option<String>>,
    on_queue_resource_change: EventHandler<String>,
    on_database_resource_change: EventHandler<String>,
) -> Element {
    rsx! {
        div { class: "form-control w-full",
            label { class: "label mb-4",
                span { class: "label-text font-semibold", "选择消息队列" }
            }
            select {
                class: "select select-bordered w-full",
                onchange: move |evt| {
                    let value = evt.value();
                    selected_queue_resource.set(Some(value.clone()));
                    on_queue_resource_change.call(value);
                },
                option { value: "", "Select a queue resource..." }
                for resource in resources.iter().filter(|r| r.category == Category::Queue) {
                    option {
                        value: "{resource.id}",
                        "{resource.name} ({resource.resource_type:?})"
                    }
                }
            }

            div { class: "divider" }

            label { class: "label mb-4",
                span { class: "label-text font-semibold", "选择目标数据库" }
            }
            select {
                class: "select select-bordered w-full",
                onchange: move |evt| {
                    let value = evt.value();
                    selected_database_resource.set(Some(value.clone()));
                    on_database_resource_change.call(value);
                },
                option { value: "", "Select a target database resource..." }
                for resource in resources.iter().filter(|r| r.category == Category::RelationalDatabase) {
                    option {
                        value: "{resource.id}",
                        "{resource.name} ({resource.resource_type:?})"
                    }
                }
            }
        }
    }
}



#[component]
pub fn ResourceSelectorForDatabaseFull(
    resources: Vec<Resource>,
    selected_database_resource: Signal<Option<String>>,
    on_database_resource_change: EventHandler<String>,
) -> Element {
    rsx! {
        div { class: "form-control w-full",
            label { class: "label mb-4",
                span { class: "label-text font-semibold", "选择目标数据库" }
            }
            select {
                class: "select select-bordered w-full",
                onchange: move |evt| {
                    let value = evt.value();
                    selected_database_resource.set(Some(value.clone()));
                    on_database_resource_change.call(value);
                },
                option { value: "", "Select a target database resource..." }
                for resource in resources.iter().filter(|r| r.category == Category::RelationalDatabase) {
                    option {
                        value: "{resource.id}",
                        "{resource.name} ({resource.resource_type:?})"
                    }
                }
            }
        }
    }
}