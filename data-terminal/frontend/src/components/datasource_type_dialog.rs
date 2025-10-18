use dioxus::prelude::*;
use crate::models::datasource::DataSourceType;
use dioxus_free_icons::{icons::hi_outline_icons::*, Icon};

#[component]
pub fn DataSourceTypeDialog(
    show: Signal<bool>,
    on_select: EventHandler<DataSourceType>,
    on_close: EventHandler<()>,
) -> Element {
    if !show() {
        return rsx! {};
    }

    rsx! {
        dialog {
            class: "modal modal-open",
            div {
                class: "modal-box w-11/12 max-w-4xl",
                h3 {
                    class: "text-xl font-bold mb-6",
                    "选择数据源类型"
                }

                div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4 my-6",
                    // MySQL Card - Enabled
                    div {
                        class: "card bg-base-100 shadow hover:shadow-lg transition cursor-pointer",
                        onclick: move |_| {
                            on_select.call(DataSourceType::Mysql);
                        },
                        div { class: "card-body items-center text-center",
                            img {
                                src: asset!("/assets/resource/mysql.svg"),
                                class: "w-16 h-16 mb-4",
                                alt: "MySQL"
                            }
                            h4 { class: "card-title text-lg", "MySQL" }
                            p { class: "text-sm text-base-content/70",
                                "关系型数据库,支持事务和SQL查询"
                            }
                        }
                    }

                    // PostgreSQL Card - Enabled
                    div {
                        class: "card bg-base-100 shadow hover:shadow-lg transition cursor-pointer",
                        onclick: move |_| {
                            on_select.call(DataSourceType::Postgres);
                        },
                        div { class: "card-body items-center text-center",
                            img {
                                src: asset!("/assets/resource/postgres.svg"),
                                class: "w-16 h-16 mb-4",
                                alt: "PostgreSQL"
                            }
                            h4 { class: "card-title text-lg", "PostgreSQL" }
                            p { class: "text-sm text-base-content/70",
                                "高级关系型数据库"
                            }
                        }
                    }

                    // Query API Card - Enabled
                    div {
                        class: "card bg-base-100 shadow hover:shadow-lg transition cursor-pointer",
                        onclick: move |_| {
                            on_select.call(DataSourceType::QueryApi);
                        },
                        div { class: "card-body items-center text-center",
                            Icon {
                                icon: HiDatabase,
                                class: "w-16 h-16 mb-4"
                            }
                            h4 { class: "card-title text-lg", "Query API" }
                            p { class: "text-sm text-base-content/70",
                                "查询型API接口"
                            }
                        }
                    }

                    // Subscribe API Card - Enabled
                    div {
                        class: "card bg-base-100 shadow hover:shadow-lg transition cursor-pointer",
                        onclick: move |_| {
                            on_select.call(DataSourceType::SubscribeApi);
                        },
                        div { class: "card-body items-center text-center",
                            Icon {
                                icon: HiFolder,
                                class: "w-16 h-16 mb-4"
                            }
                            h4 { class: "card-title text-lg", "Subscribe API" }
                            p { class: "text-sm text-base-content/70",
                                "订阅型API接口"
                            }
                        }
                    }
                }

                div {
                    class: "modal-action",
                    button {
                        class: "btn btn-outline",
                        onclick: move |_| {
                            on_close.call(());
                        },
                        "取消"
                    }
                }
            }

            form {
                method: "dialog",
                class: "modal-backdrop",
                onclick: move |_| {
                    on_close.call(());
                },
                button { "close" }
            }
        }
    }
}