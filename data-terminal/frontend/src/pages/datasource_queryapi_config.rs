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
        Self::Get
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
        Self::None
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
pub struct QueryApiConfig {
    pub name: String,
    pub description: String,
    pub method: HttpMethod,
    pub domain: String,
    pub path: String,
    pub path_params: HashMap<String, String>,
    pub query_params: HashMap<String, String>,
    pub headers: HashMap<String, String>,
    pub body_type: BodyType,
    pub body_content: String,
    pub response_schema: String,
}

impl Default for QueryApiConfig {
    fn default() -> Self {
        Self {
            name: String::new(),
            description: String::new(),
            method: HttpMethod::Get,
            domain: String::new(),
            path: String::new(),
            path_params: HashMap::new(),
            query_params: HashMap::new(),
            headers: HashMap::new(),
            body_type: BodyType::None,
            body_content: String::new(),
            response_schema: String::new(),
        }
    }
}

// Shared validation function
fn validate_config(cfg: &QueryApiConfig) -> Vec<String> {
    let mut errors = Vec::new();

    if cfg.name.trim().is_empty() {
        errors.push("数据源名称不能为空".to_string());
    }
    if cfg.domain.trim().is_empty() {
        errors.push("接口域名不能为空".to_string());
    }
    if !cfg.domain.starts_with("http://") && !cfg.domain.starts_with("https://") {
        errors.push("接口域名必须以http://或https://开头".to_string());
    }
    if cfg.path.trim().is_empty() {
        errors.push("接口路径不能为空".to_string());
    }
    if !cfg.path.starts_with("/") {
        errors.push("接口路径必须以/开头".to_string());
    }

    // Validate JSON body if body_type is JSON
    if cfg.body_type == BodyType::Json && !cfg.body_content.trim().is_empty() {
        if let Err(_) = serde_json::from_str::<serde_json::Value>(&cfg.body_content) {
            errors.push("Body内容不是有效的JSON格式".to_string());
        }
    }

    // Validate response schema if not empty
    if !cfg.response_schema.trim().is_empty() {
        if let Err(_) = serde_json::from_str::<serde_json::Value>(&cfg.response_schema) {
            errors.push("Response Schema不是有效的JSON格式".to_string());
        }
    }

    errors
}

#[component]
pub fn DatasourceQueryApiEdit(id: String) -> Element {
    let mut config = use_signal(QueryApiConfig::default);
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
                                    if let Ok(queryapi_config) = serde_json::from_value::<QueryApiConfig>(ds.connection_config) {
                                        config.set(queryapi_config);
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
                let queryapi_config = DataSourceCreateUpdate {
                    id: id.clone(),
                    name: config().name,
                    description: config().description,
                    category: DataSourceCategory::Api,
                    datasource_type: DataSourceType::QueryApi,
                    connection_config: serde_json::to_value(config()).unwrap(),
                };
                let response = client
                    .post("/api/v1/datasource/update", Some(req_config), Some(queryapi_config))
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
            let queryapi_config = DataSourceCreateUpdate {
                id: String::new(),
                name: config().name,
                description: config().description,
                category: DataSourceCategory::Api,
                datasource_type: DataSourceType::QueryApi,
                connection_config: serde_json::to_value(config()).unwrap(),
            };
            let response = client
                .post("/api/v1/datasource/ping", Some(req_config), Some(queryapi_config))
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
                        li { "编辑Query API数据源" }
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
                                        config.set(QueryApiConfig {
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
                                        config.set(QueryApiConfig {
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
                                            _ => HttpMethod::Get,
                                        };
                                        config.set(QueryApiConfig {
                                            method,
                                            ..config()
                                        });
                                    },
                                    option { value: "GET", "GET" }
                                    option { value: "POST", "POST" }
                                    option { value: "PUT", "PUT" }
                                    option { value: "DELETE", "DELETE" }
                                    option { value: "PATCH", "PATCH" }
                                }
                            }

                            // Domain and Path
                            div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                                div { class: "form-control",
                                    label { class: "label",
                                        span { class: "label-text font-medium",
                                            "接口域名"
                                            span { class: "text-error ml-1", "*" }
                                        }
                                    }
                                    input {
                                        class: "input input-bordered w-full",
                                        r#type: "text",
                                        placeholder: "https://api.example.com",
                                        value: "{config().domain}",
                                        oninput: move |e| {
                                            config.set(QueryApiConfig {
                                                domain: e.value(),
                                                ..config()
                                            });
                                        }
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
                                        placeholder: "/api/v1/users",
                                        value: "{config().path}",
                                        oninput: move |e| {
                                            config.set(QueryApiConfig {
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
                                                    config.set(QueryApiConfig {
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
                                    input {
                                        class: "input input-bordered flex-1",
                                        r#type: "text",
                                        placeholder: "参数值",
                                        value: "{new_path_param_value()}",
                                        oninput: move |e| new_path_param_value.set(e.value())
                                    }
                                    button {
                                        class: "btn btn-primary btn-sm",
                                        onclick: move |_| {
                                            if !new_path_param_key().is_empty() {
                                                let mut params = config().path_params.clone();
                                                params.insert(new_path_param_key(), new_path_param_value());
                                                config.set(QueryApiConfig {
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
                                                    config.set(QueryApiConfig {
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
                                    input {
                                        class: "input input-bordered flex-1",
                                        r#type: "text",
                                        placeholder: "参数值",
                                        value: "{new_query_param_value()}",
                                        oninput: move |e| new_query_param_value.set(e.value())
                                    }
                                    button {
                                        class: "btn btn-primary btn-sm",
                                        onclick: move |_| {
                                            if !new_query_param_key().is_empty() {
                                                let mut params = config().query_params.clone();
                                                params.insert(new_query_param_key(), new_query_param_value());
                                                config.set(QueryApiConfig {
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
                                                    config.set(QueryApiConfig {
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
                                    input {
                                        class: "input input-bordered flex-1",
                                        r#type: "text",
                                        placeholder: "Header值",
                                        value: "{new_header_value()}",
                                        oninput: move |e| new_header_value.set(e.value())
                                    }
                                    button {
                                        class: "btn btn-primary btn-sm",
                                        onclick: move |_| {
                                            if !new_header_key().is_empty() {
                                                let mut headers = config().headers.clone();
                                                headers.insert(new_header_key(), new_header_value());
                                                config.set(QueryApiConfig {
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
                                            _ => BodyType::None,
                                        };
                                        config.set(QueryApiConfig {
                                            body_type,
                                            ..config()
                                        });
                                    },
                                    option { value: "none", "无" }
                                    option { value: "json", "JSON" }
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
                                            config.set(QueryApiConfig {
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

                // Section 5: Response
                div { class: "card bg-base-100 shadow-sm",
                    div { class: "card-body",
                        h4 { class: "text-lg font-semibold mb-4", "响应配置" }
                        div { class: "form-control",
                            label { class: "label",
                                span { class: "label-text font-medium", "Response Schema (JSON格式)" }
                            }
                            textarea {
                                class: "textarea textarea-bordered w-full h-48 font-mono",
                                placeholder: r#"{{"type": "object", "properties": {{"id": {{"type": "number"}}, "name": {{"type": "string"}}}}}}"#,
                                value: "{config().response_schema}",
                                oninput: move |e| {
                                    config.set(QueryApiConfig {
                                        response_schema: e.value(),
                                        ..config()
                                    });
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
pub fn DatasourceQueryApiAdd() -> Element {
    let mut config = use_signal(QueryApiConfig::default);
    let mut validation_errors = use_signal(|| Vec::<String>::new());

    // Signals for dynamic key-value pairs
    let mut new_path_param_key = use_signal(String::new);
    let mut new_path_param_value = use_signal(String::new);
    let mut new_query_param_key = use_signal(String::new);
    let mut new_query_param_value = use_signal(String::new);
    let mut new_header_key = use_signal(String::new);
    let mut new_header_value = use_signal(String::new);

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
            let queryapi_config = DataSourceCreateUpdate {
                id: String::new(),
                name: config().name,
                description: config().description,
                category: DataSourceCategory::Api,
                datasource_type: DataSourceType::QueryApi,
                connection_config: serde_json::to_value(config()).unwrap(),
            };
            let response = client
                .post("/api/v1/datasource/add", Some(req_config), Some(queryapi_config))
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
            let queryapi_config = DataSourceCreateUpdate {
                id: String::new(),
                name: config().name,
                description: config().description,
                category: DataSourceCategory::Api,
                datasource_type: DataSourceType::QueryApi,
                connection_config: serde_json::to_value(config()).unwrap(),
            };
            let response = client
                .post("/api/v1/datasource/ping", Some(req_config), Some(queryapi_config))
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
                        li { "添加Query API数据源" }
                    }
                }
            }

            // Form Content - Same as Edit component
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
                                        config.set(QueryApiConfig {
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
                                        config.set(QueryApiConfig {
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
                                            _ => HttpMethod::Get,
                                        };
                                        config.set(QueryApiConfig {
                                            method,
                                            ..config()
                                        });
                                    },
                                    option { value: "GET", "GET" }
                                    option { value: "POST", "POST" }
                                    option { value: "PUT", "PUT" }
                                    option { value: "DELETE", "DELETE" }
                                    option { value: "PATCH", "PATCH" }
                                }
                            }

                            // Domain and Path
                            div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                                div { class: "form-control",
                                    label { class: "label",
                                        span { class: "label-text font-medium",
                                            "接口域名"
                                            span { class: "text-error ml-1", "*" }
                                        }
                                    }
                                    input {
                                        class: "input input-bordered w-full",
                                        r#type: "text",
                                        placeholder: "https://api.example.com",
                                        value: "{config().domain}",
                                        oninput: move |e| {
                                            config.set(QueryApiConfig {
                                                domain: e.value(),
                                                ..config()
                                            });
                                        }
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
                                        placeholder: "/api/v1/users",
                                        value: "{config().path}",
                                        oninput: move |e| {
                                            config.set(QueryApiConfig {
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
                                                    config.set(QueryApiConfig {
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
                                    input {
                                        class: "input input-bordered flex-1",
                                        r#type: "text",
                                        placeholder: "参数值",
                                        value: "{new_path_param_value()}",
                                        oninput: move |e| new_path_param_value.set(e.value())
                                    }
                                    button {
                                        class: "btn btn-primary btn-sm",
                                        onclick: move |_| {
                                            if !new_path_param_key().is_empty() {
                                                let mut params = config().path_params.clone();
                                                params.insert(new_path_param_key(), new_path_param_value());
                                                config.set(QueryApiConfig {
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
                                                    config.set(QueryApiConfig {
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
                                    input {
                                        class: "input input-bordered flex-1",
                                        r#type: "text",
                                        placeholder: "参数值",
                                        value: "{new_query_param_value()}",
                                        oninput: move |e| new_query_param_value.set(e.value())
                                    }
                                    button {
                                        class: "btn btn-primary btn-sm",
                                        onclick: move |_| {
                                            if !new_query_param_key().is_empty() {
                                                let mut params = config().query_params.clone();
                                                params.insert(new_query_param_key(), new_query_param_value());
                                                config.set(QueryApiConfig {
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
                                                    config.set(QueryApiConfig {
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
                                    input {
                                        class: "input input-bordered flex-1",
                                        r#type: "text",
                                        placeholder: "Header值",
                                        value: "{new_header_value()}",
                                        oninput: move |e| new_header_value.set(e.value())
                                    }
                                    button {
                                        class: "btn btn-primary btn-sm",
                                        onclick: move |_| {
                                            if !new_header_key().is_empty() {
                                                let mut headers = config().headers.clone();
                                                headers.insert(new_header_key(), new_header_value());
                                                config.set(QueryApiConfig {
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
                                            _ => BodyType::None,
                                        };
                                        config.set(QueryApiConfig {
                                            body_type,
                                            ..config()
                                        });
                                    },
                                    option { value: "none", "无" }
                                    option { value: "json", "JSON" }
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
                                            config.set(QueryApiConfig {
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

                // Section 5: Response
                div { class: "card bg-base-100 shadow-sm",
                    div { class: "card-body",
                        h4 { class: "text-lg font-semibold mb-4", "响应配置" }
                        div { class: "form-control",
                            label { class: "label",
                                span { class: "label-text font-medium", "Response Schema (JSON格式)" }
                            }
                            textarea {
                                class: "textarea textarea-bordered w-full h-48 font-mono",
                                placeholder: r#"{{"type": "object", "properties": {{"id": {{"type": "number"}}, "name": {{"type": "string"}}}}}}"#,
                                value: "{config().response_schema}",
                                oninput: move |e| {
                                    config.set(QueryApiConfig {
                                        response_schema: e.value(),
                                        ..config()
                                    });
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
