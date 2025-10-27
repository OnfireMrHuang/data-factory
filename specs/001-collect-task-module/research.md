# Research: Collect Task Module

**Feature**: `001-collect-task-module`
**Date**: 2025-10-27
**Purpose**: Resolve technical unknowns and establish design patterns for collection task implementation

## Research Items

### 1. Data-Engine Integration API

**Question**: How does data-terminal communicate with data-engine for pipeline execution?

**Decision**: REST API integration with data-engine module

**Rationale**:
- Constitution principle VI (Multi-Module Architecture) mandates inter-module communication via REST APIs only
- No direct database access across module boundaries allowed
- Each module must be independently deployable
- Versioned API contracts ensure backward compatibility

**Integration Pattern**:
```rust
// data-terminal/backend/src/utils/data_engine_client.rs
pub struct DataEngineClient {
    base_url: String,
    http_client: reqwest::Client,
}

impl DataEngineClient {
    pub async fn publish_collection_task(
        &self,
        task_config: CollectionTaskConfig,
    ) -> Result<TaskExecutionId, DataEngineError> {
        // POST /api/v1/pipeline/tasks
        // Body: { task_type: "collection", config: {...} }
    }

    pub async fn get_task_status(
        &self,
        task_id: &str,
    ) -> Result<TaskStatus, DataEngineError> {
        // GET /api/v1/pipeline/tasks/{task_id}/status
    }
}
```

**API Contract Requirements**:
- Endpoint: `POST /api/v1/pipeline/tasks` - Submit collection task for execution
- Endpoint: `GET /api/v1/pipeline/tasks/{id}/status` - Query task execution status
- Endpoint: `DELETE /api/v1/pipeline/tasks/{id}` - Cancel running task
- Request/Response format: JSON with versioned schemas
- Authentication: JWT token passed in Authorization header
- Error handling: Standard HTTP status codes + error detail in response body

**Alternatives Considered**:
- **Shared database**: Rejected because violates constitution (no cross-module DB access)
- **Message queue (async)**: Rejected because adds operational complexity and latency for this use case
- **gRPC**: Rejected because REST is simpler, more debuggable, and fits with existing data-terminal stack

**Follow-up Actions**:
- Document API contract in `contracts/data-engine-integration.yaml` (Phase 1)
- Coordinate with data-engine team to ensure endpoint availability
- Add integration tests to verify contract compliance

---

### 2. Database Collection - Table/Field Metadata Discovery

**Question**: How to retrieve table and field metadata from MySQL/PostgreSQL datasources?

**Decision**: Use SQLx with database-specific information schema queries

**Rationale**:
- SQLx already used in data-terminal backend (existing dependency)
- Information schema is standardized in MySQL and PostgreSQL
- Allows dynamic metadata discovery without hardcoding schema
- Type-safe query execution with Rust compile-time checks

**Implementation Pattern**:
```rust
// MySQL: Query INFORMATION_SCHEMA.TABLES and INFORMATION_SCHEMA.COLUMNS
pub async fn get_tables_mysql(
    pool: &MySqlPool,
) -> Result<Vec<TableMetadata>, sqlx::Error> {
    sqlx::query_as!(
        TableMetadata,
        r#"
        SELECT table_name, table_comment
        FROM information_schema.tables
        WHERE table_schema = DATABASE()
        "#
    )
    .fetch_all(pool)
    .await
}

// PostgreSQL: Query pg_catalog.pg_tables and pg_catalog.pg_attribute
pub async fn get_tables_postgres(
    pool: &PgPool,
) -> Result<Vec<TableMetadata>, sqlx::Error> {
    sqlx::query_as!(
        TableMetadata,
        r#"
        SELECT tablename as table_name, '' as table_comment
        FROM pg_catalog.pg_tables
        WHERE schemaname = 'public'
        "#
    )
    .fetch_all(pool)
    .await
}
```

**Alternatives Considered**:
- **Manual schema upload**: Rejected because requires manual maintenance and prone to staleness
- **ORM introspection**: Rejected because adds heavy dependency (Diesel, SeaORM) for simple use case
- **Direct system tables**: Rejected because information schema is more portable

---

### 3. API Collection - Query Scheduling Strategy

**Question**: How to implement periodic query scheduling for Query API datasources?

**Decision**: Store schedule configuration in database, delegate execution to data-engine

**Rationale**:
- data-terminal is responsible for configuration management only (UI + metadata)
- data-engine handles pipeline execution (scheduling, retries, monitoring)
- Separation of concerns: configuration vs. execution
- Aligns with constitution multi-module architecture principles

**Configuration Storage**:
```rust
pub struct ApiQuerySchedule {
    pub interval_seconds: Option<u32>,      // Simple interval: every N seconds
    pub cron_expression: Option<String>,    // Advanced: cron syntax (e.g., "0 0 * * *")
    pub start_time: Option<DateTime<Utc>>,  // When to start scheduling
    pub end_time: Option<DateTime<Utc>>,    // Optional: when to stop
}

pub struct CursorUpdateStrategy {
    pub strategy_type: CursorType, // Offset, Timestamp, Token
    pub field_path: String,         // JSON path to cursor field (e.g., "data.next_page_token")
}

pub enum CursorType {
    Offset,       // Numeric offset (page-based)
    Timestamp,    // Last fetched timestamp
    Token,        // Opaque continuation token
}
```

**Execution Flow**:
1. User configures schedule in data-terminal UI
2. data-terminal stores schedule config in `df_c_collection.rule` JSON field
3. User clicks "Apply" → data-terminal sends config to data-engine via REST API
4. data-engine registers scheduled job with internal scheduler (e.g., Quartz, Celery)
5. data-engine executes API queries on schedule, manages cursor state
6. data-engine writes results to target resource (database/file system)

**Alternatives Considered**:
- **In-process scheduler in data-terminal**: Rejected because violates separation of concerns
- **External scheduler (Airflow, Temporal)**: Deferred to data-engine implementation choice

---

### 4. Incremental Collection - CDC Implementation Strategy

**Question**: How to implement Change Data Capture for MySQL/PostgreSQL?

**Decision**: Delegate CDC execution to data-engine using established CDC tools

**Rationale**:
- CDC is a complex data-engine responsibility (log parsing, state management, offset tracking)
- data-terminal only manages CDC configuration (which tables, filter rules)
- Use industry-standard CDC tools: Debezium (Kafka Connect), Maxwell, pg_logical_replication

**Configuration in data-terminal**:
```rust
pub struct CdcConfig {
    pub source_tables: Vec<String>,           // Tables to capture changes from
    pub operations: Vec<CdcOperation>,        // INSERT, UPDATE, DELETE
    pub snapshot_mode: SnapshotMode,          // Initial, Never, Always
}

pub enum CdcOperation {
    Insert,
    Update,
    Delete,
}

pub enum SnapshotMode {
    Initial,  // Snapshot existing data first, then stream changes
    Never,    // Only stream changes (no initial snapshot)
    Always,   // Re-snapshot on every restart
}
```

**CDC Architecture**:
- data-engine deploys CDC connectors (Debezium) to read MySQL binlog / PostgreSQL WAL
- CDC events published to Kafka topics
- data-engine applies filter rules and transformations from `df_c_collection.rule`
- Filtered events written to target queue resource

**Alternatives Considered**:
- **Trigger-based CDC**: Rejected because requires schema modifications and has performance overhead
- **Polling-based CDC**: Rejected because high latency and misses hard deletes

---

### 5. Frontend State Management for Multi-Step Configuration

**Question**: How to manage complex state for collection task configuration wizard?

**Decision**: Use Dioxus signals with local component state, persist to backend on "Confirm"

**Rationale**:
- Dioxus signals provide reactive state management (similar to React hooks)
- Multi-step wizard has complex interdependent state (mode → datasource → resource → rules)
- Local state reduces backend round trips during configuration
- Persist to backend only on explicit user action ("Confirm" button)

**State Structure**:
```rust
#[derive(Clone, PartialEq)]
pub struct CollectionTaskState {
    pub mode: CollectionMode,                    // Full / Incremental
    pub datasource: Option<Datasource>,          // Selected datasource
    pub resource: Option<Resource>,              // Selected resource
    pub config: ConfigState,                     // Mode-specific config
}

pub enum ConfigState {
    FullDatabase {
        selected_tables: Vec<TableSelection>,
        transformation_sql: String,
        target_schema: Option<TableSchema>,
    },
    FullApi {
        schedule: ApiQuerySchedule,
        cursor_strategy: CursorUpdateStrategy,
        transformation_json: String,
        target: TargetConfig,
    },
    IncrementalDatabase {
        cdc_config: CdcConfig,
        filter_rules: Vec<FilterRule>,
        topic_config: TopicConfig,
    },
    IncrementalApi {
        filter_rules: Vec<FilterRule>,
        topic_config: TopicConfig,
    },
}
```

**Validation Strategy**:
- Validate on user input (client-side): Type constraints, required fields
- Validate on "Confirm" (server-side): SQL syntax, schema compatibility, resource availability
- Display validation errors inline using dioxus-toast

**Alternatives Considered**:
- **Global state management (Context API)**: Rejected because state is local to configuration page
- **Auto-save on every change**: Rejected because creates excessive backend requests
- **Redux-style store**: Rejected because overkill for single-page wizard state

---

### 6. SQL/JSON Transformation Editor UX

**Question**: What editor component to use for SQL/JSON transformation rules?

**Decision**: Use HTML `<textarea>` with syntax highlighting library (highlight.js or Monaco Editor)

**Rationale**:
- Textarea is simple, accessible, and works in WASM environment
- Syntax highlighting improves readability without complex editor integration
- Monaco Editor (VS Code editor) is an alternative for richer features (autocomplete, linting)

**Implementation**:
```rust
// Simple textarea approach
#[component]
pub fn TransformEditor(
    language: String,          // "sql" or "json"
    value: Signal<String>,
    on_change: EventHandler<String>,
) -> Element {
    rsx! {
        div { class: "form-control",
            label { class: "label",
                span { class: "label-text", "Transformation Logic" }
            }
            textarea {
                class: "textarea textarea-bordered font-mono",
                rows: 10,
                value: "{value}",
                oninput: move |evt| on_change.call(evt.value().clone())
            }
        }
    }
}
```

**Future Enhancement** (post-MVP):
- Integrate Monaco Editor for advanced features (autocomplete, error highlighting)
- Add "Test Transformation" button to preview results
- Add syntax validation on blur

**Alternatives Considered**:
- **CodeMirror**: Rejected because Monaco has better TypeScript/Rust support
- **Custom editor**: Rejected because reinventing the wheel

---

### 7. Target Schema Auto-Generation

**Question**: How to auto-generate target table schema from source table/fields?

**Decision**: Type mapping logic in backend service with user override capability

**Rationale**:
- Source table metadata (field types) can be queried from information schema
- Type mapping rules differ by datasource and target resource (MySQL → MySQL, MySQL → Postgres)
- User should be able to review and modify generated schema before applying

**Type Mapping Examples**:
```rust
pub fn map_mysql_to_mysql(source_type: &str) -> String {
    match source_type.to_lowercase().as_str() {
        "int" | "integer" => "INT",
        "bigint" => "BIGINT",
        "varchar" => "VARCHAR(255)",
        "text" => "TEXT",
        "datetime" => "DATETIME",
        "timestamp" => "TIMESTAMP",
        _ => "TEXT", // Safe fallback
    }
}

pub fn map_mysql_to_postgres(source_type: &str) -> String {
    match source_type.to_lowercase().as_str() {
        "int" | "integer" => "INTEGER",
        "bigint" => "BIGINT",
        "varchar" => "VARCHAR(255)",
        "text" => "TEXT",
        "datetime" | "timestamp" => "TIMESTAMP",
        _ => "TEXT",
    }
}
```

**Generated Schema Format**:
```rust
pub struct TableSchema {
    pub table_name: String,
    pub fields: Vec<FieldSchema>,
}

pub struct FieldSchema {
    pub field_name: String,
    pub field_type: String,
    pub nullable: bool,
    pub default_value: Option<String>,
    pub primary_key: bool,
}
```

**User Workflow**:
1. User selects source tables/fields
2. Backend generates target schema (POST `/api/v1/collection/generate-schema`)
3. Frontend displays schema in editable form
4. User modifies field names/types/constraints
5. User clicks "Apply" → Backend executes `CREATE TABLE` DDL on resource
6. Success → Save schema to `df_c_collection.rule`

**Alternatives Considered**:
- **Exact type copy**: Rejected because doesn't handle cross-database type differences
- **Manual schema definition**: Rejected because high friction for common case

---

### 8. Filter Rule Syntax for Incremental Collection

**Question**: What syntax/format for filter rules in CDC and webhook scenarios?

**Decision**: JSON-based predicate DSL (Domain-Specific Language)

**Rationale**:
- JSON is easy to serialize, validate, and edit in UI
- DSL provides structure for common filter operations (equals, contains, range)
- Avoids security risks of eval-ing arbitrary code
- Can be validated on backend before sending to data-engine

**Filter Rule Schema**:
```json
{
  "rules": [
    {
      "field": "user_id",
      "operator": "equals",
      "value": 12345
    },
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
  "logic": "AND"  // or "OR"
}
```

**Supported Operators**:
- `equals`, `not_equals`
- `in`, `not_in`
- `greater_than`, `less_than`, `greater_or_equal`, `less_or_equal`
- `contains`, `starts_with`, `ends_with` (string operations)
- `is_null`, `is_not_null`

**Field Transformations**:
```json
{
  "transformations": [
    {
      "type": "add_field",
      "field": "processed_at",
      "value": "${current_timestamp}"
    },
    {
      "type": "rename_field",
      "from": "old_name",
      "to": "new_name"
    },
    {
      "type": "computed_field",
      "field": "total_amount",
      "expression": "${quantity} * ${unit_price}"
    }
  ]
}
```

**Alternatives Considered**:
- **SQL WHERE clause**: Rejected because security risk (SQL injection) and not portable to webhook filters
- **JavaScript expressions**: Rejected because requires JS runtime in data-engine
- **Python expressions**: Rejected because requires Python runtime (data-engine is Java)

---

## Technology Choices Summary

| Component | Technology | Justification |
|-----------|-----------|---------------|
| Inter-module API | REST (JSON) | Constitution mandate, simplicity, debuggability |
| Metadata discovery | SQLx + Information Schema | Existing dependency, type-safe, standardized |
| State management | Dioxus signals | Native to framework, reactive, simple |
| Transformation editor | Textarea + highlight.js | WASM-compatible, accessible, upgradable |
| Filter rules | JSON DSL | Secure, portable, validatable |
| CDC implementation | Delegate to data-engine (Debezium) | Separation of concerns, production-ready tooling |
| Scheduling | Delegate to data-engine | Separation of concerns, centralized execution |

---

## Open Questions for Clarification

1. **data-engine API availability**: Does data-engine already expose `/api/v1/pipeline/tasks` endpoints, or do they need to be implemented? (Action: Coordinate with data-engine team)
2. **Authentication flow**: How does data-terminal authenticate to data-engine? Shared JWT issuer? Service account? (Action: Review auth architecture docs)
3. **Resource provisioning**: Does data-terminal need to provision target tables/files on resources, or does data-engine handle this? (Decision: data-terminal provisions via resource connection, confirmed in feature spec FR-020)
4. **CDC connector deployment**: Who deploys and manages Debezium connectors? data-engine or separate ops? (Action: Review devops docs)

---

## Best Practices

### Rust/Dioxus Best Practices
- Use `Result<T, E>` for all fallible operations (no unwrap in production)
- Use Dioxus signals for reactive state, avoid unnecessary re-renders
- Use daisy-rsx components for consistency with existing UI
- Use dioxus-toast for user-facing errors
- Use dioxus-query for async data fetching with caching

### API Design Best Practices
- Version all API endpoints (`/api/v1/...`)
- Use RESTful conventions (GET for queries, POST for creates, PUT/PATCH for updates, DELETE for deletes)
- Return standard HTTP status codes (200 OK, 201 Created, 400 Bad Request, 404 Not Found, 500 Internal Server Error)
- Include request IDs in responses for debugging
- Use pagination for list endpoints (`?page=1&limit=20`)

### Testing Best Practices
- Write tests before implementation (TDD)
- Use test fixtures for database setup/teardown
- Mock external APIs (data-engine) in unit tests
- Use integration tests for end-to-end flows
- Aim for 80%+ code coverage on services/repositories

---

## Next Steps (Phase 1)

1. Generate `data-model.md` with entity schemas and relationships
2. Generate `contracts/collection-api.yaml` with OpenAPI spec for data-terminal REST API
3. Generate `contracts/data-engine-integration.yaml` with expected data-engine API contract
4. Generate `quickstart.md` with developer setup instructions
5. Update agent context with new technologies
6. Re-evaluate Constitution Check post-design
