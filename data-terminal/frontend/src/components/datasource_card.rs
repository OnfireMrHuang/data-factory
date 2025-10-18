use dioxus::prelude::*;
use crate::models::datasource::*;
use dioxus_free_icons::{icons::hi_outline_icons::*, Icon};
use std::sync::Arc;

// 获取数据源图标
fn get_datasource_icon(datasource_type: &DataSourceType) -> Element {
    match datasource_type {
        DataSourceType::Mysql => rsx! {
            img { 
                src: asset!("/assets/resource/mysql.svg"), 
                class: "w-16 h-16",
                alt: "MySQL"
            }
        },
        DataSourceType::Postgres => rsx! {
            img { 
                src: asset!("/assets/resource/postgres.svg"), 
                class: "w-16 h-16",
                alt: "PostgreSQL"
            }
        },
        DataSourceType::QueryApi => rsx! {
            img { 
                src: asset!("/assets/datasource/query-api.svg"), 
                class: "w-16 h-16",
                alt: "Query API"
            }
        },
        DataSourceType::SubscribeApi => rsx! {
            img { 
                src: asset!("/assets/datasource/subscribe-api.svg"), 
                class: "w-16 h-16",
                alt: "Subscribe API"
            }
        },
    }
}

// 获取连接状态样式
fn get_connection_status_style(status: &ConnectionStatus) -> &'static str {
    match status {
        ConnectionStatus::Connected => "badge badge-success gap-2",
        ConnectionStatus::Disconnected => "badge badge-warning gap-2",
        ConnectionStatus::Error => "badge badge-error gap-2",
    }
}

// 获取连接状态文本
fn get_connection_status_text(status: &ConnectionStatus) -> &'static str {
    match status {
        ConnectionStatus::Connected => "已连接",
        ConnectionStatus::Disconnected => "未连接",
        ConnectionStatus::Error => "连接错误",
    }
}

// 获取连接状态图标
fn get_connection_status_icon(status: &ConnectionStatus) -> Element {
    match status {
        ConnectionStatus::Connected => rsx! {
            Icon { 
                icon: HiCheckCircle,
                class: "w-4 h-4"
            }
        },
        ConnectionStatus::Disconnected => rsx! {
            Icon { 
                icon: HiExclamation,
                class: "w-4 h-4"
            }
        },
        ConnectionStatus::Error => rsx! {
            Icon { 
                icon: HiXCircle,
                class: "w-4 h-4"
            }
        },
    }
}

#[component]
pub fn DatasourceCard(
    datasource: DataSource,
    on_edit: EventHandler<(String, DataSourceType)>,
    on_delete: EventHandler<String>,
) -> Element {
    let id = Arc::new(datasource.id.clone());
    let ds_type = datasource.datasource_type.clone();
    let mut show_menu = use_signal(|| false);
    
    rsx! {
        div { key: "{datasource.id}", class: "card bg-base-100 shadow-md hover:shadow-lg transition-shadow",
            div { class: "card-body",
                // 卡片头部
                div { class: "flex items-start justify-between",
                    div { class: "flex items-center space-x-3",
                        {get_datasource_icon(&datasource.datasource_type)}
                        div {
                            h3 { class: "font-semibold text-lg", "{datasource.name}" }
                            p { class: "text-sm text-gray-500", "{datasource.description}" }
                        }
                    }
                    div { class: "relative",
                        button { 
                            class: "btn btn-ghost btn-sm",
                            onclick: move |_| show_menu.set(!show_menu()),
                            Icon { icon: HiDotsVertical, class: "w-4 h-4" }
                        }
                        {if show_menu() {
                            rsx! {
                                ul { class: "absolute right-0 top-full mt-1 menu p-2 shadow bg-base-100 rounded-box w-52 z-10",
                                    li {
                                        button {
                                            class: "btn btn-ghost btn-sm w-full justify-start",
                                            onclick: {
                                                let id = Arc::clone(&id);
                                                let ds_type = ds_type.clone();
                                                move |_| on_edit.call(((*id).clone(), ds_type.clone()))
                                            },
                                            Icon { icon: HiPencil, class: "w-4 h-4 mr-2" }
                                            "编辑"
                                        }
                                    }
                                    li {
                                        button {
                                            class: "btn btn-ghost btn-sm w-full justify-start text-error",
                                            onclick: {
                                                let id = Arc::clone(&id);
                                                move |_| on_delete.call((*id).clone())
                                            },
                                            Icon { icon: HiTrash, class: "w-4 h-4 mr-2" }
                                            "删除"
                                        }
                                    }
                                }
                            }
                        } else {
                            rsx! {}
                        }}
                    }
                }
                
                // 连接状态
                div { class: "mt-4",
                    div { class: "{get_connection_status_style(&datasource.connection_status)}",
                        {get_connection_status_icon(&datasource.connection_status)}
                        "{get_connection_status_text(&datasource.connection_status)}"
                    }
                }
                
                // 创建时间
                div { class: "mt-2 text-xs text-gray-400",
                    "创建时间: {datasource.created_at}"
                }
            }
        }
    }
}
