# Data-Terminal Brownfield Architecture Document

## Introduction

This document captures the **CURRENT STATE** of the Data-Terminal component within the data-factory platform, including technical debt, workarounds, and real-world patterns. It serves as a reference for AI agents working on bug fixes and generating data node code for ETL pipelines.

### Document Scope

**Focused on data-terminal component** - the Rust-based full-stack application serving as the data management interface for the broader data-factory platform. This analysis is targeted at senior developers working on:
- Bug fixes in the data management interface
- AI-generated data node code for ETL pipeline integration
- Extension of current CRUD operations to support workflow orchestration

### Change Log

| Date   | Version | Description                 | Author    |
| ------ | ------- | --------------------------- | --------- |
| 2025-09-16 | 1.0     | Initial brownfield analysis | Winston (AI Architect) |

## Quick Reference - Key Files and Entry Points

### Critical Files for Understanding the System

**Backend (Rust + Axum):**
- **Main Entry**: `data-terminal/backend/src/main.rs` - Application bootstrap
- **Dependency Injection**: `data-terminal/backend/src/autofac.rs` - Global state management with Shaku
- **Core Business Logic**: `data-terminal/backend/src/services/` - Service layer implementations
- **Data Models**: `data-terminal/backend/src/models/` - Domain models with validation
- **Database Layer**: `data-terminal/backend/src/repositories/` - SQLx-based data access
- **API Endpoints**: `data-terminal/backend/src/routes/` - Axum route handlers
- **Configuration**: `data-terminal/backend/config/Setting.toml` - Database and app configuration

**Frontend (Dioxus + WebAssembly):**
- **Main Entry**: `data-terminal/frontend/src/main.rs` - Dioxus application entry
- **Router Configuration**: `data-terminal/frontend/src/routes.rs` - Route definitions
- **Page Components**: `data-terminal/frontend/src/pages/` - Main UI pages (Home, Login, Resource, Datasource)
- **Reusable Components**: `data-terminal/frontend/src/components/` - Shared UI components
- **Data Models**: `data-terminal/frontend/src/models/` - Frontend DTOs matching backend models
- **Client Utilities**: `data-terminal/frontend/src/utils/` - HTTP client, validation, cookies

### **CRITICAL ARCHITECTURAL GAP**: ETL Pipeline Logic, DAG Dataflow, and AI Workflow Integration

**⚠️ IMPORTANT FOR AI AGENTS**: The current data-terminal implementation is a **foundational data management interface** but does **NOT YET CONTAIN**:
- ETL pipeline logic implementation
- DAG-based workflow orchestration
- AI-driven workflow automation
- Data transformation pipelines
- Batch/stream processing capabilities

The current system provides CRUD operations for:
- **Projects**: Logical containers for data processing workflows
- **Resources**: Infrastructure components (databases, compute engines, storage)
- **DataSources**: Input data connections (MySQL, PostgreSQL, APIs)

## High Level Architecture

### Technical Summary

**Current Reality**: Data-terminal serves as the **management interface** for the broader data-factory platform. It provides foundational data management capabilities but delegates actual ETL processing to the Java-based data-engine component.

### Actual Tech Stack (from Cargo.toml files)

| Category | Technology | Version | Notes |
| -------- | ---------- | ------- | ----- |
| **Backend Runtime** | Rust | 2024 edition | Modern async Rust with Tokio |
| **Web Framework** | Axum | 0.8.4 | High-performance HTTP server with macros |
| **Database** | MySQL via SQLx | 0.8.6 | Async SQL toolkit with compile-time queries |
| **Dependency Injection** | Shaku | 0.6.2 | IoC container for service management |
| **Authentication** | JWT (jsonwebtoken) | 9.3.1 | Custom implementation, no OAuth |
| **Frontend Framework** | Dioxus | 0.6.3 | React-like framework targeting WebAssembly |
| **UI Components** | DaisyUI + Tailwind | Latest | CSS framework with component library |
| **State Management** | dioxus-query | 0.8.1 | Data fetching and caching |
| **Icons** | dioxus-free-icons | 0.9 | Multiple icon sets (Bootstrap, Hero, FA, etc.) |

### Repository Structure Reality Check

- **Type**: Cargo workspace with 3 members (backend, frontend, docsite)
- **Package Manager**: Cargo (Rust native)
- **Architecture Pattern**: Clean Architecture with dependency injection
- **Database**: MySQL with table prefix `data_factory_`

## Source Tree and Module Organization

### Project Structure (Actual)

```text
data-terminal/
├── backend/                 # Axum REST API server
│   ├── src/
│   │   ├── main.rs         # Application entry point
│   │   ├── autofac.rs      # Dependency injection container (Shaku)
│   │   ├── models/         # Domain models with validation traits
│   │   │   ├── project.rs  # Project entity with status tracking
│   │   │   ├── resource.rs # Infrastructure resource definitions
│   │   │   ├── datasource.rs # Data source connection models
│   │   │   └── error.rs    # Error types and handling
│   │   ├── services/       # Business logic layer
│   │   ├── repositories/   # Data access layer (SQLx)
│   │   ├── routes/         # HTTP route handlers
│   │   └── utils/          # Configuration and database utilities
│   ├── config/
│   │   └── Setting.toml    # Database connection configuration
│   └── Cargo.toml
├── frontend/                # Dioxus WebAssembly frontend
│   ├── src/
│   │   ├── main.rs         # Dioxus application entry
│   │   ├── routes.rs       # Client-side routing
│   │   ├── pages/          # Route-level components
│   │   │   ├── home.rs     # Dashboard page
│   │   │   ├── login.rs    # Authentication page
│   │   │   ├── resource.rs # Infrastructure management
│   │   │   └── datasource.rs # Data source management
│   │   ├── components/     # Reusable UI components
│   │   ├── models/         # Frontend DTOs
│   │   └── utils/          # HTTP client, cookies, validation
│   ├── assets/             # Static assets and Tailwind CSS
│   └── Cargo.toml
└── docsite/                # Documentation site
```

### Key Modules and Their Purpose

**Backend Core Services:**
- **Project Service**: `backend/src/services/project.rs` - Manages data processing project lifecycles
- **Resource Service**: `backend/src/services/resource.rs` - Infrastructure resource management
- **DataSource Service**: `backend/src/services/datasource.rs` - Data input connection management
- **Global State**: `backend/src/autofac.rs` - Singleton container for dependency injection

**Frontend Pages:**
- **Home Page**: `frontend/src/pages/home.rs` - Main dashboard interface
- **Resource Management**: `frontend/src/pages/resource.rs` - Infrastructure configuration UI
- **DataSource Management**: `frontend/src/pages/datasource.rs` - Data connection configuration UI
- **Authentication**: `frontend/src/pages/login.rs` - JWT-based login interface

## Data Models and APIs

### Core Data Models

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

### API Specifications

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

## Current State vs. Expected ETL/AI Capabilities

### What EXISTS Currently

✅ **Foundation Layer Complete:**
- Project lifecycle management with status tracking
- Infrastructure resource registration and configuration
- Data source connection management and status monitoring
- Clean architecture with proper separation of concerns
- Async Rust backend with compile-time SQL validation
- Modern React-like frontend with WebAssembly performance

### What is MISSING for ETL/DAG/AI Integration

❌ **ETL Pipeline Logic:** No workflow orchestration engine
❌ **DAG Implementation:** No directed acyclic graph processing
❌ **AI Workflow Integration:** No AI agent coordination
❌ **Data Transformation:** No actual data processing capabilities
❌ **Batch Processing:** No job scheduling or execution
❌ **Stream Processing:** No real-time data handling

### **Integration Point with data-engine**

The current data-terminal serves as a **management interface** that will orchestrate the Java-based data-engine component (located at `../data-engine/`) which contains the actual ETL processing capabilities. The integration pattern is:

1. **data-terminal**: Project/resource/datasource configuration and monitoring
2. **data-engine**: Actual ETL execution, job scheduling, and data processing
3. **data-agent**: AI-powered workflow generation and optimization

## Technical Debt and Known Issues

### Critical Technical Debt

1. **Missing Core Functionality**: The ETL pipeline, DAG workflows, and AI integration are architectural gaps
2. **No Data Processing Engine**: Currently only manages metadata, no actual data transformation
3. **Authentication Limitations**: JWT implementation lacks proper token refresh and session management
4. **Configuration Management**: Database credentials hardcoded in TOML files
5. **Error Handling**: Basic error types, needs comprehensive error taxonomy for data processing failures

### Current Implementation Constraints

- **Database**: Hardcoded to MySQL, limited multi-database support
- **Authentication**: Simple JWT without OAuth or enterprise SSO integration
- **Scalability**: Single-instance architecture, no clustering or load balancing
- **Monitoring**: No observability, logging, or metrics collection for ETL workflows

### Known Workarounds

- **CORS Configuration**: Hardcoded to `localhost:8080` for frontend communication
- **Database Connection**: Single connection pool, no connection retries or failover
- **Static Assets**: Manual Tailwind CSS compilation required for styling updates

## Integration Points and External Dependencies

### Current External Services

| Service | Purpose | Integration Type | Key Files |
| ------- | ------- | ---------------- | --------- |
| MySQL Database | Data persistence | SQLx async | `backend/src/utils/database.rs` |
| None (Yet) | ETL Processing | Future: data-engine integration | TBD |
| None (Yet) | AI Workflows | Future: data-agent integration | TBD |

### Internal Integration Points

- **Frontend ↔ Backend**: REST API on port 3000, JSON communication
- **Database Layer**: SQLx with compile-time query validation
- **Static Assets**: Dioxus serves WebAssembly to browser
- **Future Integration**: HTTP/gRPC communication with data-engine for ETL execution

## Development and Deployment

### Local Development Setup

**Backend Development:**
```bash
cd data-terminal/backend
cargo run  # Starts server on port 3000
```

**Frontend Development:**
```bash
cd data-terminal/frontend
dx serve --platform web  # Starts dev server on port 8080
npm run build:css  # Compile Tailwind styles
```

**Database Setup:**
- MySQL server on localhost:3306
- Database: `data_factory_config`
- Tables with `data_factory_` prefix
- Configuration in `backend/config/Setting.toml`

### Build and Deployment Process

**Production Build:**
```bash
# Backend
cargo build --release

# Frontend
cd frontend && dx build --platform web
```

**Current Deployment**: Manual deployment, no CI/CD pipeline configured

## Testing Reality

### Current Test Coverage

- **Unit Tests**: Basic model validation tests
- **Integration Tests**: None implemented
- **E2E Tests**: None implemented
- **Manual Testing**: Primary QA method

### Running Tests

```bash
# Backend tests
cd backend && cargo test

# Frontend tests
cd frontend && cargo test
```

## Future Architecture for ETL/DAG/AI Integration

### Planned Integration Points

**For AI agents implementing ETL pipeline logic:**

1. **Workflow Definition API**: REST endpoints for DAG creation and management
2. **Node Registration**: Dynamic data processing node registration system
3. **Execution Engine Integration**: Communication layer with data-engine for job execution
4. **AI Agent Coordination**: Interfaces for AI-driven workflow optimization and generation

**Expected Data Flow:**
```
data-terminal (UI/Management) → data-engine (Processing) → data-agent (AI Optimization)
```

### Code Generation Target Areas

**For AI agents generating data node code:**

- **Workflow Nodes**: `backend/src/workflow/nodes/` (to be created)
- **DAG Definition**: `backend/src/workflow/dag/` (to be created)
- **Processing Logic**: Integration with `../data-engine/` Java components
- **AI Integration**: Communication with `../data-agent/` Python components

## Appendix - Useful Commands and Scripts

### Development Commands

```bash
# Backend
cargo run                    # Start development server
cargo test                   # Run unit tests
cargo check                  # Check code without building
cargo fmt                    # Format code
cargo clippy                 # Lint code

# Frontend
dx serve --platform web      # Start frontend development server
dx serve --platform desktop  # Run as desktop app
npm run build:css           # Compile Tailwind CSS
npm run watch:css           # Watch CSS changes
```

### Database Operations

```bash
# Connect to MySQL
mysql -u root -p data_factory_config

# Check table structure
SHOW TABLES LIKE 'data_factory_%';
DESCRIBE data_factory_projects;
```

### Common Issues and Solutions

- **CORS Errors**: Ensure frontend runs on localhost:8080
- **Database Connection**: Verify MySQL service and credentials in Setting.toml
- **CSS Not Loading**: Run `npm run build:css` after Tailwind changes
- **WebAssembly Errors**: Clear browser cache and rebuild frontend

---

## Summary for AI Agents

This data-terminal component provides the **foundational management layer** for the data-factory platform but requires significant extension for full ETL/DAG/AI capabilities. Current implementation handles project, resource, and datasource CRUD operations with a modern Rust/WebAssembly stack.

**Key areas for AI-generated code enhancement:**
1. Workflow orchestration engine integration
2. DAG processing node implementations
3. Communication bridges to data-engine (Java) and data-agent (Python)
4. Enhanced error handling for data processing workflows
5. Real-time status monitoring and logging systems

The architecture is well-structured for extension, following clean architecture principles with proper dependency injection and separation of concerns.