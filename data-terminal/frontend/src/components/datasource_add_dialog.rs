use dioxus::prelude::*;
use crate::models::datasource::*;
use dioxus_free_icons::{icons::hi_outline_icons::*, Icon};

#[derive(Props, PartialEq, Clone)]
pub struct AddDatasourceDialogProps {
    pub open: bool,
    pub on_close: EventHandler<()>,
    pub on_submit: EventHandler<DataSourceFormData>,
}

#[component]
pub fn AddDatasourceDialog(props: AddDatasourceDialogProps) -> Element {
    if !props.open { return rsx!{}; }

    let mut name = use_signal(String::new);
    let mut description = use_signal(String::new);
    let mut category = use_signal(|| DataSourceCategory::Database);
    let mut ds_type = use_signal(|| DataSourceType::Mysql);
    let mut conn = use_signal(|| serde_json::json!({"host":"","port":0,"username":"","database":""}));

    rsx! {
        div { class: "modal modal-open",
            div { class: "modal-box max-w-2xl",
                h3 { class: "font-bold text-lg mb-4", "添加数据源" }

                div { class: "form-control gap-3",
                    input { class: "input input-bordered w-full", placeholder: "名称", value: "{name}", oninput: move |e| name.set(e.value().to_string()) }
                    textarea { class: "textarea textarea-bordered w-full", placeholder: "描述", value: "{description}", oninput: move |e| description.set(e.value().to_string()) }
                    select { class: "select select-bordered w-full", value: "{category}", onchange: move |e| {
                            let val = e.value();
                            category.set(if val == "Api" { DataSourceCategory::Api } else { DataSourceCategory::Database });
                        },
                        option { value: "Database", "数据库" }
                        option { value: "Api", "API" }
                    }
                    select { class: "select select-bordered w-full", value: "{ds_type}", onchange: move |e| {
                            ds_type.set(match e.value().as_str() {
                                "Postgres" => DataSourceType::Postgres,
                                "QueryApi" => DataSourceType::QueryApi,
                                "SubscribeApi" => DataSourceType::SubscribeApi,
                                _ => DataSourceType::Mysql,
                            });
                        },
                        option { value: "Mysql", "MySQL" }
                        option { value: "Postgres", "PostgreSQL" }
                        option { value: "QueryApi", "查询API" }
                        option { value: "SubscribeApi", "订阅API" }
                    }
                }

                div { class: "modal-action",
                    button { class: "btn", onclick: move |_| props.on_close.call(()), "取消" }
                    button { class: "btn btn-primary", onclick: move |_| {
                            let payload = DataSourceFormData { 
                                name: name(), 
                                description: description(), 
                                category: category(),
                                datasource_type: ds_type(),
                                connection_config: conn(),
                            };
                            props.on_submit.call(payload);
                        }, 
                        Icon { icon: HiCheck, class: "w-4 h-4 mr-2" }
                        "保存"
                    }
                }
            }
        }
    }
}


