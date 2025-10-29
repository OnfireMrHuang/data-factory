use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// CollectTask model for frontend
#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct CollectTask {
    pub id: String,
    pub code: String,
    pub name: String,
    pub description: String,
    pub category: CollectionCategory,
    pub collect_type: CollectType,
    pub datasource: DatasourceInfo,
    pub resource: ResourceInfo,
    pub rule: CollectionRule,
    pub stage: TaskStage,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub applied_at: Option<DateTime<Utc>>,
}

#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum CollectionCategory {
    Database,
    Api,
    Crawler,
}

#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum CollectType {
    Full,
    Incremental,
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
    pub resource_id: String,
    pub rule: CollectionRule,
}

#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct UpdateCollectTaskRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub rule: Option<CollectionRule>,
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
