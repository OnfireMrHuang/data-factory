
use gloo::net::http::{Request as GlooRequest};
use serde::{de::DeserializeOwned, Serialize};
use std::collections::HashMap;
use crate::utils::error::{RequestError, RequestResult};

/// HTTP 请求方法枚举
#[derive(Debug, Clone, Copy)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
}

/// HTTP 请求配置
#[derive(Debug, Clone)]
pub struct RequestConfig {
    pub headers: Option<HashMap<String, String>>,
    pub query_params: Option<HashMap<String, String>>,
    pub cookies: Option<HashMap<String, String>>,
    pub timeout: Option<std::time::Duration>,
}

impl Default for RequestConfig {
    fn default() -> Self {
        Self {
            headers: None,
            query_params: None,
            cookies: None,
            timeout: None,
        }
    }
}

/// HTTP 请求 trait，支持异步操作
#[async_trait::async_trait(?Send)]
pub trait HttpRequest {
    /// 发送 HTTP 请求
    async fn request<T>(
        &self,
        method: HttpMethod,
        url: &str,
        config: Option<RequestConfig>,
        body: Option<T>,
    ) -> RequestResult<String>
    where
        T: Serialize + Send + Sync;

    /// GET 请求
    async fn get(&self, url: &str, config: Option<RequestConfig>) -> RequestResult<String> {
        self.request(HttpMethod::GET, url, config, Option::<()>::None).await
    }

    /// POST 请求
    async fn post<T>(&self, url: &str, config: Option<RequestConfig>, body: T) -> RequestResult<String>
    where
        T: Serialize + Send + Sync,
    {
        self.request(HttpMethod::POST, url, config, Some(body)).await
    }

    /// PUT 请求
    async fn put<T>(&self, url: &str, config: Option<RequestConfig>, body: T) -> RequestResult<String>
    where
        T: Serialize + Send + Sync,
    {
        self.request(HttpMethod::PUT, url, config, Some(body)).await
    }

    /// DELETE 请求
    async fn delete(&self, url: &str, config: Option<RequestConfig>) -> RequestResult<String> {
        self.request(HttpMethod::DELETE, url, config, Option::<()>::None).await
    }

    /// PATCH 请求
    async fn patch<T>(&self, url: &str, config: Option<RequestConfig>, body: T) -> RequestResult<String>
    where
        T: Serialize + Send + Sync,
    {
        self.request(HttpMethod::PATCH, url, config, Some(body)).await
    }

    /// 发送请求并反序列化为指定类型
    async fn request_json<T, R>(
        &self,
        method: HttpMethod,
        url: &str,
        config: Option<RequestConfig>,
        body: Option<T>,
    ) -> RequestResult<R>
    where
        T: Serialize + Send + Sync,
        R: DeserializeOwned,
    {
        let response = self.request(method, url, config, body).await?;
        serde_json::from_str(&response)
            .map_err(|e| RequestError::deserialization_error(format!("JSON deserialization error: {}", e)))
    }

    /// GET 请求并反序列化为 JSON
    async fn get_json<R>(&self, url: &str, config: Option<RequestConfig>) -> RequestResult<R>
    where
        R: DeserializeOwned,
    {
        self.request_json(HttpMethod::GET, url, config, Option::<()>::None).await
    }

    /// POST 请求并反序列化为 JSON
    async fn post_json<T, R>(
        &self,
        url: &str,
        config: Option<RequestConfig>,
        body: T,
    ) -> RequestResult<R>
    where
        T: Serialize + Send + Sync,
        R: DeserializeOwned,
    {
        self.request_json(HttpMethod::POST, url, config, Some(body)).await
    }
}

/// 使用 gloo-net 的 HTTP 客户端
pub struct HttpClient {
    base_url: String,
    auth_token: Option<String>,
    cookies: HashMap<String, String>,
}

impl HttpClient {
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            auth_token: None,
            cookies: HashMap::new(),
        }
    }

    pub fn with_auth(base_url: String, auth_token: String) -> Self {
        Self {
            base_url,
            auth_token: Some(auth_token),
            cookies: HashMap::new(),
        }
    }

    pub fn set_auth_token(&mut self, token: String) {
        self.auth_token = Some(token);
    }

    pub fn clear_auth_token(&mut self) {
        self.auth_token = None;
    }

    /// 设置单个 Cookie
    pub fn set_cookie(&mut self, name: &str, value: &str) {
        self.cookies.insert(name.to_string(), value.to_string());
    }

    /// 批量设置 Cookies
    pub fn set_cookies(&mut self, cookies: HashMap<String, String>) {
        self.cookies.extend(cookies);
    }

    /// 获取单个 Cookie
    pub fn get_cookie(&self, name: &str) -> Option<&String> {
        self.cookies.get(name)
    }

    /// 获取所有 Cookies
    pub fn get_cookies(&self) -> &HashMap<String, String> {
        &self.cookies
    }

    /// 删除单个 Cookie
    pub fn remove_cookie(&mut self, name: &str) {
        self.cookies.remove(name);
    }

    /// 清空所有 Cookies
    pub fn clear_cookies(&mut self) {
        self.cookies.clear();
    }

    /// 构建 Cookie 字符串
    fn build_cookie_string(&self, request_cookies: Option<&HashMap<String, String>>) -> Option<String> {
        let mut all_cookies = self.cookies.clone();
        
        // 合并请求级别的 cookies
        if let Some(req_cookies) = request_cookies {
            all_cookies.extend(req_cookies.clone());
        }

        if all_cookies.is_empty() {
            return None;
        }

        let cookie_string = all_cookies
            .iter()
            .map(|(key, value)| format!("{}={}", key, value))
            .collect::<Vec<_>>()
            .join("; ");

        Some(cookie_string)
    }

    fn build_url(&self, path: &str) -> String {
        format!("{}{}", self.base_url.trim_end_matches('/'), path)
    }

    fn apply_query_params(&self, url: &str, config: &RequestConfig) -> String {
        if let Some(params) = &config.query_params {
            let mut url = url.to_string();
            let mut first = true;
            for (key, value) in params {
                if first {
                    url.push('?');
                    first = false;
                } else {
                    url.push('&');
                }
                url.push_str(&format!("{}={}", key, value));
            }
            url
        } else {
            url.to_string()
        }
    }
}

#[async_trait::async_trait(?Send)]
impl HttpRequest for HttpClient {
    async fn request<T>(
        &self,
        method: HttpMethod,
        url: &str,
        config: Option<RequestConfig>,
        body: Option<T>,
    ) -> RequestResult<String>
    where
        T: Serialize + Send + Sync,
    {
        let config = config.unwrap_or_default();
        let full_url = self.build_url(url);
        let full_url = self.apply_query_params(&full_url, &config);

        let mut request_builder = match method {
            HttpMethod::GET => GlooRequest::get(&full_url),
            HttpMethod::POST => GlooRequest::post(&full_url),
            HttpMethod::PUT => GlooRequest::put(&full_url),
            HttpMethod::DELETE => GlooRequest::delete(&full_url),
            HttpMethod::PATCH => GlooRequest::patch(&full_url),
        };

        // 设置credentials模式为include，确保能接收cookie
        request_builder = request_builder.credentials(web_sys::RequestCredentials::Include);

        // 添加认证头
        if let Some(token) = &self.auth_token {
            request_builder = request_builder.header("Authorization", &format!("Bearer {}", token));
        }

        // 添加 Cookie 头
        if let Some(cookie_string) = self.build_cookie_string(config.cookies.as_ref()) {
            request_builder = request_builder.header("Cookie", &cookie_string);
        }

        // 添加自定义头
        if let Some(headers) = &config.headers {
            for (key, value) in headers {
                request_builder = request_builder.header(key, value);
            }
        }

        // 添加请求体
        let request = if let Some(body_data) = body {
            let json_body = serde_json::to_string(&body_data)
                .map_err(|e| RequestError::serialization_error(format!("JSON serialization error: {}", e)))?;
            request_builder.body(json_body)
                .map_err(|e| RequestError::request_build_error(format!("Request build error: {}", e)))?
        } else {
            request_builder.build()
                .map_err(|e| RequestError::request_build_error(format!("Request build error: {}", e)))?
        };

        // 发送请求
        let response = request
            .send()
            .await
            .map_err(|e| RequestError::network_error(format!("Request failed: {}", e)))?;

        if response.status() >= 400 {
            return Err(RequestError::http_error(
                response.status(),
                response.status_text()
            ));
        }

        response
            .text()
            .await
            .map_err(|e| RequestError::response_read_error(format!("Failed to read response: {}", e)))
    }
}

/// 便捷的请求构建器
pub struct RequestBuilder {
    config: RequestConfig,
}

impl RequestBuilder {
    pub fn new() -> Self {
        Self {
            config: RequestConfig::default(),
        }
    }

    pub fn header(mut self, key: &str, value: &str) -> Self {
        if self.config.headers.is_none() {
            self.config.headers = Some(HashMap::new());
        }
        if let Some(headers) = &mut self.config.headers {
            headers.insert(key.to_string(), value.to_string());
        }
        self
    }

    pub fn query_param(mut self, key: &str, value: &str) -> Self {
        if self.config.query_params.is_none() {
            self.config.query_params = Some(HashMap::new());
        }
        if let Some(params) = &mut self.config.query_params {
            params.insert(key.to_string(), value.to_string());
        }
        self
    }

    pub fn cookie(mut self, name: &str, value: &str) -> Self {
        if self.config.cookies.is_none() {
            self.config.cookies = Some(HashMap::new());
        }
        if let Some(cookies) = &mut self.config.cookies {
            cookies.insert(name.to_string(), value.to_string());
        }
        self
    }

    pub fn cookies(mut self, cookies: HashMap<String, String>) -> Self {
        self.config.cookies = Some(cookies);
        self
    }

    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.config.timeout = Some(timeout);
        self
    }

    pub fn build(self) -> RequestConfig {
        self.config
    }
}

impl Default for RequestBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// 为 Component 使用提供的便捷函数
pub fn create_client(base_url: &str) -> HttpClient {
    HttpClient::new(base_url.to_string())
}

pub fn create_client_with_token(base_url: &str, token: &str) -> HttpClient {
    HttpClient::with_auth(base_url.to_string(), token.to_string())
}

/// 创建带有 Cookies 的客户端
pub fn create_client_with_cookies(base_url: &str, cookies: HashMap<String, String>) -> HttpClient {
    let mut client = HttpClient::new(base_url.to_string());
    client.set_cookies(cookies);
    client
}




