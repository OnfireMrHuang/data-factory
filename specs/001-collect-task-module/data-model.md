# Data Model: Collect Task Module

**Feature**: `001-collect-task-module`
**Date**: 2025-10-27
**Purpose**: Define data entities, schemas, and relationships for collection task functionality

---

## Entity Relationship Diagram

```
┌─────────────────┐
│   Datasource    │
│  (existing)     │
└────────┬────────┘
         │
         │ 1
         │
         │ N
┌────────▼────────┐        ┌─────────────────┐
│  CollectTask    │────────│   Resource      │
│                 │   N:1  │   (existing)    │
└────────┬────────┘        └─────────────────┘
         │
         │ 1
         │
         │ 1
┌────────▼────────┐
│ CollectionRule  │
│  (JSON field)   │
└─────────────────┘
```

**Relationships**:
- **Datasource** (1) → (N) **CollectTask**: One datasource can be source for many collection tasks
- **Resource** (1) → (N) **CollectTask**: One resource can be target for many collection tasks
- **CollectTask** (1) → (1) **CollectionRule**: Each task has one configuration rule (stored as JSON)

---

## Core Entities

### 1. CollectTask (Primary Entity)

**Purpose**: Represents a data collection task configuration that defines how data flows from a datasource to a resource.

**Database Table**: `df_c_collection` (already exists in `migrations/v1.0.0/init_project.sql`)

#### Schema

| Field | Type | Constraints | Description |
|-------|------|-------------|-------------|
| `id` | `CHAR(36)` | PRIMARY KEY | Unique task identifier (UUID) |
| `name` | `VARCHAR(64)` | NOT NULL | User-friendly task name |
| `description` | `VARCHAR(255)` | DEFAULT '' | Task description |
| `category` | `ENUM('database', 'api', 'crawler')` | NOT NULL | Collection category (datasource type) |
| `collect_type` | `ENUM('full', 'incremental')` | NOT NULL | Collection mode |
| `datasource_id` | `CHAR(36)` | NOT NULL, FOREIGN KEY | Reference to `df_c_datasource.id` |
| `resource_id` | `CHAR(36)` | NOT NULL, FOREIGN KEY | Reference to resource table |
| `rule` | `JSON` | NOT NULL | Collection rule configuration (mode-specific) |
| `status` | `ENUM('draft', 'saved', 'applied', 'running', 'failed')` | NOT NULL, DEFAULT 'draft' | Task lifecycle status |
| `created_at` | `TIMESTAMP` | NOT NULL, DEFAULT CURRENT_TIMESTAMP | Creation timestamp |
| `updated_at` | `TIMESTAMP` | NOT NULL, DEFAULT CURRENT_TIMESTAMP ON UPDATE | Last update timestamp |
| `applied_at` | `TIMESTAMP` | NULL | Timestamp when task was applied to pipeline |

#### Indexes

```sql
CREATE INDEX idx_datasource_id ON df_c_collection(datasource_id);
CREATE INDEX idx_resource_id ON df_c_collection(resource_id);
CREATE INDEX idx_status ON df_c_collection(status);
CREATE INDEX idx_category_type ON df_c_collection(category, collect_type);
```

#### Rust Model (Backend)

```rust
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CollectTask {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: CollectionCategory,
    pub collect_type: CollectType,
    pub datasource_id: String,
    pub resource_id: String,
    #[sqlx(json)]
    pub rule: CollectionRule,
    pub status: TaskStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub applied_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "varchar", rename_all = "lowercase")]
pub enum CollectionCategory {
    Database,
    Api,
    Crawler,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "varchar", rename_all = "lowercase")]
pub enum CollectType {
    Full,
    Incremental,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "varchar", rename_all = "lowercase")]
pub enum TaskStatus {
    Draft,      // User is configuring
    Saved,      // Configuration saved but not applied
    Applied,    // Submitted to data-engine
    Running,    // Actively executing in pipeline
    Failed,     // Execution failed
}
```

#### Validation Rules

- **Name**: Required, 1-64 characters, alphanumeric + spaces/dashes
- **Category**: Must match datasource category
- **Collect Type**: Must be valid for datasource type:
  - Database datasource: Both `full` and `incremental` allowed
  - Query API datasource: Only `full` allowed
  - Subscription API datasource: Only `incremental` allowed
- **Resource Type Constraints**:
  - Full + Database → `relational_database` only
  - Full + API → `relational_database` or `file_system`
  - Incremental + Database → `queue` only
  - Incremental + API → `queue` only
- **Rule**: Must be valid JSON conforming to mode-specific schema (see CollectionRule below)
- **Status Transitions**: Draft → Saved → Applied → Running/Failed

---

### 2. CollectionRule (Nested Entity)

**Purpose**: Mode-specific configuration stored as JSON in `df_c_collection.rule` field.

**Storage**: JSON field, no separate table

#### Schema Variants

##### 2.1 Full Collection - Database

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum CollectionRule {
    FullDatabase(FullDatabaseRule),
    FullApi(FullApiRule),
    IncrementalDatabase(IncrementalDatabaseRule),
    IncrementalApi(IncrementalApiRule),
}

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
```

**Example JSON**:
```json
{
  "type": "full_database",
  "selected_tables": [
    {
      "table_name": "users",
      "selected_fields": ["id", "name", "email", "created_at"]
    },
    {
      "table_name": "orders",
      "selected_fields": []
    }
  ],
  "transformation_sql": "SELECT u.id, u.name, o.order_id FROM users u JOIN orders o ON u.id = o.user_id",
  "target_schema": {
    "table_name": "user_orders",
    "fields": [
      {
        "field_name": "id",
        "field_type": "INT",
        "nullable": false,
        "default_value": null,
        "primary_key": true,
        "auto_increment": true
      },
      {
        "field_name": "name",
        "field_type": "VARCHAR(100)",
        "nullable": false,
        "default_value": null,
        "primary_key": false,
        "auto_increment": false
      }
    ]
  }
}
```

##### 2.2 Full Collection - API

```rust
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
```

**Example JSON**:
```json
{
  "type": "full_api",
  "schedule": {
    "cron_expression": "0 0 * * *",
    "start_time": "2025-10-28T00:00:00Z",
    "end_time": null
  },
  "cursor_strategy": {
    "strategy_type": "token",
    "field_path": "pagination.next_token"
  },
  "transformation_json": "{ \"id\": data.user_id, \"name\": data.full_name }",
  "target": {
    "target_type": "table",
    "table_name": "api_users",
    "schema": {
      "table_name": "api_users",
      "fields": [...]
    }
  }
}
```

##### 2.3 Incremental Collection - Database (CDC)

```rust
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
```

**Example JSON**:
```json
{
  "type": "incremental_database",
  "cdc_config": {
    "source_tables": ["users", "orders"],
    "operations": ["insert", "update"],
    "snapshot_mode": "initial"
  },
  "filter_rules": [
    {
      "field": "status",
      "operator": "equals",
      "value": "active"
    }
  ],
  "message_transformations": [
    {
      "type": "add_field",
      "field": "processed_at",
      "value": "${current_timestamp}"
    }
  ],
  "topic_config": {
    "topic_name": "user_changes",
    "message_schema": {
      "fields": [
        {
          "field_name": "id",
          "field_type": "number",
          "required": true
        },
        {
          "field_name": "operation",
          "field_type": "string",
          "required": true
        }
      ]
    }
  }
}
```

##### 2.4 Incremental Collection - API (Webhook)

```rust
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
```

**Example JSON**:
```json
{
  "type": "incremental_api",
  "filter_rules": [
    {
      "field": "event_type",
      "operator": "in",
      "value": ["order_created", "order_shipped"]
    },
    {
      "field": "amount",
      "operator": "greater_than",
      "value": 100.0
    }
  ],
  "message_transformations": [
    {
      "type": "add_field",
      "field": "processed_at",
      "value": "${current_timestamp}"
    },
    {
      "type": "rename_field",
      "from": "evt_type",
      "to": "event_type"
    }
  ],
  "topic_config": {
    "topic_name": "order_events",
    "message_schema": {
      "fields": [...]
    }
  }
}
```

---

### 3. Datasource (Existing Entity)

**Purpose**: Represents a data source from which data is collected.

**Database Table**: `df_c_datasource` (already exists)

#### Schema (Reference)

| Field | Type | Constraints | Description |
|-------|------|-------------|-------------|
| `id` | `CHAR(36)` | PRIMARY KEY | Datasource identifier |
| `name` | `VARCHAR(64)` | NOT NULL | Datasource name |
| `description` | `VARCHAR(255)` | DEFAULT '' | Description |
| `category` | `ENUM('database', 'api')` | NOT NULL | Datasource category |
| `datasource_type` | `VARCHAR(64)` | NOT NULL | Specific type (mysql, postgres, query_api, subscribe_api) |
| `connection_config` | `JSON` | NOT NULL | Connection details |
| `connection_status` | `ENUM('connected', 'disconnected', 'error')` | NOT NULL, DEFAULT 'disconnected' | Status |
| `created_at` | `TIMESTAMP` | NOT NULL | Creation timestamp |
| `updated_at` | `TIMESTAMP` | NOT NULL | Last update timestamp |

#### Rust Model (Reference)

```rust
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Datasource {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: DatasourceCategory,
    pub datasource_type: String,
    #[sqlx(json)]
    pub connection_config: serde_json::Value,
    pub connection_status: ConnectionStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "varchar", rename_all = "lowercase")]
pub enum DatasourceCategory {
    Database,
    Api,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "varchar", rename_all = "lowercase")]
pub enum ConnectionStatus {
    Connected,
    Disconnected,
    Error,
}
```

---

### 4. Resource (Existing Entity)

**Purpose**: Represents a target destination where collected data is stored.

**Database Table**: `df_c_resource` (assumed to exist based on feature spec)

#### Schema (Assumed)

| Field | Type | Constraints | Description |
|-------|------|-------------|-------------|
| `id` | `CHAR(36)` | PRIMARY KEY | Resource identifier |
| `name` | `VARCHAR(64)` | NOT NULL | Resource name |
| `description` | `VARCHAR(255)` | DEFAULT '' | Description |
| `resource_type` | `ENUM('relational_database', 'file_system', 'queue')` | NOT NULL | Resource type |
| `connection_config` | `JSON` | NOT NULL | Connection details |
| `status` | `ENUM('active', 'inactive')` | NOT NULL, DEFAULT 'active' | Status |
| `created_at` | `TIMESTAMP` | NOT NULL | Creation timestamp |
| `updated_at` | `TIMESTAMP` | NOT NULL | Last update timestamp |

#### Rust Model (Assumed)

```rust
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Resource {
    pub id: String,
    pub name: String,
    pub description: String,
    pub resource_type: ResourceType,
    #[sqlx(json)]
    pub connection_config: serde_json::Value,
    pub status: ResourceStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "varchar", rename_all = "snake_case")]
pub enum ResourceType {
    RelationalDatabase,
    FileSystem,
    Queue,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "varchar", rename_all = "lowercase")]
pub enum ResourceStatus {
    Active,
    Inactive,
}
```

---

## Data Transfer Objects (DTOs)

### Request DTOs

#### CreateCollectTaskRequest

```rust
#[derive(Debug, Deserialize, Validate)]
pub struct CreateCollectTaskRequest {
    #[validate(length(min = 1, max = 64))]
    pub name: String,

    #[validate(length(max = 255))]
    pub description: Option<String>,

    pub category: CollectionCategory,
    pub collect_type: CollectType,

    #[validate(length(equal = 36))]
    pub datasource_id: String,

    #[validate(length(equal = 36))]
    pub resource_id: String,

    pub rule: CollectionRule,
}
```

#### UpdateCollectTaskRequest

```rust
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateCollectTaskRequest {
    #[validate(length(min = 1, max = 64))]
    pub name: Option<String>,

    #[validate(length(max = 255))]
    pub description: Option<String>,

    pub rule: Option<CollectionRule>,
}
```

#### ApplyCollectTaskRequest

```rust
#[derive(Debug, Deserialize)]
pub struct ApplyCollectTaskRequest {
    pub task_id: String,
}
```

### Response DTOs

#### CollectTaskResponse

```rust
#[derive(Debug, Serialize)]
pub struct CollectTaskResponse {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: CollectionCategory,
    pub collect_type: CollectType,
    pub datasource: DatasourceInfo,
    pub resource: ResourceInfo,
    pub rule: CollectionRule,
    pub status: TaskStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub applied_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct DatasourceInfo {
    pub id: String,
    pub name: String,
    pub datasource_type: String,
}

#[derive(Debug, Serialize)]
pub struct ResourceInfo {
    pub id: String,
    pub name: String,
    pub resource_type: ResourceType,
}
```

#### TableMetadataResponse

```rust
#[derive(Debug, Serialize)]
pub struct TableMetadataResponse {
    pub tables: Vec<TableMetadata>,
}

#[derive(Debug, Serialize)]
pub struct TableMetadata {
    pub table_name: String,
    pub table_comment: String,
    pub fields: Vec<FieldMetadata>,
}

#[derive(Debug, Serialize)]
pub struct FieldMetadata {
    pub field_name: String,
    pub field_type: String,
    pub nullable: bool,
    pub default_value: Option<String>,
    pub primary_key: bool,
}
```

#### GenerateSchemaResponse

```rust
#[derive(Debug, Serialize)]
pub struct GenerateSchemaResponse {
    pub target_schema: TableSchema,
}
```

---

## State Transitions

### CollectTask Status Lifecycle

```
┌───────┐
│ draft │  (User creates task, configuring)
└───┬───┘
    │ [User clicks "Confirm"]
    │ Validates configuration
    ▼
┌───────┐
│ saved │  (Configuration saved, not applied to pipeline)
└───┬───┘
    │ [User clicks "Apply"]
    │ Submits to data-engine via REST API
    ▼
┌─────────┐
│ applied │  (Task registered in data-engine, pending execution)
└───┬─────┘
    │ [data-engine starts execution]
    ▼
┌─────────┐                          ┌────────┐
│ running │─────[Error occurs]──────▶│ failed │
└─────────┘                          └────────┘
```

**Status Descriptions**:
- **draft**: User is actively configuring the task in the UI. Not persisted to database.
- **saved**: Configuration persisted to database but not yet submitted to data-engine.
- **applied**: Task submitted to data-engine and registered for execution.
- **running**: Task is actively executing in data-engine pipeline.
- **failed**: Task execution encountered an error.

**Editable States**: Only `draft` and `saved` tasks can be edited. `applied` and `running` tasks are read-only.

---

## Database Migrations

### Migration: Add status field to df_c_collection (if not exists)

```sql
-- Check if status field exists in df_c_collection
-- If not, add it with the following migration:

ALTER TABLE df_c_collection
ADD COLUMN IF NOT EXISTS status ENUM('draft', 'saved', 'applied', 'running', 'failed')
NOT NULL DEFAULT 'draft'
AFTER rule;

ALTER TABLE df_c_collection
ADD COLUMN IF NOT EXISTS applied_at TIMESTAMP NULL
AFTER updated_at;

-- Add index for status queries
CREATE INDEX IF NOT EXISTS idx_status ON df_c_collection(status);
```

**Note**: If the `status` and `applied_at` fields already exist in the schema (check `migrations/v1.0.0/init_project.sql`), this migration is not needed.

---

## Frontend Models

Frontend models mirror backend DTOs but are adapted for Dioxus component props:

```rust
// frontend/src/models/collection.rs

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Clone, PartialEq, Deserialize, Serialize)]
pub struct CollectTask {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: CollectionCategory,
    pub collect_type: CollectType,
    pub datasource: DatasourceInfo,
    pub resource: ResourceInfo,
    pub rule: CollectionRule,
    pub status: TaskStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub applied_at: Option<DateTime<Utc>>,
}

// ... (same enums and structs as backend, no sqlx attributes)
```

---

## Summary

### New Tables
- None (reusing existing `df_c_collection`, `df_c_datasource`, and `df_c_resource`)

### Schema Updates
- Add `status` and `applied_at` fields to `df_c_collection` (if not already present)
- Add indexes for performance optimization

### Total Entity Count
- **1 new entity**: CollectTask (using existing table)
- **4 nested types**: CollectionRule variants (JSON)
- **2 existing entities**: Datasource, Resource (read-only references)
- **15+ DTOs**: Request/Response/Metadata types

### Complexity Assessment
- **Moderate complexity**: Multiple rule variants require careful validation
- **JSON schema flexibility**: Allows extensibility but requires strict validation
- **Type safety**: Rust enums enforce valid state transitions and configurations
