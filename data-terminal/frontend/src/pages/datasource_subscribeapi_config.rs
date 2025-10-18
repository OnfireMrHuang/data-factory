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
pub struct SubscribeApiConfig {
    pub name: String,
    pub description: String,
    pub base_url: String,
    pub api_key: String,
    pub webhook_url: String,
    pub timeout_seconds: u32,
    // Token application fields
    pub token_name: String,
    pub token_expiry_days: u32,
    pub token_scopes: Vec<String>,
}

impl Default for SubscribeApiConfig {
    fn default() -> Self {
        Self {
            name: String::new(),
            description: String::new(),
            base_url: String::new(),
            api_key: String::new(),
            webhook_url: String::new(),
            timeout_seconds: 30,
            token_name: String::new(),
            token_expiry_days: 30,
            token_scopes: Vec::new(),
        }
    }
}

// Shared validation function
fn validate_config(cfg: &SubscribeApiConfig) -> Vec<String> {
    let mut errors = Vec::new();

    if cfg.name.trim().is_empty() {
        errors.push("数据源名称不能为空".to_string());
    }
    if cfg.base_url.trim().is_empty() {
        errors.push("API基础URL不能为空".to_string());
    }
    if !cfg.base_url.starts_with("http://") && !cfg.base_url.starts_with("https://") {
        errors.push("API基础URL必须以http://或https://开头".to_string());
    }
    if !cfg.webhook_url.trim().is_empty() && !cfg.webhook_url.starts_with("http://") && !cfg.webhook_url.starts_with("https://") {
        errors.push("Webhook URL必须以http://或https://开头".to_string());
    }
    if cfg.timeout_seconds == 0 || cfg.timeout_seconds > 300 {
        errors.push("超时时间必须在1-300秒之间".to_string());
    }

    errors
}

#[component]
pub fn DatasourceSubscribeApiEdit(id: String) -> Element {
    let mut config = use_signal(SubscribeApiConfig::default);
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
                                    // Parse connection_config into SubscribeApiConfig
                                    if let Ok(subscribeapi_config) = serde_json::from_value::<SubscribeApiConfig>(ds.connection_config) {
                                        config.set(subscribeapi_config);
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
                let subscribeapi_config = DataSourceCreateUpdate{
                    id: id.clone(),
                    name: config().name,
                    description: config().description,
                    category: DataSourceCategory::Api,
                    datasource_type: DataSourceType::SubscribeApi,
                    connection_config: serde_json::to_value(config()).unwrap(),
                };
                let response = client
                                    .post("/api/v1/datasource/update", Some(req_config), Some(subscribeapi_config))
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
            let subscribeapi_config = DataSourceCreateUpdate{
                id: String::new(),
                name: config().name,
                description: config().description,
                category: DataSourceCategory::Api,
                datasource_type: DataSourceType::SubscribeApi,
                connection_config: serde_json::to_value(config()).unwrap(),
            };
            let response = client
                                .post("/api/v1/datasource/ping", Some(req_config), Some(subscribeapi_config))
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
                        li { "编辑Subscribe API数据源" }
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
                                        config.set(SubscribeApiConfig {
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
                                        config.set(SubscribeApiConfig {
                                            description: e.value(),
                                            ..config()
                                        });
                                    }
                                }
                            }
                        }
                    }
                }

                // Section 2: API Configuration
                div { class: "card bg-base-100 shadow-sm",
                    div { class: "card-body",
                        h4 { class: "text-lg font-semibold mb-4", "API配置" }
                        div { class: "space-y-4",
                            // Base URL
                            div { class: "form-control",
                                label { class: "label",
                                    span { class: "label-text font-medium",
                                        "API基础URL"
                                        span { class: "text-error ml-1", "*" }
                                    }
                                }
                                input {
                                    class: "input input-bordered w-full",
                                    r#type: "text",
                                    placeholder: "https://api.example.com",
                                    value: "{config().base_url}",
                                    oninput: move |e| {
                                        config.set(SubscribeApiConfig {
                                            base_url: e.value(),
                                            ..config()
                                        });
                                    }
                                }
                            }

                            // API Key
                            div { class: "form-control",
                                label { class: "label",
                                    span { class: "label-text font-medium", "API密钥" }
                                }
                                input {
                                    class: "input input-bordered w-full",
                                    r#type: "password",
                                    placeholder: "可选,如果API需要认证",
                                    value: "{config().api_key}",
                                    oninput: move |e| {
                                        config.set(SubscribeApiConfig {
                                            api_key: e.value(),
                                            ..config()
                                        });
                                    }
                                }
                            }

                            // Webhook URL
                            div { class: "form-control",
                                label { class: "label",
                                    span { class: "label-text font-medium", "Webhook回调URL" }
                                }
                                input {
                                    class: "input input-bordered w-full",
                                    r#type: "text",
                                    placeholder: "https://your-domain.com/webhook",
                                    value: "{config().webhook_url}",
                                    oninput: move |e| {
                                        config.set(SubscribeApiConfig {
                                            webhook_url: e.value(),
                                            ..config()
                                        });
                                    }
                                }
                                label { class: "label",
                                    span { class: "label-text-alt", "用于接收订阅推送的回调地址" }
                                }
                            }

                            // Timeout
                            div { class: "form-control",
                                label { class: "label",
                                    span { class: "label-text font-medium",
                                        "超时时间(秒)"
                                        span { class: "text-error ml-1", "*" }
                                    }
                                }
                                input {
                                    class: "input input-bordered w-full",
                                    r#type: "number",
                                    value: "{config().timeout_seconds}",
                                    oninput: move |e| {
                                        if let Ok(timeout) = e.value().parse::<u32>() {
                                            config.set(SubscribeApiConfig {
                                                timeout_seconds: timeout,
                                                ..config()
                                            });
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // Section 3: Token Application
                div { class: "card bg-base-100 shadow-sm",
                    div { class: "card-body",
                        h4 { class: "text-lg font-semibold mb-4", "Token申请配置" }
                        div { class: "space-y-4",
                            // Token Name
                            div { class: "form-control",
                                label { class: "label",
                                    span { class: "label-text font-medium", "Token名称" }
                                }
                                input {
                                    class: "input input-bordered w-full",
                                    r#type: "text",
                                    placeholder: "为此API Token命名",
                                    value: "{config().token_name}",
                                    oninput: move |e| {
                                        config.set(SubscribeApiConfig {
                                            token_name: e.value(),
                                            ..config()
                                        });
                                    }
                                }
                                label { class: "label",
                                    span { class: "label-text-alt", "用于标识和管理API访问令牌" }
                                }
                            }

                            // Token Expiry Days
                            div { class: "form-control",
                                label { class: "label",
                                    span { class: "label-text font-medium", "Token有效期(天)" }
                                }
                                input {
                                    class: "input input-bordered w-full",
                                    r#type: "number",
                                    value: "{config().token_expiry_days}",
                                    oninput: move |e| {
                                        if let Ok(days) = e.value().parse::<u32>() {
                                            config.set(SubscribeApiConfig {
                                                token_expiry_days: days,
                                                ..config()
                                            });
                                        }
                                    }
                                }
                                label { class: "label",
                                    span { class: "label-text-alt", "Token过期后需要重新申请" }
                                }
                            }

                            // Token Scopes (as comma-separated text)
                            div { class: "form-control",
                                label { class: "label",
                                    span { class: "label-text font-medium", "访问权限范围" }
                                }
                                textarea {
                                    class: "textarea textarea-bordered w-full h-24",
                                    placeholder: "read, write, subscribe (用逗号分隔)",
                                    value: "{config().token_scopes.join(\", \")}",
                                    oninput: move |e| {
                                        let scopes: Vec<String> = e.value()
                                            .split(',')
                                            .map(|s| s.trim().to_string())
                                            .filter(|s| !s.is_empty())
                                            .collect();
                                        config.set(SubscribeApiConfig {
                                            token_scopes: scopes,
                                            ..config()
                                        });
                                    }
                                }
                                label { class: "label",
                                    span { class: "label-text-alt", "定义Token可以访问的API权限" }
                                }
                            }

                            // Apply for Token Button
                            div { class: "form-control mt-4",
                                button {
                                    class: "btn btn-outline btn-primary",
                                    r#type: "button",
                                    "申请新Token"
                                }
                                label { class: "label",
                                    span { class: "label-text-alt text-info", "点击此按钮将生成新的API访问令牌" }
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
pub fn DatasourceSubscribeApiAdd() -> Element {
    let mut config = use_signal(SubscribeApiConfig::default);
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
            let subscribeapi_config = DataSourceCreateUpdate{
                id: String::new(),
                name: config().name,
                description: config().description,
                category: DataSourceCategory::Api,
                datasource_type: DataSourceType::SubscribeApi,
                connection_config: serde_json::to_value(config()).unwrap(),
            };
            let response = client
                                .post("/api/v1/datasource/add", Some(req_config), Some(subscribeapi_config))
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
            let subscribeapi_config = DataSourceCreateUpdate{
                id: String::new(),
                name: config().name,
                description: config().description,
                category: DataSourceCategory::Api,
                datasource_type: DataSourceType::SubscribeApi,
                connection_config: serde_json::to_value(config()).unwrap(),
            };
            let response = client
                                .post("/api/v1/datasource/ping", Some(req_config), Some(subscribeapi_config))
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
                        li { "添加Subscribe API数据源" }
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
                                        config.set(SubscribeApiConfig {
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
                                        config.set(SubscribeApiConfig {
                                            description: e.value(),
                                            ..config()
                                        });
                                    }
                                }
                            }
                        }
                    }
                }

                // Section 2: API Configuration
                div { class: "card bg-base-100 shadow-sm",
                    div { class: "card-body",
                        h4 { class: "text-lg font-semibold mb-4", "API配置" }
                        div { class: "space-y-4",
                            // Base URL
                            div { class: "form-control",
                                label { class: "label",
                                    span { class: "label-text font-medium",
                                        "API基础URL"
                                        span { class: "text-error ml-1", "*" }
                                    }
                                }
                                input {
                                    class: "input input-bordered w-full",
                                    r#type: "text",
                                    placeholder: "https://api.example.com",
                                    value: "{config().base_url}",
                                    oninput: move |e| {
                                        config.set(SubscribeApiConfig {
                                            base_url: e.value(),
                                            ..config()
                                        });
                                    }
                                }
                            }

                            // API Key
                            div { class: "form-control",
                                label { class: "label",
                                    span { class: "label-text font-medium", "API密钥" }
                                }
                                input {
                                    class: "input input-bordered w-full",
                                    r#type: "password",
                                    placeholder: "可选,如果API需要认证",
                                    value: "{config().api_key}",
                                    oninput: move |e| {
                                        config.set(SubscribeApiConfig {
                                            api_key: e.value(),
                                            ..config()
                                        });
                                    }
                                }
                            }

                            // Webhook URL
                            div { class: "form-control",
                                label { class: "label",
                                    span { class: "label-text font-medium", "Webhook回调URL" }
                                }
                                input {
                                    class: "input input-bordered w-full",
                                    r#type: "text",
                                    placeholder: "https://your-domain.com/webhook",
                                    value: "{config().webhook_url}",
                                    oninput: move |e| {
                                        config.set(SubscribeApiConfig {
                                            webhook_url: e.value(),
                                            ..config()
                                        });
                                    }
                                }
                                label { class: "label",
                                    span { class: "label-text-alt", "用于接收订阅推送的回调地址" }
                                }
                            }

                            // Timeout
                            div { class: "form-control",
                                label { class: "label",
                                    span { class: "label-text font-medium",
                                        "超时时间(秒)"
                                        span { class: "text-error ml-1", "*" }
                                    }
                                }
                                input {
                                    class: "input input-bordered w-full",
                                    r#type: "number",
                                    value: "{config().timeout_seconds}",
                                    oninput: move |e| {
                                        if let Ok(timeout) = e.value().parse::<u32>() {
                                            config.set(SubscribeApiConfig {
                                                timeout_seconds: timeout,
                                                ..config()
                                            });
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // Section 3: Token Application
                div { class: "card bg-base-100 shadow-sm",
                    div { class: "card-body",
                        h4 { class: "text-lg font-semibold mb-4", "Token申请配置" }
                        div { class: "space-y-4",
                            // Token Name
                            div { class: "form-control",
                                label { class: "label",
                                    span { class: "label-text font-medium", "Token名称" }
                                }
                                input {
                                    class: "input input-bordered w-full",
                                    r#type: "text",
                                    placeholder: "为此API Token命名",
                                    value: "{config().token_name}",
                                    oninput: move |e| {
                                        config.set(SubscribeApiConfig {
                                            token_name: e.value(),
                                            ..config()
                                        });
                                    }
                                }
                                label { class: "label",
                                    span { class: "label-text-alt", "用于标识和管理API访问令牌" }
                                }
                            }

                            // Token Expiry Days
                            div { class: "form-control",
                                label { class: "label",
                                    span { class: "label-text font-medium", "Token有效期(天)" }
                                }
                                input {
                                    class: "input input-bordered w-full",
                                    r#type: "number",
                                    value: "{config().token_expiry_days}",
                                    oninput: move |e| {
                                        if let Ok(days) = e.value().parse::<u32>() {
                                            config.set(SubscribeApiConfig {
                                                token_expiry_days: days,
                                                ..config()
                                            });
                                        }
                                    }
                                }
                                label { class: "label",
                                    span { class: "label-text-alt", "Token过期后需要重新申请" }
                                }
                            }

                            // Token Scopes (as comma-separated text)
                            div { class: "form-control",
                                label { class: "label",
                                    span { class: "label-text font-medium", "访问权限范围" }
                                }
                                textarea {
                                    class: "textarea textarea-bordered w-full h-24",
                                    placeholder: "read, write, subscribe (用逗号分隔)",
                                    value: "{config().token_scopes.join(\", \")}",
                                    oninput: move |e| {
                                        let scopes: Vec<String> = e.value()
                                            .split(',')
                                            .map(|s| s.trim().to_string())
                                            .filter(|s| !s.is_empty())
                                            .collect();
                                        config.set(SubscribeApiConfig {
                                            token_scopes: scopes,
                                            ..config()
                                        });
                                    }
                                }
                                label { class: "label",
                                    span { class: "label-text-alt", "定义Token可以访问的API权限" }
                                }
                            }

                            // Apply for Token Button
                            div { class: "form-control mt-4",
                                button {
                                    class: "btn btn-outline btn-primary",
                                    r#type: "button",
                                    "申请新Token"
                                }
                                label { class: "label",
                                    span { class: "label-text-alt text-info", "点击此按钮将生成新的API访问令牌" }
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
