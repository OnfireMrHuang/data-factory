# Quick Reference - Key Files and Entry Points

## Critical Files for Understanding the System

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

## **CRITICAL ARCHITECTURAL GAP**: ETL Pipeline Logic, DAG Dataflow, and AI Workflow Integration

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
