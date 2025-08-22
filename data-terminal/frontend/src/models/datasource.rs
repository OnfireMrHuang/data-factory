use serde::{Deserialize, Serialize};

// 数据源分类枚举
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DataSourceCategory {
    Database,
    Api,
}

// 数据源类型枚举
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DataSourceType {
    Mysql,
    Postgres,
    QueryApi,
    SubscribeApi,
}

// 连接状态枚举
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ConnectionStatus {
    Connected,
    Disconnected,
    Error,
}

// 数据源结构体
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DataSource {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: DataSourceCategory,
    pub datasource_type: DataSourceType,
    pub connection_config: serde_json::Value,
    pub connection_status: ConnectionStatus,
    pub created_at: String,
    pub updated_at: String,
}

// 创建/更新数据源结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSourceCreateUpdate {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: DataSourceCategory,
    pub datasource_type: DataSourceType,
    pub connection_config: serde_json::Value,
}

// 数据源创建表单数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSourceFormData {
    pub name: String,
    pub description: String,
    pub category: DataSourceCategory,
    pub datasource_type: DataSourceType,
    pub connection_config: serde_json::Value,
}
