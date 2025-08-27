use dioxus::prelude::*;
use crate::models::datasource::*;
use dioxus_free_icons::{icons::hi_outline_icons::*, Icon};

#[derive(Props, PartialEq, Clone)]
pub struct DeleteDatasourceDialogProps {
    pub open: bool,
    pub datasource: Option<DataSource>,
    pub on_close: EventHandler<()>,
    pub on_confirm: EventHandler<String>, // pass id to delete
}

#[component]
pub fn DeleteDatasourceDialog(props: DeleteDatasourceDialogProps) -> Element {
    if !props.open { return rsx!{}; }

    let name = props.datasource.as_ref().map(|d| d.name.clone()).unwrap_or_default();
    let delete_id = props.datasource.as_ref().map(|d| d.id.clone());
    rsx! {
        div { class: "modal modal-open",
            div { class: "modal-box max-w-md",
                h3 { class: "font-bold text-lg mb-2", "确认删除" }
                p { class: "text-sm text-base-content/70", "确定要删除数据源 \"{name}\" 吗？该操作不可撤销。" }
                div { class: "modal-action",
                    button { class: "btn", onclick: move |_| props.on_close.call(()), "取消" }
                    if let Some(id) = delete_id {
                        button { class: "btn btn-error", onclick: move |_| props.on_confirm.call(id.clone()),
                            Icon { icon: HiTrash, class: "w-4 h-4 mr-2" }
                            "删除"
                        }
                    }
                }
            }
        }
    }
}


