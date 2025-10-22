use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::utils::{
    cookie,
    request::{HttpRequest, RequestBuilder},
};
use crate::routes::Route;
use crate::models::datasource::*;
use crate::models::protocol::ApiResponse;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
    Patch,
}

impl Default for HttpMethod {
    fn default() -> Self {
        Self::Post
    }
}

impl std::fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HttpMethod::Get => write!(f, "GET"),
            HttpMethod::Post => write!(f, "POST"),
            HttpMethod::Put => write!(f, "PUT"),
            HttpMethod::Delete => write!(f, "DELETE"),
            HttpMethod::Patch => write!(f, "PATCH"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum BodyType {
    None,
    Json,
    FormData,
}

impl Default for BodyType {
    fn default() -> Self {
        Self::Json
    }
}

impl std::fmt::Display for BodyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BodyType::None => write!(f, "无"),
            BodyType::Json => write!(f, "JSON"),
            BodyType::FormData => write!(f, "Form Data"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ApiToken {
    pub id: String,
    pub name: String,
    pub token: String,
    pub scopes: Vec<String>,
    pub expiry_days: u32,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SubscribeApiConfig {
    pub name: String,
    pub description: String,
    pub method: HttpMethod,
    pub path: String,
    pub path_params: HashMap<String, String>,
    pub query_params: HashMap<String, String>,
    pub headers: HashMap<String, String>,
    pub body_type: BodyType,
    pub body_content: String,
    pub webhook_url: String,
    pub timeout_seconds: u32,
    pub tokens: Vec<ApiToken>,
}

impl Default for SubscribeApiConfig {
    fn default() -> Self {
        Self {
            name: String::new(),
            description: String::new(),
            method: HttpMethod::Post,
            path: String::new(),
            path_params: HashMap::new(),
            query_params: HashMap::new(),
            headers: HashMap::new(),
            body_type: BodyType::Json,
            body_content: String::new(),
            webhook_url: String::new(),
            timeout_seconds: 30,
            tokens: Vec::new(),
        }
    }
}

// Shared validation function
fn validate_config(cfg: &SubscribeApiConfig) -> Vec<String> {
    let mut errors = Vec::new();

    if cfg.name.trim().is_empty() {
        errors.push("数据源名称不能为空".to_string());
    }
    if cfg.path.trim().is_empty() {
        errors.push("接口路径不能为空".to_string());
    }
    if !cfg.path.starts_with("/") {
        errors.push("接口路径必须以/开头".to_string());
    }
    if !cfg.webhook_url.trim().is_empty() && !cfg.webhook_url.starts_with("http://") && !cfg.webhook_url.starts_with("https://") {
        errors.push("Webhook URL必须以http://或https://开头".to_string());
    }
    if cfg.timeout_seconds == 0 || cfg.timeout_seconds > 300 {
        errors.push("超时时间必须在1-300秒之间".to_string());
    }

    // Validate JSON body if body_type is JSON
    if cfg.body_type == BodyType::Json && !cfg.body_content.trim().is_empty() {
        if let Err(_) = serde_json::from_str::<serde_json::Value>(&cfg.body_content) {
            errors.push("Body内容不是有效的JSON格式".to_string());
        }
    }

    errors
}

// Get current domain from browser
fn get_current_domain() -> String {
    #[cfg(target_arch = "wasm32")]
    {
        web_sys::window()
            .and_then(|w| w.location().origin().ok())
            .unwrap_or_else(|| "http://localhost:8080".to_string())
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        "http://localhost:8080".to_string()
    }
}

#[component]
pub fn DatasourceSubscribeApiEdit(id: String) -> Element {
    let mut config = use_signal(SubscribeApiConfig::default);
    let mut validation_errors = use_signal(|| Vec::<String>::new());
    let mut is_loading = use_signal(|| true);

    // Signals for dynamic key-value pairs
    let mut new_path_param_key = use_signal(String::new);
    let mut new_path_param_value = use_signal(String::new);
    let mut new_query_param_key = use_signal(String::new);
    let mut new_query_param_value = use_signal(String::new);
    let mut new_header_key = use_signal(String::new);
    let mut new_header_value = use_signal(String::new);

    let navigator = use_navigator();
    let current_domain = get_current_domain();

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
                let subscribeapi_config = DataSourceCreateUpdate {
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
                                    navigator.push(Route::DatasourceOverViewPage {});
                                } else {
                                    let mut errs = errors.clone();
                                    errs.push(result.msg);
                                    validation_errors.set(errs);
                                }
                            }
                            Err(e) => {
                                let mut errs = errors.clone();
                                errs.push(e.to_string());
                                validation_errors.set(errs);
                            }
                        }
                    }
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
            let subscribeapi_config = DataSourceCreateUpdate {
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
                        }
                        Err(e) => {
                            let mut errs = errors.clone();
                            errs.push(e.to_string());
                            validation_errors.set(errs);
                        }
                    }
                }
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
                            Link { to: Route::DatasourceOverViewPage {}, "数据源管理" }
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
                            // HTTP Method
                            div { class: "form-control",
                                label { class: "label",
                                    span { class: "label-text font-medium",
                                        "请求方法"
                                        span { class: "text-error ml-1", "*" }
                                    }
                                }
                                select {
                                    class: "select select-bordered w-full",
                                    value: "{config().method}",
                                    onchange: move |e| {
                                        let method = match e.value().as_str() {
                                            "GET" => HttpMethod::Get,
                                            "POST" => HttpMethod::Post,
                                            "PUT" => HttpMethod::Put,
                                            "DELETE" => HttpMethod::Delete,
                                            "PATCH" => HttpMethod::Patch,
                                            _ => HttpMethod::Post,
                                        };
                                        config.set(SubscribeApiConfig {
                                            method,
                                            ..config()
                                        });
                                    },
                                    option { value: "GET", "GET" }
                                    option { value: "POST", selected: true, "POST" }
                                    option { value: "PUT", "PUT" }
                                    option { value: "DELETE", "DELETE" }
                                    option { value: "PATCH", "PATCH" }
                                }
                            }

                            // Domain (Read-only label) and Path
                            div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                                div { class: "form-control",
                                    label { class: "label",
                                        span { class: "label-text font-medium", "接口域名" }
                                    }
                                    div { class: "flex items-center h-12 px-4 bg-base-200 rounded-lg",
                                        span { class: "text-base-content", "{current_domain}" }
                                    }
                                    label { class: "label",
                                        span { class: "label-text-alt", "当前系统域名" }
                                    }
                                }

                                div { class: "form-control",
                                    label { class: "label",
                                        span { class: "label-text font-medium",
                                            "接口路径"
                                            span { class: "text-error ml-1", "*" }
                                        }
                                    }
                                    input {
                                        class: "input input-bordered w-full",
                                        r#type: "text",
                                        placeholder: "/api/v1/subscribe",
                                        value: "{config().path}",
                                        oninput: move |e| {
                                            config.set(SubscribeApiConfig {
                                                path: e.value(),
                                                ..config()
                                            });
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // Section 3: Parameters
                div { class: "card bg-base-100 shadow-sm",
                    div { class: "card-body",
                        h4 { class: "text-lg font-semibold mb-4", "参数配置" }

                        // Path Parameters
                        div { class: "mb-6",
                            label { class: "label",
                                span { class: "label-text font-medium", "Path参数" }
                            }
                            div { class: "space-y-2",
                                for (key, value) in config().path_params.iter() {
                                    div { class: "flex gap-2 items-center",
                                        input {
                                            class: "input input-bordered flex-1",
                                            r#type: "text",
                                            value: "{key}",
                                            disabled: true
                                        }
                                        input {
                                            class: "input input-bordered flex-1",
                                            r#type: "text",
                                            value: "{value}",
                                            disabled: true
                                        }
                                        button {
                                            class: "btn btn-error btn-sm",
                                            onclick: {
                                                let key = key.clone();
                                                move |_| {
                                                    let mut params = config().path_params.clone();
                                                    params.remove(&key);
                                                    config.set(SubscribeApiConfig {
                                                        path_params: params,
                                                        ..config()
                                                    });
                                                }
                                            },
                                            "删除"
                                        }
                                    }
                                }
                                div { class: "flex gap-2",
                                    input {
                                        class: "input input-bordered flex-1",
                                        r#type: "text",
                                        placeholder: "参数名",
                                        value: "{new_path_param_key()}",
                                        oninput: move |e| new_path_param_key.set(e.value())
                                    }
                                    // Datalist for token selection
                                    div { class: "flex-1 relative",
                                        input {
                                            class: "input input-bordered w-full",
                                            r#type: "text",
                                            list: "path-param-tokens",
                                            placeholder: "参数值 (可选择Token)",
                                            value: "{new_path_param_value()}",
                                            oninput: move |e| new_path_param_value.set(e.value())
                                        }
                                        datalist { id: "path-param-tokens",
                                            for token in config().tokens.iter() {
                                                option { value: "{token.token}", "{token.name}" }
                                            }
                                        }
                                    }
                                    button {
                                        class: "btn btn-primary btn-sm",
                                        onclick: move |_| {
                                            if !new_path_param_key().is_empty() {
                                                let mut params = config().path_params.clone();
                                                params.insert(new_path_param_key(), new_path_param_value());
                                                config.set(SubscribeApiConfig {
                                                    path_params: params,
                                                    ..config()
                                                });
                                                new_path_param_key.set(String::new());
                                                new_path_param_value.set(String::new());
                                            }
                                        },
                                        "添加"
                                    }
                                }
                            }
                        }

                        // Query Parameters
                        div { class: "mb-6",
                            label { class: "label",
                                span { class: "label-text font-medium", "Query参数" }
                            }
                            div { class: "space-y-2",
                                for (key, value) in config().query_params.iter() {
                                    div { class: "flex gap-2 items-center",
                                        input {
                                            class: "input input-bordered flex-1",
                                            r#type: "text",
                                            value: "{key}",
                                            disabled: true
                                        }
                                        input {
                                            class: "input input-bordered flex-1",
                                            r#type: "text",
                                            value: "{value}",
                                            disabled: true
                                        }
                                        button {
                                            class: "btn btn-error btn-sm",
                                            onclick: {
                                                let key = key.clone();
                                                move |_| {
                                                    let mut params = config().query_params.clone();
                                                    params.remove(&key);
                                                    config.set(SubscribeApiConfig {
                                                        query_params: params,
                                                        ..config()
                                                    });
                                                }
                                            },
                                            "删除"
                                        }
                                    }
                                }
                                div { class: "flex gap-2",
                                    input {
                                        class: "input input-bordered flex-1",
                                        r#type: "text",
                                        placeholder: "参数名",
                                        value: "{new_query_param_key()}",
                                        oninput: move |e| new_query_param_key.set(e.value())
                                    }
                                    div { class: "flex-1 relative",
                                        input {
                                            class: "input input-bordered w-full",
                                            r#type: "text",
                                            list: "query-param-tokens",
                                            placeholder: "参数值 (可选择Token)",
                                            value: "{new_query_param_value()}",
                                            oninput: move |e| new_query_param_value.set(e.value())
                                        }
                                        datalist { id: "query-param-tokens",
                                            for token in config().tokens.iter() {
                                                option { value: "{token.token}", "{token.name}" }
                                            }
                                        }
                                    }
                                    button {
                                        class: "btn btn-primary btn-sm",
                                        onclick: move |_| {
                                            if !new_query_param_key().is_empty() {
                                                let mut params = config().query_params.clone();
                                                params.insert(new_query_param_key(), new_query_param_value());
                                                config.set(SubscribeApiConfig {
                                                    query_params: params,
                                                    ..config()
                                                });
                                                new_query_param_key.set(String::new());
                                                new_query_param_value.set(String::new());
                                            }
                                        },
                                        "添加"
                                    }
                                }
                            }
                        }

                        // Headers
                        div {
                            label { class: "label",
                                span { class: "label-text font-medium", "请求头" }
                            }
                            div { class: "space-y-2",
                                for (key, value) in config().headers.iter() {
                                    div { class: "flex gap-2 items-center",
                                        input {
                                            class: "input input-bordered flex-1",
                                            r#type: "text",
                                            value: "{key}",
                                            disabled: true
                                        }
                                        input {
                                            class: "input input-bordered flex-1",
                                            r#type: "text",
                                            value: "{value}",
                                            disabled: true
                                        }
                                        button {
                                            class: "btn btn-error btn-sm",
                                            onclick: {
                                                let key = key.clone();
                                                move |_| {
                                                    let mut headers = config().headers.clone();
                                                    headers.remove(&key);
                                                    config.set(SubscribeApiConfig {
                                                        headers,
                                                        ..config()
                                                    });
                                                }
                                            },
                                            "删除"
                                        }
                                    }
                                }
                                div { class: "flex gap-2",
                                    input {
                                        class: "input input-bordered flex-1",
                                        r#type: "text",
                                        placeholder: "Header名称",
                                        value: "{new_header_key()}",
                                        oninput: move |e| new_header_key.set(e.value())
                                    }
                                    div { class: "flex-1 relative",
                                        input {
                                            class: "input input-bordered w-full",
                                            r#type: "text",
                                            list: "header-tokens",
                                            placeholder: "Header值 (可选择Token)",
                                            value: "{new_header_value()}",
                                            oninput: move |e| new_header_value.set(e.value())
                                        }
                                        datalist { id: "header-tokens",
                                            for token in config().tokens.iter() {
                                                option { value: "{token.token}", "{token.name}" }
                                            }
                                        }
                                    }
                                    button {
                                        class: "btn btn-primary btn-sm",
                                        onclick: move |_| {
                                            if !new_header_key().is_empty() {
                                                let mut headers = config().headers.clone();
                                                headers.insert(new_header_key(), new_header_value());
                                                config.set(SubscribeApiConfig {
                                                    headers,
                                                    ..config()
                                                });
                                                new_header_key.set(String::new());
                                                new_header_value.set(String::new());
                                            }
                                        },
                                        "添加"
                                    }
                                }
                            }
                        }
                    }
                }

                // Section 4: Body
                div { class: "card bg-base-100 shadow-sm",
                    div { class: "card-body",
                        h4 { class: "text-lg font-semibold mb-4", "请求体" }
                        div { class: "space-y-4",
                            // Body Type
                            div { class: "form-control",
                                label { class: "label",
                                    span { class: "label-text font-medium", "Body类型" }
                                }
                                select {
                                    class: "select select-bordered w-full",
                                    value: match config().body_type {
                                        BodyType::None => "none",
                                        BodyType::Json => "json",
                                        BodyType::FormData => "formdata",
                                    },
                                    onchange: move |e| {
                                        let body_type = match e.value().as_str() {
                                            "none" => BodyType::None,
                                            "json" => BodyType::Json,
                                            "formdata" => BodyType::FormData,
                                            _ => BodyType::Json,
                                        };
                                        config.set(SubscribeApiConfig {
                                            body_type,
                                            ..config()
                                        });
                                    },
                                    option { value: "none", "无" }
                                    option { value: "json", selected: true, "JSON" }
                                    option { value: "formdata", "Form Data" }
                                }
                            }

                            // Body Content
                            if config().body_type != BodyType::None {
                                div { class: "form-control",
                                    label { class: "label",
                                        span { class: "label-text font-medium",
                                            if config().body_type == BodyType::Json {
                                                "JSON内容 (JSON Schema格式)"
                                            } else {
                                                "Form Data内容"
                                            }
                                        }
                                    }
                                    textarea {
                                        class: "textarea textarea-bordered w-full h-48 font-mono",
                                        placeholder: if config().body_type == BodyType::Json {
                                            r#"{{"type": "object", "properties": {{"name": {{"type": "string"}}}}}}"#
                                        } else {
                                            "key1=value1&key2=value2"
                                        },
                                        value: "{config().body_content}",
                                        oninput: move |e| {
                                            config.set(SubscribeApiConfig {
                                                body_content: e.value(),
                                                ..config()
                                            });
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // Section 5: Token Management
                div { class: "card bg-base-100 shadow-sm",
                    div { class: "card-body",
                        div { class: "flex justify-between items-center mb-4",
                            h4 { class: "text-lg font-semibold", "Token管理" }
                            Link {
                                to: Route::DatasourceSubscribeApiTokenManagement { id: id.clone() },
                                class: "btn btn-sm btn-primary",
                                "管理Tokens"
                            }
                        }
                        div { class: "overflow-x-auto",
                            if config().tokens.is_empty() {
                                div { class: "text-center py-8 text-base-content/60",
                                    "暂无Token，点击上方\"管理Tokens\"按钮创建"
                                }
                            } else {
                                table { class: "table table-sm",
                                    thead {
                                        tr {
                                            th { "名称" }
                                            th { "Token" }
                                            th { "权限范围" }
                                            th { "有效期" }
                                            th { "创建时间" }
                                        }
                                    }
                                    tbody {
                                        for token in config().tokens.iter() {
                                            tr {
                                                td { "{token.name}" }
                                                td { class: "font-mono text-xs",
                                                    "{token.token.chars().take(20).collect::<String>()}..."
                                                }
                                                td { "{token.scopes.join(\", \")}" }
                                                td { "{token.expiry_days}天" }
                                                td { "{token.created_at}" }
                                            }
                                        }
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
                div { class: "w-1/2 flex-shrink-0",
                    if !validation_errors().is_empty() {
                        ul { class: "list-disc list-inside space-y-1",
                            for error in validation_errors().iter() {
                                li { class: "text-error", "{error}" }
                            }
                        }
                    }
                }
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

    // Signals for dynamic key-value pairs
    let mut new_path_param_key = use_signal(String::new);
    let mut new_path_param_value = use_signal(String::new);
    let mut new_query_param_key = use_signal(String::new);
    let mut new_query_param_value = use_signal(String::new);
    let mut new_header_key = use_signal(String::new);
    let mut new_header_value = use_signal(String::new);

    let navigator = use_navigator();
    let current_domain = get_current_domain();

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
            let subscribeapi_config = DataSourceCreateUpdate {
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
                                navigator.push(Route::DatasourceOverViewPage {});
                            } else {
                                let mut errs = errors.clone();
                                errs.push(result.msg);
                                validation_errors.set(errs);
                            }
                        }
                        Err(e) => {
                            let mut errs = errors.clone();
                            errs.push(e.to_string());
                            validation_errors.set(errs);
                        }
                    }
                }
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
            let subscribeapi_config = DataSourceCreateUpdate {
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
                        }
                        Err(e) => {
                            let mut errs = errors.clone();
                            errs.push(e.to_string());
                            validation_errors.set(errs);
                        }
                    }
                }
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
                            Link { to: Route::DatasourceOverViewPage {}, "数据源管理" }
                        }
                        li { "添加Subscribe API数据源" }
                    }
                }
            }

            // Form Content - Same structure as Edit component
            div { class: "space-y-6",
                // Section 1: Basic Information
                div { class: "card bg-base-100 shadow-sm",
                    div { class: "card-body",
                        h4 { class: "text-lg font-semibold mb-4", "基本信息" }
                        div { class: "space-y-4",
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
                            // HTTP Method
                            div { class: "form-control",
                                label { class: "label",
                                    span { class: "label-text font-medium",
                                        "请求方法"
                                        span { class: "text-error ml-1", "*" }
                                    }
                                }
                                select {
                                    class: "select select-bordered w-full",
                                    value: "{config().method}",
                                    onchange: move |e| {
                                        let method = match e.value().as_str() {
                                            "GET" => HttpMethod::Get,
                                            "POST" => HttpMethod::Post,
                                            "PUT" => HttpMethod::Put,
                                            "DELETE" => HttpMethod::Delete,
                                            "PATCH" => HttpMethod::Patch,
                                            _ => HttpMethod::Post,
                                        };
                                        config.set(SubscribeApiConfig {
                                            method,
                                            ..config()
                                        });
                                    },
                                    option { value: "GET", "GET" }
                                    option { value: "POST", selected: true, "POST" }
                                    option { value: "PUT", "PUT" }
                                    option { value: "DELETE", "DELETE" }
                                    option { value: "PATCH", "PATCH" }
                                }
                            }

                            // Domain (Read-only label) and Path
                            div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                                div { class: "form-control",
                                    label { class: "label",
                                        span { class: "label-text font-medium", "接口域名" }
                                    }
                                    div { class: "flex items-center h-12 px-4 bg-base-200 rounded-lg",
                                        span { class: "text-base-content", "{current_domain}" }
                                    }
                                    label { class: "label",
                                        span { class: "label-text-alt", "当前系统域名" }
                                    }
                                }

                                div { class: "form-control",
                                    label { class: "label",
                                        span { class: "label-text font-medium",
                                            "接口路径"
                                            span { class: "text-error ml-1", "*" }
                                        }
                                    }
                                    input {
                                        class: "input input-bordered w-full",
                                        r#type: "text",
                                        placeholder: "/api/v1/subscribe",
                                        value: "{config().path}",
                                        oninput: move |e| {
                                            config.set(SubscribeApiConfig {
                                                path: e.value(),
                                                ..config()
                                            });
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // Section 3: Parameters
                div { class: "card bg-base-100 shadow-sm",
                    div { class: "card-body",
                        h4 { class: "text-lg font-semibold mb-4", "参数配置" }

                        // Path Parameters
                        div { class: "mb-6",
                            label { class: "label",
                                span { class: "label-text font-medium", "Path参数" }
                            }
                            div { class: "space-y-2",
                                for (key, value) in config().path_params.iter() {
                                    div { class: "flex gap-2 items-center",
                                        input {
                                            class: "input input-bordered flex-1",
                                            r#type: "text",
                                            value: "{key}",
                                            disabled: true
                                        }
                                        input {
                                            class: "input input-bordered flex-1",
                                            r#type: "text",
                                            value: "{value}",
                                            disabled: true
                                        }
                                        button {
                                            class: "btn btn-error btn-sm",
                                            onclick: {
                                                let key = key.clone();
                                                move |_| {
                                                    let mut params = config().path_params.clone();
                                                    params.remove(&key);
                                                    config.set(SubscribeApiConfig {
                                                        path_params: params,
                                                        ..config()
                                                    });
                                                }
                                            },
                                            "删除"
                                        }
                                    }
                                }
                                div { class: "flex gap-2",
                                    input {
                                        class: "input input-bordered flex-1",
                                        r#type: "text",
                                        placeholder: "参数名",
                                        value: "{new_path_param_key()}",
                                        oninput: move |e| new_path_param_key.set(e.value())
                                    }
                                    div { class: "flex-1 relative",
                                        input {
                                            class: "input input-bordered w-full",
                                            r#type: "text",
                                            list: "path-param-tokens-add",
                                            placeholder: "参数值 (可选择Token)",
                                            value: "{new_path_param_value()}",
                                            oninput: move |e| new_path_param_value.set(e.value())
                                        }
                                        datalist { id: "path-param-tokens-add",
                                            for token in config().tokens.iter() {
                                                option { value: "{token.token}", "{token.name}" }
                                            }
                                        }
                                    }
                                    button {
                                        class: "btn btn-primary btn-sm",
                                        onclick: move |_| {
                                            if !new_path_param_key().is_empty() {
                                                let mut params = config().path_params.clone();
                                                params.insert(new_path_param_key(), new_path_param_value());
                                                config.set(SubscribeApiConfig {
                                                    path_params: params,
                                                    ..config()
                                                });
                                                new_path_param_key.set(String::new());
                                                new_path_param_value.set(String::new());
                                            }
                                        },
                                        "添加"
                                    }
                                }
                            }
                        }

                        // Query Parameters
                        div { class: "mb-6",
                            label { class: "label",
                                span { class: "label-text font-medium", "Query参数" }
                            }
                            div { class: "space-y-2",
                                for (key, value) in config().query_params.iter() {
                                    div { class: "flex gap-2 items-center",
                                        input {
                                            class: "input input-bordered flex-1",
                                            r#type: "text",
                                            value: "{key}",
                                            disabled: true
                                        }
                                        input {
                                            class: "input input-bordered flex-1",
                                            r#type: "text",
                                            value: "{value}",
                                            disabled: true
                                        }
                                        button {
                                            class: "btn btn-error btn-sm",
                                            onclick: {
                                                let key = key.clone();
                                                move |_| {
                                                    let mut params = config().query_params.clone();
                                                    params.remove(&key);
                                                    config.set(SubscribeApiConfig {
                                                        query_params: params,
                                                        ..config()
                                                    });
                                                }
                                            },
                                            "删除"
                                        }
                                    }
                                }
                                div { class: "flex gap-2",
                                    input {
                                        class: "input input-bordered flex-1",
                                        r#type: "text",
                                        placeholder: "参数名",
                                        value: "{new_query_param_key()}",
                                        oninput: move |e| new_query_param_key.set(e.value())
                                    }
                                    div { class: "flex-1 relative",
                                        input {
                                            class: "input input-bordered w-full",
                                            r#type: "text",
                                            list: "query-param-tokens-add",
                                            placeholder: "参数值 (可选择Token)",
                                            value: "{new_query_param_value()}",
                                            oninput: move |e| new_query_param_value.set(e.value())
                                        }
                                        datalist { id: "query-param-tokens-add",
                                            for token in config().tokens.iter() {
                                                option { value: "{token.token}", "{token.name}" }
                                            }
                                        }
                                    }
                                    button {
                                        class: "btn btn-primary btn-sm",
                                        onclick: move |_| {
                                            if !new_query_param_key().is_empty() {
                                                let mut params = config().query_params.clone();
                                                params.insert(new_query_param_key(), new_query_param_value());
                                                config.set(SubscribeApiConfig {
                                                    query_params: params,
                                                    ..config()
                                                });
                                                new_query_param_key.set(String::new());
                                                new_query_param_value.set(String::new());
                                            }
                                        },
                                        "添加"
                                    }
                                }
                            }
                        }

                        // Headers
                        div {
                            label { class: "label",
                                span { class: "label-text font-medium", "请求头" }
                            }
                            div { class: "space-y-2",
                                for (key, value) in config().headers.iter() {
                                    div { class: "flex gap-2 items-center",
                                        input {
                                            class: "input input-bordered flex-1",
                                            r#type: "text",
                                            value: "{key}",
                                            disabled: true
                                        }
                                        input {
                                            class: "input input-bordered flex-1",
                                            r#type: "text",
                                            value: "{value}",
                                            disabled: true
                                        }
                                        button {
                                            class: "btn btn-error btn-sm",
                                            onclick: {
                                                let key = key.clone();
                                                move |_| {
                                                    let mut headers = config().headers.clone();
                                                    headers.remove(&key);
                                                    config.set(SubscribeApiConfig {
                                                        headers,
                                                        ..config()
                                                    });
                                                }
                                            },
                                            "删除"
                                        }
                                    }
                                }
                                div { class: "flex gap-2",
                                    input {
                                        class: "input input-bordered flex-1",
                                        r#type: "text",
                                        placeholder: "Header名称",
                                        value: "{new_header_key()}",
                                        oninput: move |e| new_header_key.set(e.value())
                                    }
                                    div { class: "flex-1 relative",
                                        input {
                                            class: "input input-bordered w-full",
                                            r#type: "text",
                                            list: "header-tokens-add",
                                            placeholder: "Header值 (可选择Token)",
                                            value: "{new_header_value()}",
                                            oninput: move |e| new_header_value.set(e.value())
                                        }
                                        datalist { id: "header-tokens-add",
                                            for token in config().tokens.iter() {
                                                option { value: "{token.token}", "{token.name}" }
                                            }
                                        }
                                    }
                                    button {
                                        class: "btn btn-primary btn-sm",
                                        onclick: move |_| {
                                            if !new_header_key().is_empty() {
                                                let mut headers = config().headers.clone();
                                                headers.insert(new_header_key(), new_header_value());
                                                config.set(SubscribeApiConfig {
                                                    headers,
                                                    ..config()
                                                });
                                                new_header_key.set(String::new());
                                                new_header_value.set(String::new());
                                            }
                                        },
                                        "添加"
                                    }
                                }
                            }
                        }
                    }
                }

                // Section 4: Body
                div { class: "card bg-base-100 shadow-sm",
                    div { class: "card-body",
                        h4 { class: "text-lg font-semibold mb-4", "请求体" }
                        div { class: "space-y-4",
                            // Body Type
                            div { class: "form-control",
                                label { class: "label",
                                    span { class: "label-text font-medium", "Body类型" }
                                }
                                select {
                                    class: "select select-bordered w-full",
                                    value: match config().body_type {
                                        BodyType::None => "none",
                                        BodyType::Json => "json",
                                        BodyType::FormData => "formdata",
                                    },
                                    onchange: move |e| {
                                        let body_type = match e.value().as_str() {
                                            "none" => BodyType::None,
                                            "json" => BodyType::Json,
                                            "formdata" => BodyType::FormData,
                                            _ => BodyType::Json,
                                        };
                                        config.set(SubscribeApiConfig {
                                            body_type,
                                            ..config()
                                        });
                                    },
                                    option { value: "none", "无" }
                                    option { value: "json", selected: true, "JSON" }
                                    option { value: "formdata", "Form Data" }
                                }
                            }

                            // Body Content
                            if config().body_type != BodyType::None {
                                div { class: "form-control",
                                    label { class: "label",
                                        span { class: "label-text font-medium",
                                            if config().body_type == BodyType::Json {
                                                "JSON内容 (JSON Schema格式)"
                                            } else {
                                                "Form Data内容"
                                            }
                                        }
                                    }
                                    textarea {
                                        class: "textarea textarea-bordered w-full h-48 font-mono",
                                        placeholder: if config().body_type == BodyType::Json {
                                            r#"{{"type": "object", "properties": {{"name": {{"type": "string"}}}}}}"#
                                        } else {
                                            "key1=value1&key2=value2"
                                        },
                                        value: "{config().body_content}",
                                        oninput: move |e| {
                                            config.set(SubscribeApiConfig {
                                                body_content: e.value(),
                                                ..config()
                                            });
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // Section 5: Token Management Notice
                div { class: "alert alert-info",
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        fill: "none",
                        view_box: "0 0 24 24",
                        class: "stroke-current shrink-0 w-6 h-6",
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            stroke_width: "2",
                            d: "M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
                        }
                    }
                    span { "保存数据源后，可以在编辑页面管理Token并在参数中使用" }
                }
            }

            // Error Display and Action Buttons
            div {
                class: "flex items-start justify-between mt-6",
                div { class: "w-1/2 flex-shrink-0",
                    if !validation_errors().is_empty() {
                        ul { class: "list-disc list-inside space-y-1",
                            for error in validation_errors().iter() {
                                li { class: "text-error", "{error}" }
                            }
                        }
                    }
                }
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
pub fn DatasourceSubscribeApiTokenManagement(id: String) -> Element {
    let mut config = use_signal(SubscribeApiConfig::default);
    let mut validation_errors = use_signal(|| Vec::<String>::new());
    let mut is_loading = use_signal(|| true);
    
    // Token application fields
    let mut new_token_name = use_signal(String::new);
    let mut new_token_expiry_days = use_signal(|| 30u32);
    let mut new_token_scopes = use_signal(String::new);

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

    let handle_apply_token = move |_| {
        if new_token_name().trim().is_empty() {
            validation_errors.set(vec!["Token名称不能为空".to_string()]);
            return;
        }
        spawn(async move {
            let scopes: Vec<String> = new_token_scopes()
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();

            // Generate UUID-like token
            let token_id = format!("{:x}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos());
            let token_value = format!("sk-{}", &token_id[..32]);

            let new_token = ApiToken {
                id: token_id.clone(),
                name: new_token_name(),
                token: token_value,
                scopes,
                expiry_days: new_token_expiry_days(),
                created_at: {
                    #[cfg(target_arch = "wasm32")]
                    {
                        js_sys::Date::new_0().to_iso_string().as_string().unwrap_or_default()
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    {
                        chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
                    }
                },
            };

            let mut tokens = config().tokens.clone();
            tokens.push(new_token);
            config.set(SubscribeApiConfig {
                tokens,
                ..config()
            });

            // Clear form
            new_token_name.set(String::new());
            new_token_expiry_days.set(30);
            new_token_scopes.set(String::new());
            validation_errors.set(Vec::new());
        });
    };

    let handle_delete_token = move |token_id: String| {
        spawn(async move {
            let mut tokens = config().tokens.clone();
            tokens.retain(|t| t.id != token_id);
            config.set(SubscribeApiConfig {
                tokens,
                ..config()
            });
        });
    };

    let handle_save = {
        let id = id.clone();
        move |_| {
            let id = id.clone();
            spawn(async move {
                let client = crate::utils::request::create_client("http://localhost:3000");
                let req_config = RequestBuilder::new()
                    .header("Content-Type", "application/json")
                    .header("Cookie", &cookie::get_browser_cookies())
                    .build();
                let subscribeapi_config = DataSourceCreateUpdate {
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
                                    navigator.push(Route::DatasourceSubscribeApiEdit { id: id.clone() });
                                } else {
                                    validation_errors.set(vec![result.msg]);
                                }
                            }
                            Err(e) => {
                                validation_errors.set(vec![e.to_string()]);
                            }
                        }
                    }
                    Err(e) => {
                        validation_errors.set(vec![e.to_string()]);
                    }
                }
            });
        }
    };

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
                            Link { to: Route::DatasourceOverViewPage {}, "数据源管理" }
                        }
                        li {
                            Link { to: Route::DatasourceSubscribeApiEdit { id: id.clone() }, "编辑数据源" }
                        }
                        li { "Token管理" }
                    }
                }
            }

            // Main Content
            div { class: "space-y-6",
                // Section 1: Create New Token
                div { class: "card bg-base-100 shadow-sm",
                    div { class: "card-body",
                        h4 { class: "text-lg font-semibold mb-4", "创建新Token" }
                        div { class: "space-y-4",
                            // Token Name
                            div { class: "form-control",
                                label { class: "label",
                                    span { class: "label-text font-medium",
                                        "Token名称"
                                        span { class: "text-error ml-1", "*" }
                                    }
                                }
                                input {
                                    class: "input input-bordered w-full",
                                    r#type: "text",
                                    placeholder: "为此API Token命名",
                                    value: "{new_token_name()}",
                                    oninput: move |e| new_token_name.set(e.value())
                                }
                                label { class: "label",
                                    span { class: "label-text-alt", "用于标识和管理API访问令牌" }
                                }
                            }

                            // Token Expiry Days and Scopes in grid
                            div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                                div { class: "form-control",
                                    label { class: "label",
                                        span { class: "label-text font-medium", "Token有效期(天)" }
                                    }
                                    input {
                                        class: "input input-bordered w-full",
                                        r#type: "number",
                                        value: "{new_token_expiry_days()}",
                                        oninput: move |e| {
                                            if let Ok(days) = e.value().parse::<u32>() {
                                                new_token_expiry_days.set(days);
                                            }
                                        }
                                    }
                                    label { class: "label",
                                        span { class: "label-text-alt", "Token过期后需要重新申请" }
                                    }
                                }

                                div { class: "form-control",
                                    label { class: "label",
                                        span { class: "label-text font-medium", "访问权限范围" }
                                    }
                                    input {
                                        class: "input input-bordered w-full",
                                        r#type: "text",
                                        placeholder: "read, write, subscribe (用逗号分隔)",
                                        value: "{new_token_scopes()}",
                                        oninput: move |e| new_token_scopes.set(e.value())
                                    }
                                    label { class: "label",
                                        span { class: "label-text-alt", "定义Token可以访问的API权限" }
                                    }
                                }
                            }

                            // Apply button
                            button {
                                class: "btn btn-primary",
                                onclick: handle_apply_token,
                                "申请新Token"
                            }
                        }
                    }
                }

                // Section 2: Existing Tokens
                div { class: "card bg-base-100 shadow-sm",
                    div { class: "card-body",
                        h4 { class: "text-lg font-semibold mb-4", "已创建的Tokens" }
                        div { class: "overflow-x-auto",
                            if config().tokens.is_empty() {
                                div { class: "text-center py-12 text-base-content/60",
                                    p { class: "text-lg", "暂无Token" }
                                    p { class: "text-sm mt-2", "点击上方\"申请新Token\"按钮创建" }
                                }
                            } else {
                                table { class: "table",
                                    thead {
                                        tr {
                                            th { "名称" }
                                            th { "Token" }
                                            th { "权限范围" }
                                            th { "有效期" }
                                            th { "创建时间" }
                                            th { "操作" }
                                        }
                                    }
                                    tbody {
                                        for token in config().tokens.iter() {
                                            tr {
                                                td { "{token.name}" }
                                                td {
                                                    div { class: "flex items-center gap-2",
                                                        span { class: "font-mono text-xs",
                                                            "{token.token.chars().take(30).collect::<String>()}..."
                                                        }
                                                        button {
                                                            class: "btn btn-xs btn-ghost",
                                                            "复制"
                                                        }
                                                    }
                                                }
                                                td { "{token.scopes.join(\", \")}" }
                                                td { "{token.expiry_days}天" }
                                                td { "{token.created_at}" }
                                                td {
                                                    button {
                                                        class: "btn btn-error btn-sm",
                                                        onclick: {
                                                            let token_id = token.id.clone();
                                                            move |_| handle_delete_token(token_id.clone())
                                                        },
                                                        "删除"
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Error Display and Action Buttons
            div { class: "flex items-start justify-between mt-6",
                div { class: "w-1/2 flex-shrink-0",
                    if !validation_errors().is_empty() {
                        ul { class: "list-disc list-inside space-y-1",
                            for error in validation_errors().iter() {
                                li { class: "text-error", "{error}" }
                            }
                        }
                    }
                }
                div { class: "flex gap-4 flex-shrink-0",
                    Link {
                        to: Route::DatasourceSubscribeApiEdit { id: id.clone() },
                        class: "btn btn-ghost",
                        "返回编辑"
                    }
                    button {
                        class: "btn btn-primary",
                        onclick: handle_save,
                        "保存并返回"
                    }
                }
            }
        }
    }
}
