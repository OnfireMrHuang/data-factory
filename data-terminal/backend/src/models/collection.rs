use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use crate::models::{Validator, Error};
use crate::impl_sqlx_for_string_enum;

/// CollectTask model - represents a data collection task configuration (Internal use)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Default)]
pub struct CollectTask {
    pub id: String,
    pub code: String,
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub category: CollectionCategory,
    #[serde(default)]
    pub collect_type: CollectType,
    pub datasource_id: String,
    pub resource_id: String,
    #[sqlx(json)]
    pub rule: CollectionRule,
    #[serde(default)]
    pub stage: TaskStage,
    #[serde(default)]
    pub created_at: DateTime<Utc>,
    #[serde(default)]
    pub updated_at: DateTime<Utc>,
    pub applied_at: Option<DateTime<Utc>>,
}

impl Validator for CollectTask {
    fn validate(&self) -> Result<(), Error> {
        if self.id.is_empty() {
            return Err(Error::EmptyValue("id".to_string()));
        }
        if self.code.is_empty() {
            return Err(Error::EmptyValue("code".to_string()));
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
        if self.datasource_id.is_empty() {
            return Err(Error::EmptyValue("datasource_id".to_string()));
        }
        if self.resource_id.is_empty() {
            return Err(Error::EmptyValue("resource_id".to_string()));
        }
        Ok(())
    }
}

/// Collection category enum
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, strum::Display, strum::EnumString)]
#[strum(serialize_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum CollectionCategory {
    Database,
    Api,
    Crawler,
}

impl Default for CollectionCategory {
    fn default() -> Self {
        Self::Database
    }
}

impl_sqlx_for_string_enum!(CollectionCategory);

/// Collection type enum
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, strum::Display, strum::EnumString)]
#[strum(serialize_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum CollectType {
    Full,
    Incremental,
}

impl Default for CollectType {
    fn default() -> Self {
        Self::Full
    }
}

impl_sqlx_for_string_enum!(CollectType);

/// Task status enum
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, strum::Display, strum::EnumString)]
#[strum(serialize_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum TaskStage {
    Draft,      // User is configuring
    Applied,    // Submitted to data-engine
}

impl Default for TaskStage {
    fn default() -> Self {
        Self::Draft
    }
}

impl_sqlx_for_string_enum!(TaskStage);

/// Collection rule variants (mode-specific configuration stored as JSON)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum CollectionRule {
    FullDatabase(FullDatabaseRule),
    FullApi(FullApiRule),
    IncrementalDatabase(IncrementalDatabaseRule),
    IncrementalApi(IncrementalApiRule),
}

impl Default for CollectionRule {
    fn default() -> Self {
        Self::FullDatabase(FullDatabaseRule::default())
    }
}

// ============================================================================
// Full Collection - Database
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FullDatabaseRule {
    #[serde(default)]
    pub selected_tables: Vec<TableSelection>,
    pub transformation_sql: Option<String>,
    #[serde(default)]
    pub target_schema: TableSchema,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TableSelection {
    #[serde(default)]
    pub table_name: String,
    #[serde(default)]
    pub selected_fields: Vec<String>, // Empty = all fields
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TableSchema {
    #[serde(default)]
    pub table_name: String,
    #[serde(default)]
    pub fields: Vec<FieldSchema>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FieldSchema {
    #[serde(default)]
    pub field_name: String,
    #[serde(default)]
    pub field_type: String,       // SQL type (INT, VARCHAR, etc.)
    #[serde(default)]
    pub nullable: bool,
    pub default_value: Option<String>,
    #[serde(default)]
    pub primary_key: bool,
    #[serde(default)]
    pub auto_increment: bool,
}

// ============================================================================
// Full Collection - API
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FullApiRule {
    pub schedule: ApiQuerySchedule,
    pub cursor_strategy: Option<CursorUpdateStrategy>,
    pub transformation_json: Option<String>,  // JSONata or jq expression
    pub target: TargetConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiQuerySchedule {
    pub interval_seconds: Option<u32>,      // Simple interval
    pub cron_expression: Option<String>,    // Cron syntax
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CursorUpdateStrategy {
    pub strategy_type: CursorType,
    pub field_path: String,  // JSON path (e.g., "data.next_page_token")
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CursorType {
    Offset,       // Numeric offset
    Timestamp,    // Last fetched timestamp
    Token,        // Opaque continuation token
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FileFormat {
    Json,
    Csv,
    Parquet,
}

// ============================================================================
// Incremental Collection - Database (CDC)
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncrementalDatabaseRule {
    pub cdc_config: CdcConfig,
    pub filter_rules: Vec<FilterRule>,
    pub message_transformations: Vec<FieldTransformation>,
    pub topic_config: TopicConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CdcConfig {
    pub source_tables: Vec<String>,
    pub operations: Vec<CdcOperation>,
    pub snapshot_mode: SnapshotMode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CdcOperation {
    Insert,
    Update,
    Delete,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SnapshotMode {
    Initial,
    Never,
    Always,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopicConfig {
    pub topic_name: String,
    pub message_schema: MessageSchema,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageSchema {
    pub fields: Vec<MessageField>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageField {
    pub field_name: String,
    pub field_type: String,  // "string", "number", "boolean", "object", "array"
    pub required: bool,
}

// ============================================================================
// Incremental Collection - API (Webhook)
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncrementalApiRule {
    pub filter_rules: Vec<FilterRule>,
    pub message_transformations: Vec<FieldTransformation>,
    pub topic_config: TopicConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterRule {
    pub field: String,
    pub operator: FilterOperator,
    pub value: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum FieldTransformation {
    AddField {
        field: String,
        value: String,  // Expression or literal
    },
    RenameField {
        from: String,
        to: String,
    },
    ComputedField {
        field: String,
        expression: String,  // Simple expression syntax
    },
    RemoveField {
        field: String,
    },
}

// ============================================================================
// DTOs (Data Transfer Objects)
// ============================================================================

/// Request DTO for creating a collection task
#[derive(Debug, Deserialize, Serialize)]
pub struct CreateCollectTaskRequest {
    pub name: String,
    pub description: Option<String>,
    pub category: CollectionCategory,
    pub collect_type: CollectType,
    pub datasource_id: String,
    pub resource_id: String,
    pub rule: CollectionRule,
}

/// Request DTO for updating a collection task
#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateCollectTaskRequest {
    pub id: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub rule: Option<CollectionRule>,
}

/// ReadOnly DTO for collection task with formatted timestamps (API response)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CollectTaskReadOnly {
    pub id: String,
    pub code: String,
    pub name: String,
    pub description: String,
    pub category: CollectionCategory,
    pub collect_type: CollectType,
    pub datasource_id: String,
    pub resource_id: String,
    pub rule: CollectionRule,
    pub stage: TaskStage,
    pub created_at: String,
    pub updated_at: String,
    pub applied_at: Option<String>,
}

impl From<CollectTask> for CollectTaskReadOnly {
    fn from(task: CollectTask) -> Self {
        Self {
            id: task.id,
            code: task.code,
            name: task.name,
            description: task.description,
            category: task.category,
            collect_type: task.collect_type,
            datasource_id: task.datasource_id,
            resource_id: task.resource_id,
            rule: task.rule,
            stage: task.stage,
            created_at: task.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
            updated_at: task.updated_at.format("%Y-%m-%d %H:%M:%S").to_string(),
            applied_at: task.applied_at.map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string()),
        }
    }
}

impl From<CreateCollectTaskRequest> for CollectTask {
    fn from(request: CreateCollectTaskRequest) -> Self {
        Self {
            id: String::new(), // Will be set by service
            code: String::new(), // Will be set by service
            name: request.name,
            description: request.description.unwrap_or_default(),
            category: request.category,
            collect_type: request.collect_type,
            datasource_id: request.datasource_id,
            resource_id: request.resource_id,
            rule: request.rule,
            stage: TaskStage::Draft,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            applied_at: None,
        }
    }
}

/// Response DTO for collection task with nested datasource/resource info
#[derive(Debug, Serialize)]
pub struct CollectTaskResponse {
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

#[derive(Debug, Serialize, Deserialize)]
pub struct DatasourceInfo {
    pub id: String,
    pub name: String,
    pub datasource_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceInfo {
    pub id: String,
    pub name: String,
    pub resource_type: String,
}

/// Response DTO for table metadata
#[derive(Debug, Serialize)]
pub struct TableMetadataResponse {
    pub tables: Vec<TableMetadata>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TableMetadata {
    pub table_name: String,
    pub table_comment: String,
    pub fields: Vec<FieldMetadata>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FieldMetadata {
    pub field_name: String,
    pub field_type: String,
    pub nullable: bool,
    pub default_value: Option<String>,
    pub primary_key: bool,
}

/// Response DTO for schema generation
#[derive(Debug, Serialize)]
pub struct GenerateSchemaResponse {
    pub target_schema: TableSchema,
}
