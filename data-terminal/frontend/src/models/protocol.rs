use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub result: bool,
    pub msg: String,
    pub data: T,
}

// 资源分类枚举
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Category {
    RelationalDatabase,
    TimeSeriesDatabase,
    DocumentDatabase,
    VectorDatabase,
    GraphDatabase,
    KVDatabase,
    Filesystem, 
    Queue,
    BatchCompute,
    StreamCompute,
}

// 资源类型枚举
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ResourceType {
    Mysql,
    Postgres,
    Doris,
    Hdfs,
    Kafka,
    Spark,
    Flink,
    Mailvus
}

// 资源状态枚举
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Status {
    Active,
    Inactive,
}

// 资源结构体
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Resource {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: Category,
    pub resource_type: ResourceType,
    pub config: serde_json::Value,
    pub created_at: String,
    pub updated_at: String,
    pub status: Status,
}

// 创建/更新资源结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceCreateUpdate {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: Category,
    pub resource_type: ResourceType,
    pub config: serde_json::Value,
}

// 资源创建表单数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceFormData {
    pub name: String,
    pub description: String,
    pub category: Category,
    pub resource_type: ResourceType,
    pub config: serde_json::Value,
}

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