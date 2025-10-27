# Implementation Plan: Collect Task Module

**Branch**: `001-collect-task-module` | **Date**: 2025-10-27 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/001-collect-task-module/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

Implement a comprehensive data collection task module that enables users to configure and execute data collection from various datasources (MySQL, PostgreSQL, Query API, Subscription API) to different resource types (relational database, file system, queue). The module supports both Full Collection (batch ETL) and Incremental Collection (CDC and streaming) modes. The implementation spans both frontend UI (Dioxus/Rust) for task configuration and backend services (Axum/Rust) for task management and data pipeline integration.

## Technical Context

**Language/Version**: Rust 1.75+ (both frontend and backend)
**Primary Dependencies**:
  - Frontend: Dioxus 0.6+, daisy-rsx, dioxus-router, dioxus-query, dioxus-toast
  - Backend: Axum 0.7+, SQLx 0.8+, Shaku (DI), Tokio (async runtime)
**Storage**: MySQL 8.0+ (database: `data_factory_template`, tables: `df_c_datasource`, `df_c_collection`)
**Testing**: cargo test, integration tests for backend services, component tests for frontend
**Target Platform**:
  - Frontend: WebAssembly (WASM) targeting modern browsers (Chrome, Firefox, Safari)
  - Backend: Linux server (Docker containers)
**Project Type**: Web application (separate backend REST API + frontend WASM)
**Performance Goals**:
  - API response time: p95 < 200ms for list queries, < 500ms for complex queries
  - Frontend: FCP < 1.5s, TTI < 3s
  - Support minimum 100 concurrent collection task configurations
**Constraints**:
  - Backend memory: < 500MB RSS under normal load
  - Frontend memory: < 100MB heap per active tab
  - Must integrate with existing data-engine module for pipeline execution (NEEDS CLARIFICATION on integration API)
  - Must follow clean architecture pattern with dependency injection
**Scale/Scope**:
  - Support minimum 100 datasources and resources without performance degradation
  - 4 user stories (P1-P4) with 10 acceptance scenarios each
  - Estimated 15-20 new UI components, 8-10 backend services

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### Code Quality Gates
- [ ] Rust type safety: All production code uses proper `Result<T, E>` and `Option<T>` instead of `unwrap()`/`expect()`
- [ ] Zero compiler/linter warnings: `cargo clippy --all-targets -- -D warnings` passes
- [ ] Code formatting: `cargo fmt -- --check` passes for all modules
- [ ] Documentation: All public APIs have doc comments, complex logic has inline comments
- [ ] No God Objects: No files > 500 lines (refactor if needed)

### Testing Gates
- [ ] Test-First approach: Tests written before implementation for all business logic
- [ ] Test coverage: Minimum 80% coverage for backend services
- [ ] Critical user journeys have integration tests (Full Collection DB, API Collection, Incremental CDC, Webhook)
- [ ] All public API endpoints have contract tests
- [ ] Performance tests for collection task configuration (< 200ms p95)
- [ ] Tests are independent and clean up resources (test database, mock servers)

### Architecture Gates
- [ ] Clean architecture layers respected:
  - Frontend: pages → components → models → utils (no circular dependencies)
  - Backend: routes → services → repositories → models (no upward dependencies)
- [ ] Dependency injection used for all service dependencies (Shaku)
- [ ] No circular dependencies between modules
- [ ] Error handling uses proper types (no unwrap in request handlers)
- [ ] Inter-module communication: data-engine integration via versioned REST API only (no direct DB access)

### User Experience Gates
- [ ] All UI components use daisy-rsx (DaisyUI components)
- [ ] Responsive design: Tested on desktop (1920x1080) and tablet (768px)
- [ ] Error handling: Use dioxus-toast for user-facing errors with actionable messages
- [ ] Loading states: All async operations show loading indicators
- [ ] Accessibility: Keyboard navigation, proper labels, ARIA attributes
- [ ] Performance: FCP < 1.5s, TTI < 3s

### Performance Gates
- [ ] Backend API response times:
  - List collection tasks: p95 < 200ms
  - Create/update collection task: p95 < 500ms
  - Complex queries (datasource metadata): p95 < 500ms
- [ ] Frontend WASM bundle: < 5MB compressed
- [ ] Database queries use proper indexes (verify with EXPLAIN)
- [ ] No N+1 queries (use joins/eager loading)
- [ ] Connection pooling configured (5-20 connections)

### Documentation Gates
- [ ] Public APIs have doc comments (Rust doc)
- [ ] Architecture docs updated for collection task module
- [ ] API endpoints documented in OpenAPI/Swagger
- [ ] Migration guide for new `df_c_collection` table if schema changes

### Multi-Module Architecture Gates
- [ ] data-terminal handles UI and collection task metadata only
- [ ] data-engine integration via REST API (no shared database)
- [ ] API contracts versioned (v1) and documented
- [ ] Module independence: data-terminal deployable without data-engine for configuration

**Gate Violations Requiring Justification**: None identified at this stage. All requirements align with constitution principles.

---

### Post-Design Re-Evaluation (Phase 1 Complete)

**Date**: 2025-10-27
**Status**: ✅ All gates remain satisfied after design phase

**Design Artifacts Produced**:
- ✅ `research.md`: Resolved all NEEDS CLARIFICATION items (data-engine integration API)
- ✅ `data-model.md`: Entity schemas, relationships, DTOs, state transitions
- ✅ `contracts/collection-api.yaml`: OpenAPI spec for data-terminal REST API
- ✅ `contracts/data-engine-integration.yaml`: Expected data-engine API contract
- ✅ `quickstart.md`: Developer setup and common task guides
- ✅ Agent context updated: `CLAUDE.md` reflects new technologies

**Architecture Validation**:
- ✅ Clean architecture maintained: Clear separation of routes/services/repositories/models
- ✅ Inter-module communication: REST API contracts defined for data-engine integration
- ✅ No database coupling: data-terminal and data-engine use separate databases
- ✅ Type safety: All models use Rust enums and Result<T, E> for error handling
- ✅ Testability: Architecture supports unit, integration, and contract tests

**Complexity Assessment**:
- **Component Count**: ~20 new files (10 backend, 10 frontend)
- **API Endpoints**: 7 data-terminal endpoints, 6 expected data-engine endpoints
- **Entity Complexity**: 1 primary entity (CollectTask), 4 rule variants (JSON), moderate complexity
- **Test Coverage Target**: 80%+ for services, integration tests for all user stories

**Risks and Mitigations**:
1. **Risk**: data-engine API may not exist yet
   - **Mitigation**: Document expected contract in `contracts/data-engine-integration.yaml`, coordinate with data-engine team early
2. **Risk**: JSON rule validation complexity
   - **Mitigation**: Use Rust serde for type-safe deserialization, write comprehensive validation tests
3. **Risk**: Database metadata queries may be slow for large schemas
   - **Mitigation**: Add caching layer, implement pagination for table/field lists

**Constitution Compliance**:
- ✅ No violations introduced during design
- ✅ All gates remain enforceable during implementation
- ✅ No additional complexity requiring justification

**Ready for Phase 2**: Task generation (`/speckit.tasks`) can proceed

## Project Structure

### Documentation (this feature)

```
specs/001-collect-task-module/
├── spec.md              # Feature specification (completed)
├── plan.md              # This file (/speckit.plan output)
├── research.md          # Phase 0 research findings
├── data-model.md        # Phase 1 data model design
├── quickstart.md        # Phase 1 developer quickstart guide
├── contracts/           # Phase 1 API contracts (OpenAPI)
│   ├── collection-api.yaml
│   └── data-engine-integration.yaml
└── tasks.md             # Phase 2 implementation tasks (/speckit.tasks - NOT created yet)
```

### Source Code (repository root)

This is a **Web application** with separate frontend (Dioxus/WASM) and backend (Axum/Rust).

```
data-terminal/
├── backend/src/
│   ├── routes/
│   │   ├── collection.rs           # NEW: Collection task CRUD endpoints
│   │   └── mod.rs                  # UPDATE: Register collection routes
│   ├── services/
│   │   ├── collection_service.rs   # NEW: Collection task business logic
│   │   ├── datasource_service.rs   # UPDATE: Metadata queries for task config
│   │   ├── resource_service.rs     # UPDATE: Metadata queries for task config
│   │   └── mod.rs                  # UPDATE: Register new services
│   ├── repositories/
│   │   ├── collection_repository.rs # NEW: Collection task data access
│   │   └── mod.rs                   # UPDATE: Register new repository
│   ├── models/
│   │   ├── collection.rs            # NEW: CollectTask, CollectionRule DTOs
│   │   └── mod.rs                   # UPDATE: Export new models
│   ├── utils/
│   │   └── data_engine_client.rs    # NEW: HTTP client for data-engine API
│   ├── autofac.rs                   # UPDATE: Register new services in DI
│   └── main.rs                      # UPDATE: Register collection routes
│
├── frontend/src/
│   ├── pages/
│   │   ├── collection_page.rs       # NEW: Main collection task page
│   │   ├── collection_create_page.rs # NEW: Collection task creation wizard
│   │   ├── collection_edit_page.rs  # NEW: Edit existing collection task
│   │   └── mod.rs                   # UPDATE: Export new pages
│   ├── components/
│   │   ├── collection/              # NEW: Collection-specific components
│   │   │   ├── mode_selector.rs     # Full vs Incremental selection
│   │   │   ├── datasource_selector.rs # Datasource dropdown
│   │   │   ├── resource_selector.rs  # Resource dropdown
│   │   │   ├── db_config_panel.rs   # Database collection config (left panel)
│   │   │   ├── api_config_panel.rs  # API collection config (left panel)
│   │   │   ├── transform_editor.rs  # SQL/JSON transform (middle panel)
│   │   │   ├── target_schema_editor.rs # Schema editor (right panel)
│   │   │   ├── topic_config_panel.rs # Queue topic config (right panel)
│   │   │   ├── filter_rule_editor.rs # Filter rules (middle panel)
│   │   │   ├── task_status_badge.rs # Status indicator
│   │   │   └── mod.rs               # Export all collection components
│   │   └── mod.rs                   # UPDATE: Export collection module
│   ├── models/
│   │   ├── collection.rs            # NEW: Frontend DTOs matching backend
│   │   └── mod.rs                   # UPDATE: Export new models
│   ├── utils/
│   │   ├── collection_api.rs        # NEW: API client for collection endpoints
│   │   └── mod.rs                   # UPDATE: Export new utils
│   ├── routes.rs                    # UPDATE: Add collection task routes
│   └── main.rs                      # No changes needed
│
└── Cargo.toml                       # UPDATE: Add dependencies if needed

migrations/v1.0.0/
└── init_project.sql                 # UPDATE: Append new collection task schema if needed

tests/
├── backend/
│   ├── integration/
│   │   ├── collection_api_test.rs   # NEW: Integration tests for collection API
│   │   └── data_engine_integration_test.rs # NEW: Test data-engine API calls
│   └── unit/
│       ├── collection_service_test.rs # NEW: Unit tests for collection service
│       └── collection_repository_test.rs # NEW: Unit tests for repository
└── frontend/
    └── component/
        ├── collection_mode_selector_test.rs # NEW: Component tests
        └── datasource_selector_test.rs      # NEW: Component tests
```

**Structure Decision**:
- Using **Web application** structure (Option 2) with separate `backend/` and `frontend/` directories
- Backend follows clean architecture: routes → services → repositories → models
- Frontend follows component architecture: pages → components → models → utils
- All new code under `data-terminal/` module (UI + configuration management)
- Integration with `data-engine/` module via REST API (HTTP client in utils)
- Database schema changes appended to `migrations/v1.0.0/init_project.sql`

## Complexity Tracking

*Fill ONLY if Constitution Check has violations that must be justified*

No violations identified. The feature aligns with all constitutional principles:
- Clean architecture maintained with proper layer separation
- Test-first approach planned for all business logic
- UI follows daisy-rsx component standards
- Performance requirements align with constitution targets
- Inter-module communication via REST API only (no database coupling)

