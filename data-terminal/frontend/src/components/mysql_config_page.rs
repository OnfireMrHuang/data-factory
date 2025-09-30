use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MysqlConfig {
    pub name: String,
    pub description: String,
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
}

impl Default for MysqlConfig {
    fn default() -> Self {
        Self {
            name: String::new(),
            description: String::new(),
            host: String::new(),
            port: 3306,
            username: String::new(),
            password: String::new(),
            database: String::new(),
        }
    }
}

#[component]
pub fn MysqlConfigPage(
    show: Signal<bool>,
    on_save: EventHandler<MysqlConfig>,
    on_cancel: EventHandler<()>,
    on_test_connection: EventHandler<MysqlConfig>,
) -> Element {
    let mut config = use_signal(MysqlConfig::default);
    let mut validation_errors = use_signal(|| Vec::<String>::new());

    // Validation function
    let validate_form = move || -> Vec<String> {
        let mut errors = Vec::new();
        let cfg = config();

        if cfg.name.trim().is_empty() {
            errors.push("数据源名称不能为空".to_string());
        }
        if cfg.host.trim().is_empty() {
            errors.push("主机地址不能为空".to_string());
        }
        if cfg.port == 0 || cfg.port > 65535 {
            errors.push("端口必须在1-65535之间".to_string());
        }
        if cfg.username.trim().is_empty() {
            errors.push("用户名不能为空".to_string());
        }
        if cfg.password.trim().is_empty() {
            errors.push("密码不能为空".to_string());
        }
        if cfg.database.trim().is_empty() {
            errors.push("数据库名称不能为空".to_string());
        }

        errors
    };

    let handle_save = move |_| {
        let errors = validate_form();
        if errors.is_empty() {
            on_save.call(config());
            config.set(MysqlConfig::default());
            validation_errors.set(Vec::new());
        } else {
            validation_errors.set(errors);
        }
    };

    let handle_test = move |_| {
        let errors = validate_form();
        if errors.is_empty() {
            on_test_connection.call(config());
            validation_errors.set(Vec::new());
        } else {
            validation_errors.set(errors);
        }
    };

    let handle_cancel = move |_| {
        config.set(MysqlConfig::default());
        validation_errors.set(Vec::new());
        on_cancel.call(());
    };

    if !show() {
        return rsx! {};
    }

    let is_form_valid = validate_form().is_empty();

    rsx! {
        dialog {
            class: "modal modal-open",
            div {
                class: "modal-box w-11/12 max-w-5xl max-h-[90vh] overflow-y-auto",

                // Header with breadcrumb
                div { class: "mb-6",
                    div { class: "text-sm breadcrumbs",
                        ul {
                            li { a { "数据源管理" } }
                            li { a { "添加数据源" } }
                            li { "MySQL配置" }
                        }
                    }
                    h3 { class: "text-2xl font-bold mt-2", "MySQL数据源配置" }
                }

                // Validation Errors Display
                if !validation_errors().is_empty() {
                    div { class: "alert alert-error mb-4",
                        ul {
                            for error in validation_errors().iter() {
                                li { "{error}" }
                            }
                        }
                    }
                }

                // Form Content
                div { class: "space-y-6",

                    // Section 1: Basic Information
                    div { class: "card bg-base-100 shadow-sm",
                        div { class: "card-body",
                            h4 { class: "text-lg font-semibold mb-4", "基本信息" }
                            div { class: "space-y-4",
                                // Name Field
                                div { class: "form-control",
                                    label { class: "label",
                                        span { class: "label-text font-medium",
                                            "数据源名称"
                                            span { class: "text-error ml-1", "*" }
                                        }
                                    }
                                    input {
                                        class: "input input-bordered w-full",
                                        r#type: "text",
                                        placeholder: "请输入数据源名称",
                                        value: "{config().name}",
                                        oninput: move |e| {
                                            config.set(MysqlConfig {
                                                name: e.value(),
                                                ..config()
                                            });
                                        }
                                    }
                                }

                                // Description Field
                                div { class: "form-control",
                                    label { class: "label",
                                        span { class: "label-text font-medium", "数据源描述" }
                                    }
                                    textarea {
                                        class: "textarea textarea-bordered w-full h-24",
                                        placeholder: "请输入数据源描述",
                                        value: "{config().description}",
                                        oninput: move |e| {
                                            config.set(MysqlConfig {
                                                description: e.value(),
                                                ..config()
                                            });
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Section 2: Connection Configuration
                    div { class: "card bg-base-100 shadow-sm",
                        div { class: "card-body",
                            h4 { class: "text-lg font-semibold mb-4", "连接配置" }
                            div { class: "space-y-4",
                                // Host and Port
                                div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                                    div { class: "form-control",
                                        label { class: "label",
                                            span { class: "label-text font-medium",
                                                "主机地址"
                                                span { class: "text-error ml-1", "*" }
                                            }
                                        }
                                        input {
                                            class: "input input-bordered w-full",
                                            r#type: "text",
                                            placeholder: "192.168.1.100",
                                            value: "{config().host}",
                                            oninput: move |e| {
                                                config.set(MysqlConfig {
                                                    host: e.value(),
                                                    ..config()
                                                });
                                            }
                                        }
                                    }

                                    div { class: "form-control",
                                        label { class: "label",
                                            span { class: "label-text font-medium",
                                                "端口"
                                                span { class: "text-error ml-1", "*" }
                                            }
                                        }
                                        input {
                                            class: "input input-bordered w-full",
                                            r#type: "number",
                                            value: "{config().port}",
                                            oninput: move |e| {
                                                if let Ok(port) = e.value().parse::<u16>() {
                                                    config.set(MysqlConfig {
                                                        port,
                                                        ..config()
                                                    });
                                                }
                                            }
                                        }
                                    }
                                }

                                // Username and Password
                                div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                                    div { class: "form-control",
                                        label { class: "label",
                                            span { class: "label-text font-medium",
                                                "用户名"
                                                span { class: "text-error ml-1", "*" }
                                            }
                                        }
                                        input {
                                            class: "input input-bordered w-full",
                                            r#type: "text",
                                            placeholder: "root",
                                            value: "{config().username}",
                                            oninput: move |e| {
                                                config.set(MysqlConfig {
                                                    username: e.value(),
                                                    ..config()
                                                });
                                            }
                                        }
                                    }

                                    div { class: "form-control",
                                        label { class: "label",
                                            span { class: "label-text font-medium",
                                                "密码"
                                                span { class: "text-error ml-1", "*" }
                                            }
                                        }
                                        input {
                                            class: "input input-bordered w-full",
                                            r#type: "password",
                                            value: "{config().password}",
                                            oninput: move |e| {
                                                config.set(MysqlConfig {
                                                    password: e.value(),
                                                    ..config()
                                                });
                                            }
                                        }
                                    }
                                }

                                // Database Name
                                div { class: "form-control",
                                    label { class: "label",
                                        span { class: "label-text font-medium",
                                            "数据库名称"
                                            span { class: "text-error ml-1", "*" }
                                        }
                                    }
                                    input {
                                        class: "input input-bordered w-full",
                                        r#type: "text",
                                        placeholder: "database_name",
                                        value: "{config().database}",
                                        oninput: move |e| {
                                            config.set(MysqlConfig {
                                                database: e.value(),
                                                ..config()
                                            });
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // Action Buttons
                div {
                    class: "modal-action",
                    button {
                        class: "btn btn-outline",
                        onclick: handle_cancel,
                        "取消"
                    }
                    button {
                        class: "btn btn-info",
                        disabled: !is_form_valid,
                        onclick: handle_test,
                        "测试连接"
                    }
                    button {
                        class: "btn btn-primary",
                        disabled: !is_form_valid,
                        onclick: handle_save,
                        "保存"
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