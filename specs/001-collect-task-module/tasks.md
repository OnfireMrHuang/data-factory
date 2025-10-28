# Tasks: Collect Task Module

**Input**: Design documents from `/specs/001-collect-task-module/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/

**Tests**: Tests are NOT explicitly requested in the feature specification, so test tasks are EXCLUDED from this plan per the test-first approach (tests will be written when explicitly requested).

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story?] Description`
- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3, US4)
- Include exact file paths in descriptions

## Path Conventions
- **Web app**: `data-terminal/backend/src/`, `data-terminal/frontend/src/`
- Paths are absolute from repository root

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure

- [X] T001 Verify project structure matches plan.md (backend/, frontend/ directories in data-terminal/)
- [X] T002 [P] Add backend dependencies to data-terminal/Cargo.toml (Axum 0.7+, SQLx 0.8+, Shaku, serde, chrono, uuid)
- [X] T003 [P] Add frontend dependencies to data-terminal/Cargo.toml (Dioxus 0.6+, daisy-rsx, dioxus-router, dioxus-toast, dioxus-query)
- [X] T004 [P] Configure Rust formatting and linting in data-terminal/
- [X] T005 Verify MySQL database 'data_factory_template' exists and run schema verification

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

### Database Schema (Foundational)

- [X] T006 Verify df_c_collection table schema in migrations/v1.0.0/init_project.sql has required fields (id, name, description, category, collect_type, datasource_id, resource_id, rule, status, created_at, updated_at, applied_at)
- [X] T007 Add database indexes for df_c_collection table (idx_datasource_id, idx_resource_id, idx_status, idx_category_type)
- [X] T008 [P] Verify df_c_datasource table exists and has required fields
- [X] T009 [P] Create or verify df_c_resource table schema exists

### Backend Core Models (Foundational)

- [X] T010 [P] Create CollectTask model in data-terminal/backend/src/models/collection.rs with all enums (CollectionCategory, CollectType, TaskStatus)
- [X] T011 [P] Create CollectionRule variants in data-terminal/backend/src/models/collection.rs (FullDatabaseRule, FullApiRule, IncrementalDatabaseRule, IncrementalApiRule)
- [X] T012 [P] Create DTOs in data-terminal/backend/src/models/collection.rs (CreateCollectTaskRequest, UpdateCollectTaskRequest, CollectTaskResponse)
- [X] T013 [P] Update data-terminal/backend/src/models/mod.rs to export collection models

### Backend Infrastructure (Foundational)

- [X] T014 Create CollectionRepository trait and implementation in data-terminal/backend/src/repositories/collection_repository.rs
- [X] T015 Create CollectionService trait and implementation in data-terminal/backend/src/services/collection_service.rs
- [X] T016 Create DatasourceService extensions in data-terminal/backend/src/services/datasource_service.rs for metadata queries
- [X] T017 Create DataEngineClient HTTP client in data-terminal/backend/src/utils/data_engine_client.rs
- [X] T018 Register CollectionRepository in data-terminal/backend/src/autofac.rs
- [X] T019 Register CollectionService in data-terminal/backend/src/autofac.rs
- [X] T020 Update data-terminal/backend/src/repositories/mod.rs to export collection_repository
- [X] T021 Update data-terminal/backend/src/services/mod.rs to export collection_service and data_engine_client

### Frontend Core Models (Foundational)

- [X] T022 [P] Create frontend CollectTask model in data-terminal/frontend/src/models/collection.rs with all enums
- [X] T023 [P] Create frontend CollectionRule variants in data-terminal/frontend/src/models/collection.rs
- [X] T024 [P] Update data-terminal/frontend/src/models/mod.rs to export collection models

### Frontend Infrastructure (Foundational)

- [X] T025 Create collection API client in data-terminal/frontend/src/utils/collection_api.rs with base functions
- [X] T026 Update data-terminal/frontend/src/utils/mod.rs to export collection_api
- [X] T027 Add collection routes to data-terminal/frontend/src/routes.rs

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - Full Collection from Database Datasource (Priority: P1) 🎯 MVP

**Goal**: Enable data engineers to configure full collection tasks from database datasources (MySQL/PostgreSQL) to relational database resources with table/field selection, SQL transformation, and target schema generation.

**Independent Test**: Create a collection task that selects tables/fields from a source database, applies SQL transformations, generates and applies target table schema, and saves/applies the task to data-engine.

### Backend Implementation for User Story 1

- [ ] T028 [P] [US1] Implement get_datasource_tables method in data-terminal/backend/src/services/datasource_service.rs (query INFORMATION_SCHEMA for MySQL/PostgreSQL)
- [ ] T029 [P] [US1] Implement get_table_fields method in data-terminal/backend/src/services/datasource_service.rs (query INFORMATION_SCHEMA.COLUMNS)
- [ ] T030 [US1] Implement create_collection_task method in data-terminal/backend/src/services/collection_service.rs with validation for Full+Database mode
- [ ] T031 [US1] Implement generate_target_schema method in data-terminal/backend/src/services/collection_service.rs with type mapping logic
- [ ] T032 [US1] Implement apply_collection_task method in data-terminal/backend/src/services/collection_service.rs (call data-engine API via DataEngineClient)
- [ ] T033 [P] [US1] Implement create_collection_task repository method in data-terminal/backend/src/repositories/collection_repository.rs
- [ ] T034 [P] [US1] Implement find_by_id repository method in data-terminal/backend/src/repositories/collection_repository.rs
- [ ] T035 [P] [US1] Implement update_collection_task repository method in data-terminal/backend/src/repositories/collection_repository.rs

### Backend API Routes for User Story 1

- [ ] T036 [P] [US1] Create POST /api/v1/collections route handler in data-terminal/backend/src/routes/collection.rs
- [ ] T037 [P] [US1] Create GET /api/v1/collections/:id route handler in data-terminal/backend/src/routes/collection.rs
- [ ] T038 [P] [US1] Create PUT /api/v1/collections/:id route handler in data-terminal/backend/src/routes/collection.rs
- [ ] T039 [P] [US1] Create POST /api/v1/collections/:id/apply route handler in data-terminal/backend/src/routes/collection.rs
- [ ] T040 [P] [US1] Create GET /api/v1/datasources/:id/tables route handler in data-terminal/backend/src/routes/collection.rs
- [ ] T041 [P] [US1] Create GET /api/v1/datasources/:id/tables/:tableName/fields route handler in data-terminal/backend/src/routes/collection.rs
- [ ] T042 [P] [US1] Create POST /api/v1/collections/generate-schema route handler in data-terminal/backend/src/routes/collection.rs
- [ ] T043 [US1] Register collection routes in data-terminal/backend/src/main.rs

### Frontend Components for User Story 1

- [ ] T044 [P] [US1] Create ModeSelector component in data-terminal/frontend/src/components/collection/mode_selector.rs (Full vs Incremental)
- [ ] T045 [P] [US1] Create DatasourceSelector component in data-terminal/frontend/src/components/collection/datasource_selector.rs
- [ ] T046 [P] [US1] Create ResourceSelector component in data-terminal/frontend/src/components/collection/resource_selector.rs
- [ ] T047 [US1] Create DbConfigPanel component in data-terminal/frontend/src/components/collection/db_config_panel.rs (table/field selection)
- [ ] T048 [US1] Create TransformEditor component in data-terminal/frontend/src/components/collection/transform_editor.rs (SQL editor with textarea)
- [ ] T049 [US1] Create TargetSchemaEditor component in data-terminal/frontend/src/components/collection/target_schema_editor.rs (schema display and editing)
- [ ] T050 [US1] Create TaskStatusBadge component in data-terminal/frontend/src/components/collection/task_status_badge.rs
- [ ] T051 [US1] Update data-terminal/frontend/src/components/collection/mod.rs to export all US1 components

### Frontend Pages for User Story 1

- [ ] T052 [US1] Create CollectionPage in data-terminal/frontend/src/pages/collection_page.rs (list collection tasks)
- [ ] T053 [US1] Create CollectionCreatePage in data-terminal/frontend/src/pages/collection_create_page.rs (multi-step wizard for Full-Database)
- [ ] T054 [US1] Create CollectionEditPage in data-terminal/frontend/src/pages/collection_edit_page.rs (edit saved tasks)
- [ ] T055 [US1] Update data-terminal/frontend/src/pages/mod.rs to export collection pages

### Frontend API Client for User Story 1

- [ ] T056 [P] [US1] Implement fetch_collection_tasks function in data-terminal/frontend/src/utils/collection_api.rs
- [ ] T057 [P] [US1] Implement create_collection_task function in data-terminal/frontend/src/utils/collection_api.rs
- [ ] T058 [P] [US1] Implement update_collection_task function in data-terminal/frontend/src/utils/collection_api.rs
- [ ] T059 [P] [US1] Implement apply_collection_task function in data-terminal/frontend/src/utils/collection_api.rs
- [ ] T060 [P] [US1] Implement fetch_datasource_tables function in data-terminal/frontend/src/utils/collection_api.rs
- [ ] T061 [P] [US1] Implement fetch_table_fields function in data-terminal/frontend/src/utils/collection_api.rs
- [ ] T062 [P] [US1] Implement generate_target_schema function in data-terminal/frontend/src/utils/collection_api.rs

**Checkpoint**: User Story 1 (Full Collection from Database) should be fully functional - users can configure database-to-database collection tasks with table selection, SQL transformation, and schema generation.

---

## Phase 4: User Story 2 - Full Collection from API Datasource (Priority: P2)

**Goal**: Enable data engineers to configure full collection tasks from Query API datasources with query scheduling, cursor management, JSON transformation, and target selection (database or file system).

**Independent Test**: Configure a Query API datasource with a schedule, define cursor update strategy, transform JSON responses to target schema, select or create target table/file, and save/apply the task.

### Backend Implementation for User Story 2

- [ ] T063 [US2] Extend create_collection_task in data-terminal/backend/src/services/collection_service.rs to support Full+API mode validation
- [ ] T064 [US2] Implement validate_api_schedule method in data-terminal/backend/src/services/collection_service.rs (cron/interval validation)
- [ ] T065 [US2] Implement validate_cursor_strategy method in data-terminal/backend/src/services/collection_service.rs
- [ ] T066 [US2] Extend generate_target_schema in data-terminal/backend/src/services/collection_service.rs to support file format schemas (JSON, CSV, Parquet)

### Frontend Components for User Story 2

- [ ] T067 [P] [US2] Create ApiConfigPanel component in data-terminal/frontend/src/components/collection/api_config_panel.rs (schedule + cursor config)
- [ ] T068 [P] [US2] Update TransformEditor component in data-terminal/frontend/src/components/collection/transform_editor.rs to support JSON mode
- [ ] T069 [US2] Update data-terminal/frontend/src/components/collection/mod.rs to export ApiConfigPanel

### Frontend Pages for User Story 2

- [ ] T070 [US2] Extend CollectionCreatePage in data-terminal/frontend/src/pages/collection_create_page.rs to support Full-API wizard flow

### Frontend API Client for User Story 2

- [ ] T071 [US2] Extend create_collection_task function in data-terminal/frontend/src/utils/collection_api.rs to support FullApiRule payload

**Checkpoint**: User Story 2 (Full Collection from API) should be fully functional - users can configure API-to-database/file collection tasks with scheduling and transformations.

---

## Phase 5: User Story 3 - Incremental Collection from Database Datasource (Priority: P3)

**Goal**: Enable data engineers to configure incremental collection tasks from database datasources (CDC) to queue resources with table selection, filter rules, message transformations, and topic configuration.

**Independent Test**: Select source tables for CDC, define queue topic name and message schema, apply filter rules and field transformations, and save/apply the CDC task.

### Backend Implementation for User Story 3

- [ ] T072 [US3] Extend create_collection_task in data-terminal/backend/src/services/collection_service.rs to support Incremental+Database mode validation
- [ ] T073 [US3] Implement validate_cdc_config method in data-terminal/backend/src/services/collection_service.rs
- [ ] T074 [US3] Implement validate_filter_rules method in data-terminal/backend/src/services/collection_service.rs (JSON DSL validation)
- [ ] T075 [US3] Implement validate_message_transformations method in data-terminal/backend/src/services/collection_service.rs

### Frontend Components for User Story 3

- [ ] T076 [P] [US3] Create FilterRuleEditor component in data-terminal/frontend/src/components/collection/filter_rule_editor.rs (JSON DSL editor)
- [ ] T077 [P] [US3] Create TopicConfigPanel component in data-terminal/frontend/src/components/collection/topic_config_panel.rs (topic name + message schema)
- [ ] T078 [US3] Update data-terminal/frontend/src/components/collection/mod.rs to export US3 components

### Frontend Pages for User Story 3

- [ ] T079 [US3] Extend CollectionCreatePage in data-terminal/frontend/src/pages/collection_create_page.rs to support Incremental-Database wizard flow

### Frontend API Client for User Story 3

- [ ] T080 [US3] Extend create_collection_task function in data-terminal/frontend/src/utils/collection_api.rs to support IncrementalDatabaseRule payload

**Checkpoint**: User Story 3 (Incremental Collection from Database) should be fully functional - users can configure CDC tasks with filter rules and message transformations.

---

## Phase 6: User Story 4 - Incremental Collection from API Datasource (Priority: P4)

**Goal**: Enable data engineers to configure incremental collection tasks from Subscription API datasources (webhooks) to queue resources with filter rules, message transformations, and topic configuration.

**Independent Test**: Configure a Subscription API datasource, define filter rules and field transformations in left panel, define queue topic and message schema in right panel, and save/apply the webhook task.

### Backend Implementation for User Story 4

- [ ] T081 [US4] Extend create_collection_task in data-terminal/backend/src/services/collection_service.rs to support Incremental+API mode validation
- [ ] T082 [US4] Reuse validate_filter_rules and validate_message_transformations from US3 (no new backend logic needed)

### Frontend Pages for User Story 4

- [ ] T083 [US4] Extend CollectionCreatePage in data-terminal/frontend/src/pages/collection_create_page.rs to support Incremental-API wizard flow

### Frontend API Client for User Story 4

- [ ] T084 [US4] Extend create_collection_task function in data-terminal/frontend/src/utils/collection_api.rs to support IncrementalApiRule payload

**Checkpoint**: All user stories (US1-US4) should now be independently functional and testable.

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories or overall system quality

### Additional API Endpoints

- [ ] T085 [P] Create GET /api/v1/collections route handler in data-terminal/backend/src/routes/collection.rs (list with pagination/filtering)
- [ ] T086 [P] Create DELETE /api/v1/collections/:id route handler in data-terminal/backend/src/routes/collection.rs
- [ ] T087 [P] Implement find_all repository method in data-terminal/backend/src/repositories/collection_repository.rs (pagination + filters)
- [ ] T088 [P] Implement delete_by_id repository method in data-terminal/backend/src/repositories/collection_repository.rs

### Error Handling & Validation

- [ ] T089 [P] Add comprehensive validation in data-terminal/backend/src/services/collection_service.rs for datasource/resource compatibility
- [ ] T090 [P] Add user-facing error messages with dioxus-toast in all frontend pages
- [ ] T091 [P] Add loading states for all async operations in frontend components

### Performance & Optimization

- [ ] T092 [P] Add database connection pooling configuration in data-terminal/backend/src/main.rs
- [ ] T093 [P] Optimize database queries with EXPLAIN in data-terminal/backend/src/repositories/collection_repository.rs
- [ ] T094 [P] Add caching for datasource metadata queries in data-terminal/backend/src/services/datasource_service.rs

### Documentation

- [ ] T095 [P] Add Rust doc comments to all public APIs in data-terminal/backend/src/services/collection_service.rs
- [ ] T096 [P] Add Rust doc comments to all public APIs in data-terminal/backend/src/repositories/collection_repository.rs
- [ ] T097 [P] Verify quickstart.md instructions work end-to-end

### Code Quality

- [ ] T098 Run cargo fmt --check on all Rust code
- [ ] T099 Run cargo clippy --all-targets -- -D warnings and fix any issues
- [ ] T100 Verify no files exceed 500 lines (refactor if needed)

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3-6)**: All depend on Foundational phase completion
  - User stories can then proceed in parallel (if staffed)
  - Or sequentially in priority order (P1 → P2 → P3 → P4)
- **Polish (Phase 7)**: Depends on all desired user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 2 (P2)**: Can start after Foundational (Phase 2) - Extends US1 components (ModeSelector, DatasourceSelector, ResourceSelector) but independently testable
- **User Story 3 (P3)**: Can start after Foundational (Phase 2) - Extends US1 components but independently testable
- **User Story 4 (P4)**: Can start after Foundational (Phase 2) - Reuses US3 validation logic but independently testable

### Within Each User Story

- Backend models before services
- Services before repository implementations
- Repository before routes
- Routes before frontend components
- Frontend components before pages
- API client functions can be implemented in parallel with components
- Core implementation before integration

### Parallel Opportunities

#### Phase 1 (Setup)
- T002 (backend deps), T003 (frontend deps), T004 (formatting) can run in parallel

#### Phase 2 (Foundational)
- T008 (datasource table), T009 (resource table) can run in parallel after T006-T007
- T010 (CollectTask), T011 (CollectionRule), T012 (DTOs) can run in parallel
- T022 (frontend models), T023 (frontend rules) can run in parallel

#### Phase 3 (User Story 1)
- T028 (get_tables), T029 (get_fields) can run in parallel
- T033 (create repo), T034 (find_by_id), T035 (update repo) can run in parallel
- T036-T042 (route handlers) can run in parallel after services are complete
- T044 (ModeSelector), T045 (DatasourceSelector), T046 (ResourceSelector) can run in parallel
- T056-T062 (API client functions) can run in parallel

#### Phase 4-6 (User Stories 2-4)
- These phases extend existing components, so fewer parallel opportunities

#### Phase 7 (Polish)
- T085-T088 (additional endpoints) can run in parallel
- T089-T091 (error handling) can run in parallel
- T092-T094 (performance) can run in parallel
- T095-T097 (documentation) can run in parallel

---

## Parallel Example: User Story 1

```bash
# Launch backend repository methods together:
Task: "Implement create_collection_task repository method in data-terminal/backend/src/repositories/collection_repository.rs"
Task: "Implement find_by_id repository method in data-terminal/backend/src/repositories/collection_repository.rs"
Task: "Implement update_collection_task repository method in data-terminal/backend/src/repositories/collection_repository.rs"

# Launch backend route handlers together (after services complete):
Task: "Create POST /api/v1/collections route handler in data-terminal/backend/src/routes/collection.rs"
Task: "Create GET /api/v1/collections/:id route handler in data-terminal/backend/src/routes/collection.rs"
Task: "Create PUT /api/v1/collections/:id route handler in data-terminal/backend/src/routes/collection.rs"

# Launch frontend components together:
Task: "Create ModeSelector component in data-terminal/frontend/src/components/collection/mode_selector.rs"
Task: "Create DatasourceSelector component in data-terminal/frontend/src/components/collection/datasource_selector.rs"
Task: "Create ResourceSelector component in data-terminal/frontend/src/components/collection/resource_selector.rs"

# Launch frontend API client functions together:
Task: "Implement fetch_collection_tasks function in data-terminal/frontend/src/utils/collection_api.rs"
Task: "Implement create_collection_task function in data-terminal/frontend/src/utils/collection_api.rs"
Task: "Implement update_collection_task function in data-terminal/frontend/src/utils/collection_api.rs"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (CRITICAL - blocks all stories)
3. Complete Phase 3: User Story 1 (Full Collection from Database)
4. **STOP and VALIDATE**: Test User Story 1 independently:
   - Create a database datasource (MySQL/PostgreSQL)
   - Create a relational database resource
   - Configure a full collection task with table/field selection
   - Generate and apply target schema
   - Apply task to data-engine
   - Verify task appears in data-engine with status "applied"
5. Deploy/demo if ready

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready
2. Add User Story 1 → Test independently → Deploy/Demo (MVP!)
   - **Delivers**: Database-to-database ETL with schema generation
3. Add User Story 2 → Test independently → Deploy/Demo
   - **Delivers**: API-to-database/file integration with scheduling
4. Add User Story 3 → Test independently → Deploy/Demo
   - **Delivers**: Real-time CDC to message queues
5. Add User Story 4 → Test independently → Deploy/Demo
   - **Delivers**: Webhook integration to message queues
6. Each story adds value without breaking previous stories

### Parallel Team Strategy

With multiple developers (after Foundational phase completes):

- **Team A**: User Story 1 (Backend + Frontend) - 7-10 days
- **Team B**: User Story 2 (Backend + Frontend) - 5-7 days (starts after US1 components available)
- **Team C**: User Story 3 (Backend + Frontend) - 5-7 days (starts after US1 components available)
- **Team D**: User Story 4 (Backend + Frontend) - 3-5 days (starts after US3 validation logic available)

**Critical Path**: Foundational → US1 → US2/US3 (parallel) → US4 → Polish

---

## Summary Statistics

- **Total Tasks**: 100
- **Setup Tasks**: 5
- **Foundational Tasks**: 22 (BLOCKS all user stories)
- **User Story 1 Tasks**: 35 (MVP - Full Collection from Database)
- **User Story 2 Tasks**: 9 (Full Collection from API)
- **User Story 3 Tasks**: 9 (Incremental Collection from Database/CDC)
- **User Story 4 Tasks**: 4 (Incremental Collection from API/Webhook)
- **Polish Tasks**: 16

**Parallel Opportunities Identified**: 45 tasks marked [P] can run in parallel within their phase

**Suggested MVP Scope**: Phase 1 + Phase 2 + Phase 3 (User Story 1 only) = 62 tasks

**Independent Test Criteria**:
- **US1**: Create and apply a database-to-database collection task with schema generation
- **US2**: Create and apply an API-to-database/file collection task with scheduling
- **US3**: Create and apply a CDC task with filter rules
- **US4**: Create and apply a webhook task with transformations

---

## Notes

- [P] tasks = different files, no dependencies within the same phase
- [Story] label (US1-US4) maps task to specific user story for traceability
- Each user story is independently completable and testable
- No test tasks included per feature specification (tests not explicitly requested)
- All tasks follow checklist format: `- [ ] [ID] [P?] [Story?] Description with file path`
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- Architecture follows clean separation: routes → services → repositories → models
