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
            label { class: "label mb-4",
                span { class: "label-text font-semibold", "选择数据源" }
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
        }
    }
}
