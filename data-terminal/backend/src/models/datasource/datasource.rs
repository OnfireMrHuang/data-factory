use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::models::Validator;
use crate::models::Error;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, strum::Display, strum::EnumString, sqlx::Type)]
#[strum(serialize_all = "snake_case")]
#[sqlx(rename_all = "snake_case")]
pub enum DataSourceCategory {
    Database,
    Api,
}

impl Default for DataSourceCategory {
    fn default() -> Self {
        Self::Database
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, strum::Display, strum::EnumString, sqlx::Type)]
#[strum(serialize_all = "snake_case")]
#[sqlx(rename_all = "snake_case")]
pub enum DataSourceType {
    Mysql,
    Postgres,
    QueryApi,
    SubscribeApi,
}

impl Default for DataSourceType {
    fn default() -> Self {
        Self::Mysql
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, strum::Display, strum::EnumString, sqlx::Type)]
#[strum(serialize_all = "snake_case")]
#[sqlx(rename_all = "snake_case")]
pub enum ConnectionStatus {
    Connected,
    Disconnected,
    Error,
}

impl Default for ConnectionStatus {
    fn default() -> Self {
        Self::Disconnected
    }
}


// 内部使用的完整 DataSource 模型
#[derive(Debug, Serialize, Deserialize, FromRow, Default, Clone)]
pub struct DataSource {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub category: DataSourceCategory,
    #[serde(default)]
    pub datasource_type: DataSourceType,
    #[sqlx(json)]
    pub connection_config: serde_json::Value,
    #[serde(default)]
    pub connection_status: ConnectionStatus,
    #[serde(default)]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(default)]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl Validator for DataSource {
    fn validate(&self) -> Result<(), Error> {
        if self.id.is_empty() {
            return Err(Error::EmptyValue("id".to_string()));
        }
        if self.name.is_empty() {
            return Err(Error::EmptyValue("name".to_string()));
        }
        if self.name.len() > 64 {
            return Err(Error::InvalidValue("name length must be less than 64 characters".to_string()));
        }

        if self.description.len() > 255 {
            return Err(Error::InvalidValue("description length must be less than 255 characters".to_string()));
        }
        Ok(())
    }
}

// Web 对外接口使用的只读 DataSource 模型
#[derive(Debug, Serialize, Deserialize, FromRow, Default, Clone)]
pub struct DataSourceReadOnly {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: DataSourceCategory,
    pub datasource_type: DataSourceType,
    pub connection_config: serde_json::Value,
    pub connection_status: ConnectionStatus,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

// 用于创建和更新的 DataSource 模型（不包含 status 和 connection_status 字段）
#[derive(Debug, Serialize, Deserialize, FromRow, Default, Clone)]
pub struct DataSourceCreateUpdate {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: DataSourceCategory,
    pub datasource_type: DataSourceType,
    pub connection_config: serde_json::Value,
}

impl From<DataSourceCreateUpdate> for DataSource {
    fn from(datasource: DataSourceCreateUpdate) -> Self {
        Self {
            id: datasource.id,
            name: datasource.name,
            description: datasource.description,
            category: datasource.category,
            datasource_type: datasource.datasource_type,
            connection_config: datasource.connection_config,
            connection_status: ConnectionStatus::Disconnected,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }
}

impl From<DataSource> for DataSourceReadOnly {
    fn from(datasource: DataSource) -> Self {
        Self {
            id: datasource.id,
            name: datasource.name,
            description: datasource.description,
            category: datasource.category,
            datasource_type: datasource.datasource_type,
            connection_config: datasource.connection_config,
            connection_status: datasource.connection_status,
            created_at: datasource.created_at,
            updated_at: datasource.updated_at,
        }
    }
}
