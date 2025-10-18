use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use crate::utils::{
    cookie,
    request::{HttpRequest, RequestBuilder},
};
use crate::routes::Route;
use crate::models::datasource::*;
use crate::models::protocol::ApiResponse;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostgresConfig {
    pub name: String,
    pub description: String,
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub catalog: String,
    pub database: String,
}

impl Default for PostgresConfig {
    fn default() -> Self {
        Self {
            name: String::new(),
            description: String::new(),
            host: String::new(),
            port: 5432, // PostgreSQL default port
            username: String::new(),
            password: String::new(),
            catalog: String::new(),
            database: String::new(),
        }
    }
}

// Shared validation function
fn validate_config(cfg: &PostgresConfig) -> Vec<String> {
    let mut errors = Vec::new();

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
    if cfg.catalog.trim().is_empty() {
        errors.push("Catalog不能为空".to_string());
    }
    if cfg.database.trim().is_empty() {
        errors.push("数据库名称不能为空".to_string());
    }

    errors
}

#[component]
pub fn DatasourcePostgresEdit(id: String) -> Element {
    let mut config = use_signal(PostgresConfig::default);
    let mut validation_errors = use_signal(|| Vec::<String>::new());
    let mut is_loading = use_signal(|| true);
    let navigator = use_navigator();

    // Fetch datasource details on mount
    use_effect({
        let datasource_id = id.clone();
        move || {
            let id = datasource_id.clone();
            spawn(async move {
                let client = crate::utils::request::create_client("http://localhost:3000");
                let req_config = RequestBuilder::new()
                    .header("Content-Type", "application/json")
                    .header("Cookie", &cookie::get_browser_cookies())
                    .build();

                let response = client.get(&format!("/api/v1/datasource/{}", id), Some(req_config)).await;
                match response {
                    Ok(response_text) => {
                        match serde_json::from_str::<ApiResponse<DataSource>>(&response_text) {
                            Ok(api_response) => {
                                if api_response.result {
                                    let ds = api_response.data;
                                    // Parse connection_config into PostgresConfig
                                    if let Ok(postgres_config) = serde_json::from_value::<PostgresConfig>(ds.connection_config) {
                                        config.set(postgres_config);
                                    }
                                }
                                is_loading.set(false);
                            }
                            Err(_) => {
                                is_loading.set(false);
                            }
                        }
                    }
                    Err(_) => {
                        is_loading.set(false);
                    }
                }
            });
        }
    });

    let handle_save = {
        let id = id.clone();
        move |_| {
            let errors = validate_config(&config());
            if !errors.is_empty() {
                validation_errors.set(errors);
                return;
            }
            let id = id.clone();
            spawn(async move {
                let client = crate::utils::request::create_client("http://localhost:3000");
                let req_config = RequestBuilder::new()
                            .header("Content-Type", "application/json")
                            .header("Cookie", &cookie::get_browser_cookies())
                            .build();
                let postgres_config = DataSourceCreateUpdate{
                    id: id.clone(),
                    name: config().name,
                    description: config().description,
                    category: DataSourceCategory::Database,
                    datasource_type: DataSourceType::Postgres,
                    connection_config: serde_json::to_value(config()).unwrap(),
                };
                let response = client
                                    .post("/api/v1/datasource/update", Some(req_config), Some(postgres_config))
                                    .await;
                match response {
                    Ok(result) => {
                        match serde_json::from_str::<ApiResponse<String>>(&result) {
                            Ok(result) => {
                                if result.result {
                                    navigator.push(Route::DatasourceOverViewPage{});
                                } else {
                                    let mut errs = errors.clone();
                                    errs.push(result.msg);
                                    validation_errors.set(errs);
                                }
                            },
                            Err(e) => {
                                let mut errs = errors.clone();
                                errs.push(e.to_string());
                                validation_errors.set(errs);
                            }
                        }
                    },
                    Err(e) => {
                        let mut errs = errors.clone();
                        errs.push(e.to_string());
                        validation_errors.set(errs);
                    }
                }
            });
        }
    };

    let handle_test = move |_| {
        let errors = validate_config(&config());
        if !errors.is_empty() {
            validation_errors.set(errors);
            return;
        }
        spawn(async move {
            let client = crate::utils::request::create_client("http://localhost:3000");
            let req_config = RequestBuilder::new()
                        .header("Content-Type", "application/json")
                        .header("Cookie", &cookie::get_browser_cookies())
                        .build();
            let postgres_config = DataSourceCreateUpdate{
                id: String::new(),
                name: config().name,
                description: config().description,
                category: DataSourceCategory::Database,
                datasource_type: DataSourceType::Postgres,
                connection_config: serde_json::to_value(config()).unwrap(),
            };
            let response = client
                                .post("/api/v1/datasource/ping", Some(req_config), Some(postgres_config))
                                .await;
            match response {
                Ok(result) => {
                    match serde_json::from_str::<ApiResponse<String>>(&result) {
                        Ok(result) => {
                            if result.result {
                                validation_errors.set(Vec::new());
                            } else {
                                let mut errs = errors.clone();
                                errs.push(result.msg);
                                validation_errors.set(errs);
                            }
                        },
                        Err(e) => {
                            let mut errs = errors.clone();
                            errs.push(e.to_string());
                            validation_errors.set(errs);
                        }
                    }
                },
                Err(e) => {
                    let mut errs = errors.clone();
                    errs.push(e.to_string());
                    validation_errors.set(errs);
                }
            }
        });
    };

    let is_form_valid = validate_config(&config()).is_empty();

    if is_loading() {
        return rsx! {
            div { class: "w-full max-w-5xl mx-auto p-6 flex justify-center items-center h-64",
                span { class: "loading loading-spinner loading-lg" }
            }
        };
    }

    rsx! {
        div { class: "w-full max-w-5xl mx-auto p-6",

            // Header with breadcrumb
            div { class: "mb-6",
                div { class: "text-sm breadcrumbs",
                    ul {
                        li {
                            Link {
                                to: Route::DatasourceOverViewPage{},
                                "数据源管理"
                            }
                        }
                        li { "编辑PostgreSQL数据源" }
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
                                        config.set(PostgresConfig {
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
                                        config.set(PostgresConfig {
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
                                            config.set(PostgresConfig {
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
                                                config.set(PostgresConfig {
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
                                        placeholder: "postgres",
                                        value: "{config().username}",
                                        oninput: move |e| {
                                            config.set(PostgresConfig {
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
                                            config.set(PostgresConfig {
                                                password: e.value(),
                                                ..config()
                                            });
                                        }
                                    }
                                }
                            }

                            // Catalog Field
                            div { class: "form-control",
                                label { class: "label",
                                    span { class: "label-text font-medium",
                                        "Catalog"
                                        span { class: "text-error ml-1", "*" }
                                    }
                                }
                                input {
                                    class: "input input-bordered w-full",
                                    r#type: "text",
                                    placeholder: "catalog_name",
                                    value: "{config().catalog}",
                                    oninput: move |e| {
                                        config.set(PostgresConfig {
                                            catalog: e.value(),
                                            ..config()
                                        });
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
                                        config.set(PostgresConfig {
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

            // Error Display and Action Buttons
            div {
                class: "flex items-start justify-between mt-6",

                // Validation Errors (left side)
                div { class: "w-1/2 flex-shrink-0",
                    if !validation_errors().is_empty() {
                        ul { class: "list-disc list-inside space-y-1",
                            for error in validation_errors().iter() {
                                li { class: "text-error", "{error}" }
                            }
                        }
                    }
                }

                // Action Buttons (right side)
                div {
                    class: "flex gap-4 flex-shrink-0",
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
        }
    }
}

#[component]
pub fn DatasourcePostgresAdd() -> Element {
    let mut config = use_signal(PostgresConfig::default);
    let mut validation_errors = use_signal(|| Vec::<String>::new());
    let navigator = use_navigator();

    let handle_save = move |_| {
        let errors = validate_config(&config());
        if !errors.is_empty() {
            validation_errors.set(errors);
            return;
        }
        spawn(async move {
            let client = crate::utils::request::create_client("http://localhost:3000");
            let req_config = RequestBuilder::new()
                        .header("Content-Type", "application/json")
                        .header("Cookie", &cookie::get_browser_cookies())
                        .build();
            let postgres_config = DataSourceCreateUpdate{
                id: String::new(),
                name: config().name,
                description: config().description,
                category: DataSourceCategory::Database,
                datasource_type: DataSourceType::Postgres,
                connection_config: serde_json::to_value(config()).unwrap(),
            };
            let response = client
                                .post("/api/v1/datasource/add", Some(req_config), Some(postgres_config))
                                .await;
            match response {
                Ok(result) => {
                    match serde_json::from_str::<ApiResponse<String>>(&result) {
                        Ok(result) => {
                            if result.result {
                                navigator.push(Route::DatasourceOverViewPage{});
                            } else {
                                let mut errs = errors.clone();
                                errs.push(result.msg);
                                validation_errors.set(errs);
                            }
                        },
                        Err(e) => {
                            let mut errs = errors.clone();
                            errs.push(e.to_string());
                            validation_errors.set(errs);
                        }
                    }
                },
                Err(e) => {
                    let mut errs = errors.clone();
                    errs.push(e.to_string());
                    validation_errors.set(errs);
                }
            }
        });
    };

    let handle_test = move |_| {
        let errors = validate_config(&config());
        if !errors.is_empty() {
            validation_errors.set(errors);
            return;
        }
        spawn(async move {
            let client = crate::utils::request::create_client("http://localhost:3000");
            let req_config = RequestBuilder::new()
                        .header("Content-Type", "application/json")
                        .header("Cookie", &cookie::get_browser_cookies())
                        .build();
            let postgres_config = DataSourceCreateUpdate{
                id: String::new(),
                name: config().name,
                description: config().description,
                category: DataSourceCategory::Database,
                datasource_type: DataSourceType::Postgres,
                connection_config: serde_json::to_value(config()).unwrap(),
            };
            let response = client
                                .post("/api/v1/datasource/ping", Some(req_config), Some(postgres_config))
                                .await;
            match response {
                Ok(result) => {
                    match serde_json::from_str::<ApiResponse<String>>(&result) {
                        Ok(result) => {
                            if result.result {
                                validation_errors.set(Vec::new());
                            } else {
                                let mut errs = errors.clone();
                                errs.push(result.msg);
                                validation_errors.set(errs);
                            }
                        },
                        Err(e) => {
                            let mut errs = errors.clone();
                            errs.push(e.to_string());
                            validation_errors.set(errs);
                        }
                    }
                },
                Err(e) => {
                    let mut errs = errors.clone();
                    errs.push(e.to_string());
                    validation_errors.set(errs);
                }
            }
        });
    };

    let is_form_valid = validate_config(&config()).is_empty();

    rsx! {
        div { class: "w-full max-w-5xl mx-auto p-6",

            // Header with breadcrumb
            div { class: "mb-6",
                div { class: "text-sm breadcrumbs",
                    ul {
                        li {
                            Link {
                                to: Route::DatasourceOverViewPage{},
                                "数据源管理"
                            }
                        }
                        li { "添加PostgreSQL数据源" }
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
                                        config.set(PostgresConfig {
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
                                        config.set(PostgresConfig {
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
                                            config.set(PostgresConfig {
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
                                                config.set(PostgresConfig {
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
                                        placeholder: "postgres",
                                        value: "{config().username}",
                                        oninput: move |e| {
                                            config.set(PostgresConfig {
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
                                            config.set(PostgresConfig {
                                                password: e.value(),
                                                ..config()
                                            });
                                        }
                                    }
                                }
                            }

                            // Catalog Field
                            div { class: "form-control",
                                label { class: "label",
                                    span { class: "label-text font-medium",
                                        "Catalog"
                                        span { class: "text-error ml-1", "*" }
                                    }
                                }
                                input {
                                    class: "input input-bordered w-full",
                                    r#type: "text",
                                    placeholder: "catalog_name",
                                    value: "{config().catalog}",
                                    oninput: move |e| {
                                        config.set(PostgresConfig {
                                            catalog: e.value(),
                                            ..config()
                                        });
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
                                        config.set(PostgresConfig {
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

            // Error Display and Action Buttons
            div {
                class: "flex items-start justify-between mt-6",

                // Validation Errors (left side)
                div { class: "w-1/2 flex-shrink-0",
                    if !validation_errors().is_empty() {
                        ul { class: "list-disc list-inside space-y-1",
                            for error in validation_errors().iter() {
                                li { class: "text-error", "{error}" }
                            }
                        }
                    }
                }

                // Action Buttons (right side)
                div {
                    class: "flex gap-4 flex-shrink-0",
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
        }
    }
}
