use dioxus::prelude::*;
use crate::models::datasource::*;
use dioxus_free_icons::{icons::hi_outline_icons::*, Icon};

#[derive(Props, PartialEq, Clone)]
pub struct TestDatasourceDialogProps {
    pub open: bool,
    pub datasource: Option<DataSource>,
    pub on_close: EventHandler<()>,
    pub on_test: EventHandler<String>, // pass id to test
}

#[component]
pub fn TestDatasourceDialog(props: TestDatasourceDialogProps) -> Element {
    if !props.open { return rsx!{}; }

    let title = props.datasource.as_ref().map(|d| d.name.clone()).unwrap_or_else(|| "数据源测试".to_string());
    let test_id = props.datasource.as_ref().map(|d| d.id.clone());
    rsx! {
        div { class: "modal modal-open",
            div { class: "modal-box max-w-xl",
                h3 { class: "font-bold text-lg mb-4", "测试连接 - {title}" }
                p { class: "text-sm text-base-content/70", "此操作将尝试连接当前数据源并返回连接状态。" }
                div { class: "modal-action",
                    button { class: "btn", onclick: move |_| props.on_close.call(()), "关闭" }
                    if let Some(id) = test_id {
                        button { class: "btn btn-primary", onclick: move |_| props.on_test.call(id.clone()),
                            Icon { icon: HiLightningBolt, class: "w-4 h-4 mr-2" }
                            "开始测试"
                        }
                    }
                }
            }
        }
    }
}


