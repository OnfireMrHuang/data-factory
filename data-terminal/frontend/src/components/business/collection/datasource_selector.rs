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
                // 设置当前选中的数据源
                value: selected_datasource().clone().unwrap_or_default(),
                onchange: move |evt| {
                    let value = evt.value();
                    selected_datasource.set(Some(value.clone()));
                    on_datasource_change.call(value);
                },
                option { 
                    value: "", 
                    selected: selected_datasource().is_none(),
                    "Select a datasource..." 
                }
                for datasource in datasources.iter() {
                    option {
                        value: "{datasource.id}",
                        selected: selected_datasource()
                            .as_ref()
                            .map(|id| id == &datasource.id)
                            .unwrap_or(false),
                        "{datasource.name} ({datasource.datasource_type})"
                    }
                }
            }
        }
    }
}
