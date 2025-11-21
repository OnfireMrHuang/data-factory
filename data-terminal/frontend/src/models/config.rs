use serde::{Deserialize, Serialize};


// 数据库配置表单
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfigForm {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub databases: String,
}

// 队列配置表单
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct QueueConfigForm {
    pub host: String,
    pub port: u16,
    pub admin_port: u16,
    pub username: Option<String>,
    pub password: Option<String>,
    pub virtual_host: Option<String>,
    pub cluster_name: Option<String>,
    pub ssl_enabled: bool,
    pub sasl_enabled: bool,
    pub sasl_mechanism: Option<String>,
    pub stream_key: Option<String>,
    pub consumer_group: Option<String>,
    pub consumer_name: Option<String>,
    pub auto_ack: bool,
    pub read_batch_size: Option<u32>,
    pub read_block_ms: Option<u64>,
    pub database: Option<u8>,
}

impl Default for QueueConfigForm {
    fn default() -> Self {
        Self {
            host: String::new(),
            port: 9092,
            admin_port: 8080,
            username: None,
            password: None,
            virtual_host: None,
            cluster_name: None,
            ssl_enabled: false,
            sasl_enabled: false,
            sasl_mechanism: None,
            stream_key: None,
            consumer_group: None,
            consumer_name: None,
            auto_ack: true,
            read_batch_size: Some(100),
            read_block_ms: Some(1000),
            database: Some(0),
        }
    }
}

// 文件系统配置表单
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileSystemConfigForm {
    pub host: String,
    pub port: u16,
    pub username: Option<String>,
    pub password: Option<String>,
    pub ssl_enabled: bool,
    pub auth_token: Option<String>,
    pub access_key_id: Option<String>,
    pub secret_access_key: Option<String>,
    pub region: Option<String>,
    pub bucket: Option<String>,
}

// 向量数据库配置表单
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorDatabaseConfigForm {
    pub host: String,
    pub port: u16,
    pub username: Option<String>,
    pub password: Option<String>,
    pub ssl_enabled: bool,
    pub collection_name: Option<String>,
    pub dimension: Option<u32>,
    pub metric_type: Option<String>,
}

// 批处理计算配置表单
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchComputeConfigForm {
    pub host: String,
    pub port: u16,
    pub username: Option<String>,
    pub password: Option<String>,
    pub ssl_enabled: bool,
    pub cluster_name: Option<String>,
    pub master_url: Option<String>,
    pub worker_nodes: Option<u32>,
}

// 流处理计算配置表单
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamComputeConfigForm {
    pub host: String,
    pub port: u16,
    pub username: Option<String>,
    pub password: Option<String>,
    pub ssl_enabled: bool,
    pub cluster_name: Option<String>,
    pub job_manager_url: Option<String>,
    pub task_manager_count: Option<u32>,
}


// API配置表单
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConfigForm {
    pub url: String,
    pub method: String,
    pub headers: serde_json::Value,
    pub params: serde_json::Value,
    pub body: Option<String>,
    pub timeout: u64,
    pub auth_type: Option<String>,
    pub auth_token: Option<String>,
}

// 订阅API配置表单
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscribeApiConfigForm {
    pub url: String,
    pub method: String,
    pub headers: serde_json::Value,
    pub params: serde_json::Value,
    pub body: Option<String>,
    pub timeout: u64,
    pub auth_type: Option<String>,
    pub auth_token: Option<String>,
    pub polling_interval: u64,
    pub max_retries: u32,
}


