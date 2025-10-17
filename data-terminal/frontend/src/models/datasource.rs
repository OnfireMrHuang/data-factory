use serde::{Deserialize, Serialize};
use std::fmt;

// 数据源分类枚举
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum DataSourceCategory {
    Database,
    Api,
}

impl fmt::Display for DataSourceCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataSourceCategory::Database => write!(f, "Database"),
            DataSourceCategory::Api => write!(f, "Api"),
        }
    }
}

// 数据源类型枚举
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum DataSourceType {
    Mysql,
    Postgres,
    QueryApi,
    SubscribeApi,
}

impl fmt::Display for DataSourceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataSourceType::Mysql => write!(f, "Mysql"),
            DataSourceType::Postgres => write!(f, "Postgres"),
            DataSourceType::QueryApi => write!(f, "QueryApi"),
            DataSourceType::SubscribeApi => write!(f, "SubscribeApi"),
        }
    }
}

// 连接状态枚举
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ConnectionStatus {
    Connected,
    Disconnected,
    Error,
}

impl fmt::Display for ConnectionStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConnectionStatus::Connected => write!(f, "Connected"),
            ConnectionStatus::Disconnected => write!(f, "Disconnected"),
            ConnectionStatus::Error => write!(f, "Error"),
        }
    }
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
