use dioxus::prelude::*;
use crate::models::datasource::DataSource;

/// T045: DatasourceSelector component - Select a datasource for collection
#[component]
pub fn DatasourceSelector(
    datasources: Vec<DataSource>,
    selected_datasource: Signal<Option<String>>,
    on_datasource_change: EventHandler<String>,
) -> Element {
    rsx! {
        div { class: "form-control w-full",
            label { class: "label",
                span { class: "label-text font-semibold", "Source Datasource" }
            }
            select {
                class: "select select-bordered w-full",
                onchange: move |evt| {
                    let value = evt.value();
                    selected_datasource.set(Some(value.clone()));
                    on_datasource_change.call(value);
                },
                option { value: "", "Select a datasource..." }
                for datasource in datasources {
                    option {
                        value: "{datasource.id}",
                        "{datasource.name} ({datasource.datasource_type})"
                    }
                }
            }
            if let Some(ds_id) = selected_datasource() {
                label { class: "label",
                    span { class: "label-text-alt text-info", "Selected: {ds_id}" }
                }
            }
        }
    }
}
