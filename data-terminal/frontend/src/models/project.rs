use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Project {
    pub code: String,
    pub name: String,
    pub description: String,
    #[serde(skip_serializing, default)]
    pub logo: String,
    #[serde(skip_serializing, default)]
    pub create_status: String,
    #[serde(skip_serializing, default)]
    pub create_msg: String,
    #[serde(skip_serializing, default)]
    pub created_at: String,
    #[serde(skip_serializing, default)]
    pub updated_at: String,
}



#[derive(Debug, Clone, PartialEq)]
pub enum ProjectModalMode {
    Add,
    Edit(Project),
}