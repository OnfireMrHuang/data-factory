# Data Models and APIs

## Core Data Models

**Project Model** (`backend/src/models/project.rs:25`):
```rust
pub struct Project {
    pub code: String,              // Unique project identifier
    pub name: String,              // Display name
    pub description: String,       // Project description
    pub create_status: CreateStatus, // Pending|Running|Success|Fail
    pub create_msg: String,        // Status messages
    pub logo: String,              // Project logo URL
    // Timestamps...
}
```

**Resource Model** (`backend/src/models/resource.rs:75`):
```rust
pub struct Resource {
    pub id: String,
    pub name: String,
    pub category: Category,        // RelationalDatabase|TimeSeriesDatabase|etc.
    pub resource_type: ResourceType, // Mysql|Postgres|Doris|Hdfs|Kafka|Spark|Flink
    pub config: serde_json::Value, // Infrastructure connection config
    pub status: Status,            // Active|Inactive
}
```

**DataSource Model** (`backend/src/models/datasource.rs:56`):
```rust
pub struct DataSource {
    pub id: String,
    pub name: String,
    pub category: DataSourceCategory,     // Database|Api
    pub datasource_type: DataSourceType, // Mysql|Postgres|QueryApi|SubscribeApi
    pub connection_config: serde_json::Value,
    pub connection_status: ConnectionStatus, // Connected|Disconnected|Error
}
```

## API Specifications

**Current REST Endpoints** (from service traits in `backend/src/services/mod.rs:9-34`):

```rust
// Project Management API
POST   /projects      - Create new project
PUT    /projects      - Update existing project
DELETE /projects/{code} - Delete project
GET    /projects/{code} - Get project details
GET    /projects      - List projects with pagination

// Resource Management API
POST   /resources     - Add infrastructure resource
PUT    /resources     - Update resource configuration
DELETE /resources/{id} - Remove resource
GET    /resources/{id} - Get resource details
GET    /resources     - List available resources

// DataSource Management API
POST   /datasources   - Add data source connection
PUT    /datasources   - Update data source
DELETE /datasources/{id} - Remove data source
GET    /datasources/{id} - Get data source details
GET    /datasources   - List data sources with pagination
```
