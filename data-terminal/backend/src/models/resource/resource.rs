use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::models::Validator;
use crate::models::Error;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, strum::Display, strum::EnumString, sqlx::Type)]
#[strum(serialize_all = "snake_case")]
#[sqlx(rename_all = "snake_case")]
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

impl Default for Category {
    fn default() -> Self {
        Self::RelationalDatabase
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, strum::Display, strum::EnumString, sqlx::Type)]
#[strum(serialize_all = "snake_case")]
#[sqlx(rename_all = "snake_case")]
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

impl Default for ResourceType {
    fn default() -> Self {
        Self::Mysql
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, strum::Display, strum::EnumString, sqlx::Type)]
#[strum(serialize_all = "snake_case")]
#[sqlx(rename_all = "snake_case")]
pub enum Status {
    Active,
    Inactive,
}

impl Default for Status {
    fn default() -> Self {
        Self::Active
    }
}

// 内部使用的完整 Resource 模型
#[derive(Debug, Serialize, Deserialize, FromRow, Default, Clone)]
pub struct Resource {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub category: Category,
    #[serde(default)]
    pub resource_type: ResourceType,
    #[sqlx(json)]
    pub config: serde_json::Value,
    #[serde(default)]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(default)]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[serde(default)]
    pub status: Status,
}

impl Validator for Resource {
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



// Web 对外接口使用的只读 Resource 模型
#[derive(Debug, Serialize, Deserialize, FromRow, Default, Clone)]
pub struct ResourceReadOnly {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: Category,
    pub resource_type: ResourceType,
    pub config: serde_json::Value,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub status: Status,
}

// 用于创建和更新的 Resource 模型（不包含 status 字段）
#[derive(Debug, Serialize, Deserialize, FromRow, Default, Clone)]
pub struct ResourceCreateUpdate {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: Category,
    pub resource_type: ResourceType,
    pub config: serde_json::Value,
}

impl From<ResourceCreateUpdate> for Resource {
    fn from(resource: ResourceCreateUpdate) -> Self {
        Self {
            id: resource.id,
            name: resource.name,
            description: resource.description,
            category: resource.category,
            resource_type: resource.resource_type,
            config: resource.config,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            status: Status::Active,
        }
    }
}

impl From<Resource> for ResourceReadOnly {
    fn from(resource: Resource) -> Self {
        Self {
            id: resource.id,
            name: resource.name,
            description: resource.description,
            category: resource.category,
            resource_type: resource.resource_type,
            config: resource.config,
            created_at: resource.created_at,
            updated_at: resource.updated_at,
            status: resource.status,
        }
    }
}

