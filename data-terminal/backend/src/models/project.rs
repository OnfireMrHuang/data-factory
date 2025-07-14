
use serde::{Deserialize, Serialize};
use sqlx::FromRow;


use crate::models::Validator;
use crate::models::Error;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, strum::Display, strum::EnumString, sqlx::Type)]
#[strum(serialize_all = "snake_case")]
#[sqlx(rename_all = "snake_case")]
pub enum CreateStatus {
    Pending,
    Running,
    Success,
    Fail,
}

impl Default for CreateStatus {
    fn default() -> Self {
        Self::Pending  
    }
}


#[derive(Debug, Serialize, Deserialize, FromRow, Default, Clone)]
pub struct Project {
    pub code: String,
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub create_status: CreateStatus,
    #[serde(default)]
    pub create_msg: String,
    #[serde(default)]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(default)]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl Validator for Project {
    fn validate(&self) -> Result<(), Error> {
        if self.code.is_empty() {
            return Err(Error::EmptyValue("code".to_string()));
        }
        if self.name.is_empty() {
            return Err(Error::EmptyValue("name".to_string()));
        }
        Ok(())
    }
}
