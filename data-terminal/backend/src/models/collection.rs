use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// CollectTask model - represents a data collection task configuration
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CollectTask {
    pub id: String,
    pub code: String, 
    pub name: String,
    pub description: String,
    pub category: CollectionCategory,
    pub collect_type: CollectType,
    pub datasource_id: String,
    pub resource_id: String,
    #[sqlx(json)]
    pub rule: CollectionRule,
    pub stage: TaskStage,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub applied_at: Option<DateTime<Utc>>,
}

/// Collection category enum
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "varchar", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum CollectionCategory {
    Database,
    Api,
    Crawler,
}

/// Collection type enum
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "varchar", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum CollectType {
    Full,
    Incremental,
}

/// Task status enum
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "varchar", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum TaskStage {
    Draft,      // User is configuring
    Applied,    // Submitted to data-engine
}

/// Collection rule variants (mode-specific configuration stored as JSON)
#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FullDatabaseRule {
    pub selected_tables: Vec<TableSelection>,
    pub transformation_sql: Option<String>,
    pub target_schema: TableSchema,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableSelection {
    pub table_name: String,
    pub selected_fields: Vec<String>, // Empty = all fields
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableSchema {
    pub table_name: String,
    pub fields: Vec<FieldSchema>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldSchema {
    pub field_name: String,
    pub field_type: String,       // SQL type (INT, VARCHAR, etc.)
    pub nullable: bool,
    pub default_value: Option<String>,
    pub primary_key: bool,
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
    pub name: Option<String>,
    pub description: Option<String>,
    pub rule: Option<CollectionRule>,
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
