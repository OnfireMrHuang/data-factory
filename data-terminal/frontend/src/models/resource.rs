
use serde::{Deserialize, Serialize};

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
