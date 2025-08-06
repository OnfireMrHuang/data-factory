use dioxus::prelude::*;
use crate::models::protocol::Resource;

#[component]
pub fn ResourceDeleteDialog(
    resource: Resource,
    on_confirm: EventHandler<Resource>,
    on_cancel: EventHandler<()>,
) -> Element {
    let handle_confirm = {
        let on_confirm = on_confirm.clone();
        let resource = resource.clone();
        move |_| {
            on_confirm.call(resource.clone());
        }
    };

    let handle_cancel = {
        let on_cancel = on_cancel.clone();
        move |_| {
            on_cancel.call(());
        }
    };

    rsx! {
        dialog {
            class: "modal modal-open",
            div {
                class: "modal-box",
                h3 {
                    class: "text-lg font-bold mb-4",
                    "删除确认"
                }

                p {
                    class: "py-4 text-base-content/80",
                    "请确认是否删除资源："
                }

                div {
                    class: "bg-base-200 p-3 rounded-lg mb-4",
                    div { class: "font-medium", "{resource.name}" }
                    div { class: "text-sm text-base-content/60", "{resource.description}" }
                }

                div {
                    class: "modal-action",
                    button {
                        class: "btn btn-outline",
                        onclick: handle_cancel,
                        "取消"
                    }

                    button {
                        class: "btn btn-error",
                        onclick: handle_confirm,
                        "确认删除"
                    }
                }
            }

            form {
                method: "dialog",
                class: "modal-backdrop",
                onclick: handle_cancel,
                button { "close" }
            }
        }
    }
} 