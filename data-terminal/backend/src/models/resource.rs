use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::models::Validator;
use crate::models::Error;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, strum::Display, strum::EnumString, sqlx::Type)]
#[strum(serialize_all = "snake_case")]
#[sqlx(rename_all = "snake_case")]
pub enum Category {
    Database,
    Filesystem,
    Queue,
    BatchCompute,
    StreamCompute,
    VectorDatabase,
}

impl Default for Category {
    fn default() -> Self {
        Self::Database
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

