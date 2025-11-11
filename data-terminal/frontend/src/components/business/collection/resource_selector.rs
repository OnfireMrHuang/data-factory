use dioxus::prelude::*;
use crate::models::resource::Resource;

/// T046: ResourceSelector component - Select a target resource
#[component]
pub fn ResourceSelector(
    resources: Vec<Resource>,
    selected_resource: Signal<Option<String>>,
    on_resource_change: EventHandler<String>,
) -> Element {
    rsx! {
        div { class: "form-control w-full",
            label { class: "label",
                span { class: "label-text font-semibold", "Target Resource" }
            }
            select {
                class: "select select-bordered w-full",
                onchange: move |evt| {
                    let value = evt.value();
                    selected_resource.set(Some(value.clone()));
                    on_resource_change.call(value);
                },
                option { value: "", "Select a target resource..." }
                for resource in resources {
                    option {
                        value: "{resource.id}",
                        "{resource.name} ({resource.resource_type:?})"
                    }
                }
            }
            if let Some(res_id) = selected_resource() {
                label { class: "label",
                    span { class: "label-text-alt text-success", "Selected: {res_id}" }
                }
            }
        }
    }
}
