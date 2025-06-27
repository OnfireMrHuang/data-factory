
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[derive(Clone, Copy, PartialEq, Eq, strum::Display, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
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


#[derive(Debug, Serialize, Deserialize)]
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
    pub created_at: String,
    #[serde(default)]
    pub updated_at: String,
}