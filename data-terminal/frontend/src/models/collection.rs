use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize};

/// Custom deserializer for DateTime fields from backend
/// Backend sends dates in format "2025-11-03 10:13:27"
mod datetime_format {
    use super::*;

    const FORMAT: &str = "%Y-%m-%d %H:%M:%S";

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NaiveDateTime::parse_from_str(&s, FORMAT)
            .map_err(serde::de::Error::custom)
            .map(|dt| DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc))
    }
}

mod optional_datetime_format {
    use super::*;

    const FORMAT: &str = "%Y-%m-%d %H:%M:%S";

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let opt = Option::<String>::deserialize(deserializer)?;
        match opt {
            Some(s) => {
                NaiveDateTime::parse_from_str(&s, FORMAT)
                    .map_err(serde::de::Error::custom)
                    .map(|dt| Some(DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc)))
            }
            None => Ok(None),
        }
    }
}

/// CollectTask model for frontend
#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct CollectTask {
    pub id: String,
    pub code: String,
    pub name: String,
    pub description: String,
    pub category: CollectionCategory,
    pub collect_type: CollectType,
    pub datasource_id: String,
    pub datasource_name: String,
    pub queue_resource_id: String,
    pub queue_resource_name: String,
    pub database_resource_id: String,
    pub database_resource_name: String,
    pub rule: serde_json::Value,
    pub stage: TaskStage,
    #[serde(deserialize_with = "datetime_format::deserialize")]
    pub created_at: DateTime<Utc>,
    #[serde(deserialize_with = "datetime_format::deserialize")]
    pub updated_at: DateTime<Utc>,
    #[serde(deserialize_with = "optional_datetime_format::deserialize")]
    pub applied_at: Option<DateTime<Utc>>,
}

#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum CollectionCategory {
    Database,
    Api,
    Crawler,
}

impl CollectionCategory {

    pub fn matches_datasource_category(&self, category: &crate::models::datasource::DataSourceCategory) -> bool {
        match (self, category) {
            (CollectionCategory::Database, crate::models::datasource::DataSourceCategory::Database) => true,
            (CollectionCategory::Api, crate::models::datasource::DataSourceCategory::Api) => true,
            _ => false,
        }
    }
}

#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum CollectType {
    Full,
    Incremental,
}

impl CollectType {
    pub fn matches_resource_category(&self, datasource_type: &crate::models::resource::Category) -> bool {
        match (self, datasource_type) {
            (CollectType::Full, crate::models::resource::Category::RelationalDatabase) => true,
            (CollectType::Full, crate::models::resource::Category::TimeSeriesDatabase) => true,
            (CollectType::Full, crate::models::resource::Category::DocumentDatabase) => true,
            (CollectType::Full, crate::models::resource::Category::VectorDatabase) => true,
            (CollectType::Full, crate::models::resource::Category::Filesystem) => true,
            (CollectType::Incremental, crate::models::resource::Category::Queue) => true,
            _ => false,
        }
    }
}



#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum TaskStage {
    Draft,
    Applied,
}

/// Collection rule variants
#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum CollectionRule {
    FullDatabase(FullDatabaseRule),
    FullApi(FullApiRule),
    IncrementalDatabase(IncrementalDatabaseRule),
    IncrementalApi(IncrementalApiRule),
}

// ============================================================================
// Full Collection - Database
// ============================================================================

#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct FullDatabaseRule {
    pub selected_tables: Vec<TableSelection>,
    pub transformation_sql: Option<String>,
    pub target_schema: TableSchema,
}

#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct TableSelection {
    pub table_name: String,
    pub selected_fields: Vec<String>,
}

#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct TableSchema {
    pub table_name: String,
    pub fields: Vec<FieldSchema>,
}

#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct FieldSchema {
    pub field_name: String,
    pub field_type: String,
    pub nullable: bool,
    pub default_value: Option<String>,
    pub primary_key: bool,
    pub auto_increment: bool,
}

// ============================================================================
// Full Collection - API
// ============================================================================

#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct FullApiRule {
    pub schedule: ApiQuerySchedule,
    pub cursor_strategy: Option<CursorUpdateStrategy>,
    pub transformation_json: Option<String>,
    pub target: TargetConfig,
}

#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct ApiQuerySchedule {
    pub interval_seconds: Option<u32>,
    pub cron_expression: Option<String>,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
}

#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct CursorUpdateStrategy {
    pub strategy_type: CursorType,
    pub field_path: String,
}

#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum CursorType {
    Offset,
    Timestamp,
    Token,
}

#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
#[serde(tag = "target_type", rename_all = "snake_case")]
pub enum TargetConfig {
    Table {
        table_name: String,
        schema: TableSchema,
    },
    File {
        file_path: String,
        file_format: FileFormat,
    },
}

#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum FileFormat {
    Json,
    Csv,
    Parquet,
}

// ============================================================================
// Incremental Collection - Database (CDC)
// ============================================================================

#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct IncrementalDatabaseRule {
    pub cdc_config: CdcConfig,
    pub filter_rules: Vec<FilterRule>,
    pub message_transformations: Vec<FieldTransformation>,
    pub topic_config: TopicConfig,
}

#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct CdcConfig {
    pub source_tables: Vec<String>,
    pub operations: Vec<CdcOperation>,
    pub snapshot_mode: SnapshotMode,
}

#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum CdcOperation {
    Insert,
    Update,
    Delete,
}

#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum SnapshotMode {
    Initial,
    Never,
    Always,
}

#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct TopicConfig {
    pub topic_name: String,
    pub message_schema: MessageSchema,
}

#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct MessageSchema {
    pub fields: Vec<MessageField>,
}

#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct MessageField {
    pub field_name: String,
    pub field_type: String,
    pub required: bool,
}

// ============================================================================
// Incremental Collection - API (Webhook)
// ============================================================================

#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct IncrementalApiRule {
    pub filter_rules: Vec<FilterRule>,
    pub message_transformations: Vec<FieldTransformation>,
    pub topic_config: TopicConfig,
}

#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct FilterRule {
    pub field: String,
    pub operator: FilterOperator,
    pub value: serde_json::Value,
}

#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum FilterOperator {
    Equals,
    NotEquals,
    In,
    NotIn,
    GreaterThan,
    LessThan,
    GreaterOrEqual,
    LessOrEqual,
    Contains,
    StartsWith,
    EndsWith,
    IsNull,
    IsNotNull,
}

#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum FieldTransformation {
    AddField {
        field: String,
        value: String,
    },
    RenameField {
        from: String,
        to: String,
    },
    ComputedField {
        field: String,
        expression: String,
    },
    RemoveField {
        field: String,
    },
}

// ============================================================================
// DTOs for API communication
// ============================================================================

#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct DatasourceInfo {
    pub id: String,
    pub name: String,
    pub datasource_type: String,
}

#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct ResourceInfo {
    pub id: String,
    pub name: String,
    pub resource_type: String,
}

#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct CreateCollectTaskRequest {
    pub name: String,
    pub description: Option<String>,
    pub category: CollectionCategory,
    pub collect_type: CollectType,
    pub datasource_id: String,
    pub queue_resource_id: String,
    pub database_resource_id: String,
    pub rule: serde_json::Value,
}

#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct UpdateCollectTaskRequest {
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub rule: serde_json::Value,
}

#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct TableMetadata {
    pub table_name: String,
    pub table_comment: String,
    pub fields: Vec<FieldMetadata>,
}

#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct FieldMetadata {
    pub field_name: String,
    pub field_type: String,
    pub nullable: bool,
    pub default_value: Option<String>,
    pub primary_key: bool,
}


#[derive(Clone, PartialEq)]
pub struct IncrementalRuleForm {
    pub select_sql: String,
    pub monitor_table: String,
    pub monitor_sql: String,
    pub use_custom_monitor_table: bool,
}

impl Default for IncrementalRuleForm {
    fn default() -> Self {
        Self {
            select_sql: String::new(),
            monitor_table: String::new(),
            monitor_sql: String::new(),
            use_custom_monitor_table: false,
        }
    }
}