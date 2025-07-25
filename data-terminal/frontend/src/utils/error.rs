use thiserror::Error;

/// HTTP 请求错误类型
#[derive(Error, Debug, Clone)]
pub enum RequestError {
    #[error("网络请求失败: {0}")]
    NetworkError(String),

    #[error("HTTP 错误: {status} - {message}")]
    HttpError {
        status: u16,
        message: String,
    },

    #[error("JSON 序列化错误: {0}")]
    SerializationError(String),

    #[error("JSON 反序列化错误: {0}")]
    DeserializationError(String),

    #[error("请求构建错误: {0}")]
    RequestBuildError(String),

    #[error("响应读取错误: {0}")]
    ResponseReadError(String),

    #[error("超时错误")]
    TimeoutError,

    #[error("认证错误: {0}")]
    AuthenticationError(String),

    #[error("配置错误: {0}")]
    ConfigError(String),

    #[error("未知错误: {0}")]
    Unknown(String),
}

impl RequestError {
    /// 创建网络错误
    pub fn network_error(message: impl Into<String>) -> Self {
        RequestError::NetworkError(message.into())
    }

    /// 创建 HTTP 错误
    pub fn http_error(status: u16, message: impl Into<String>) -> Self {
        RequestError::HttpError {
            status,
            message: message.into(),
        }
    }

    /// 创建序列化错误
    pub fn serialization_error(message: impl Into<String>) -> Self {
        RequestError::SerializationError(message.into())
    }

    /// 创建反序列化错误
    pub fn deserialization_error(message: impl Into<String>) -> Self {
        RequestError::DeserializationError(message.into())
    }

    /// 创建请求构建错误
    pub fn request_build_error(message: impl Into<String>) -> Self {
        RequestError::RequestBuildError(message.into())
    }

    /// 创建响应读取错误
    pub fn response_read_error(message: impl Into<String>) -> Self {
        RequestError::ResponseReadError(message.into())
    }

    /// 创建认证错误
    pub fn authentication_error(message: impl Into<String>) -> Self {
        RequestError::AuthenticationError(message.into())
    }

    /// 创建配置错误
    pub fn config_error(message: impl Into<String>) -> Self {
        RequestError::ConfigError(message.into())
    }

    /// 创建未知错误
    pub fn unknown(message: impl Into<String>) -> Self {
        RequestError::Unknown(message.into())
    }
}

/// 类型别名，简化错误处理
pub type RequestResult<T> = Result<T, RequestError>; 