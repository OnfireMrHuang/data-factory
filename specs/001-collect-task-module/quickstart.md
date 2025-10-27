# Developer Quickstart: Collect Task Module

**Feature**: `001-collect-task-module`
**Date**: 2025-10-27
**Audience**: Developers implementing or extending the collection task module

---

## Overview

This guide helps developers quickly set up their environment and understand the
architecture for implementing the Collect Task Module. The module enables users
to configure data collection from various datasources to resources through a
full-stack Rust application (Dioxus frontend + Axum backend).

---

## Prerequisites

### Required Tools

- **Rust**: 1.75+ (install via [rustup](https://rustup.rs/))
- **Node.js**: 18+ (for Tailwind CSS build)
- **MySQL**: 8.0+ (for data_factory_template database)
- **Dioxus CLI**: For frontend development
  ```bash
  cargo install dioxus-cli
  ```

### Database Setup

1. Ensure MySQL is running on `localhost:3306`
2. Create database if not exists:
   ```sql
   CREATE DATABASE IF NOT EXISTS data_factory_template;
   USE data_factory_template;
   ```
3. Run migrations:
   ```bash
   mysql -u root -p data_factory_template < migrations/v1.0.0/init_project.sql
   ```
4. Verify tables exist:
   ```sql
   SHOW TABLES LIKE 'df_c_%';
   -- Should show: df_c_datasource, df_c_collection
   ```

---

## Repository Structure

```
data-terminal/
├── backend/                    # Axum REST API server
│   ├── src/
│   │   ├── routes/            # HTTP route handlers
│   │   ├── services/          # Business logic
│   │   ├── repositories/      # Data access layer
│   │   ├── models/            # Data models and DTOs
│   │   ├── utils/             # Utilities (DB, HTTP clients)
│   │   ├── autofac.rs         # Dependency injection
│   │   └── main.rs            # Entry point
│   └── config/Setting.toml    # Configuration
│
├── frontend/                   # Dioxus WebAssembly frontend
│   ├── src/
│   │   ├── pages/             # Route-level components
│   │   ├── components/        # Reusable UI components
│   │   ├── models/            # Frontend DTOs
│   │   ├── utils/             # Client utilities
│   │   ├── routes.rs          # Router configuration
│   │   └── main.rs            # Entry point
│   └── assets/                # Static assets (CSS, icons)
│
└── Cargo.toml                  # Workspace configuration
```

---

## Development Workflow

### Step 1: Run Backend Server

```bash
cd data-terminal
cargo run --bin backend
```

Backend will start on `http://localhost:3000`

**Verify**:
```bash
curl http://localhost:3000/api/v1/health
# Should return: {"status": "ok"}
```

### Step 2: Run Frontend Dev Server

In a new terminal:

```bash
cd data-terminal/frontend
dx serve --platform web
```

Frontend will start on `http://localhost:8080`

**Verify**: Open browser to `http://localhost:8080`

### Step 3: Watch Tailwind CSS (Optional)

If modifying styles:

```bash
cd data-terminal/frontend
npm run watch:css
```

---

## Architecture Patterns

### Clean Architecture Layers

#### Backend (Rust + Axum)

```
┌─────────────────────────────────────────────┐
│             HTTP Request                    │
└─────────────────┬───────────────────────────┘
                  │
         ┌────────▼─────────┐
         │  routes/         │  Parse request, call service
         │  collection.rs   │  Format response
         └────────┬─────────┘
                  │
         ┌────────▼─────────┐
         │  services/       │  Business logic
         │  collection_     │  Orchestrate repositories
         │  service.rs      │  Apply validation rules
         └────────┬──���──────┘
                  │
         ┌────────▼─────────┐
         │  repositories/   │  Data access
         │  collection_     │  SQL queries
         │  repository.rs   │  DB transactions
         └────────┬─────────┘
                  │
         ┌────────▼─────────┐
         │   MySQL          │
         │  df_c_collection │
         └──────────────────┘
```

**Dependency Flow**: Routes → Services → Repositories → Database
**No Upward Dependencies**: Repositories cannot import services, services cannot import routes

#### Frontend (Rust + Dioxus)

```
┌─────────────────────────────────────────────┐
│           Browser URL                       │
└─────────────────┬───────────────────────────┘
                  │
         ┌────────▼─────────┐
         │  pages/          │  Route components
         │  collection_     │  Compose smaller components
         │  page.rs         │  Manage page state
         └────────┬─────────┘
                  │
         ┌────────▼─────────┐
         │  components/     │  Reusable UI
         │  collection/     │  daisy-rsx components
         │  mode_selector.rs│  Emit events
         └────────┬─────────┘
                  │
         ┌────────▼─────────┐
         │  utils/          │  API calls
         │  collection_     │  HTTP client
         │  api.rs          │  Validation
         └────────┬─────────┘
                  │
         ┌────────▼─────────┐
         │  Backend API     │
         │  /api/v1/        │
         └──────────────────┘
```

**Dependency Flow**: Pages → Components → Utils → Backend API

---

## Key Technologies

### Backend Stack

| Technology | Purpose | Documentation |
|------------|---------|---------------|
| **Axum** | HTTP routing and middleware | [axum.rs](https://docs.rs/axum) |
| **SQLx** | Type-safe SQL queries | [sqlx](https://github.com/launchbadge/sqlx) |
| **Shaku** | Dependency injection | [shaku](https://docs.rs/shaku) |
| **Tokio** | Async runtime | [tokio.rs](https://tokio.rs) |
| **Serde** | JSON serialization | [serde.rs](https://serde.rs) |

### Frontend Stack

| Technology | Purpose | Documentation |
|------------|---------|---------------|
| **Dioxus** | React-like UI framework | [dioxuslabs.com](https://dioxuslabs.com) |
| **daisy-rsx** | DaisyUI components for Dioxus | [GitHub](https://github.com/whereasjovially/daisy-rsx) |
| **dioxus-router** | Client-side routing | [docs](https://dioxuslabs.com/learn/0.6/router) |
| **dioxus-toast** | Toast notifications | [docs](https://docs.rs/dioxus-toast) |
| **dioxus-query** | Data fetching and caching | [docs](https://docs.rs/dioxus-query) |

---

## Common Tasks

### Task 1: Add a New Backend Endpoint

**Example**: Add `GET /api/v1/collections/{id}/validate` endpoint

1. **Define route handler** (`routes/collection.rs`):
   ```rust
   pub async fn validate_task(
       Path(id): Path<String>,
       Extension(service): Extension<Arc<dyn CollectionService>>,
   ) -> Result<Json<ValidationResponse>, AppError> {
       let result = service.validate_task(&id).await?;
       Ok(Json(result))
   }
   ```

2. **Implement service method** (`services/collection_service.rs`):
   ```rust
   #[async_trait]
   pub trait CollectionService: Send + Sync {
       async fn validate_task(&self, id: &str) -> Result<ValidationResult, ServiceError>;
   }

   impl CollectionServiceImpl {
       pub async fn validate_task(&self, id: &str) -> Result<ValidationResult, ServiceError> {
           let task = self.repository.find_by_id(id).await?;
           // Validation logic here
           Ok(ValidationResult { valid: true, errors: vec![] })
       }
   }
   ```

3. **Register route** (`routes/collection.rs`):
   ```rust
   pub fn collection_routes() -> Router {
       Router::new()
           .route("/collections/:id/validate", get(validate_task))
           // ... other routes
   }
   ```

4. **Write tests** (`tests/backend/integration/collection_api_test.rs`):
   ```rust
   #[tokio::test]
   async fn test_validate_task() {
       let app = test_app().await;
       let response = app
           .oneshot(
               Request::builder()
                   .uri("/api/v1/collections/test-id/validate")
                   .method("GET")
                   .body(Body::empty())
                   .unwrap(),
           )
           .await
           .unwrap();
       assert_eq!(response.status(), StatusCode::OK);
   }
   ```

---

### Task 2: Add a New Frontend Component

**Example**: Add `TaskStatusBadge` component

1. **Create component file** (`frontend/src/components/collection/task_status_badge.rs`):
   ```rust
   use dioxus::prelude::*;
   use crate::models::collection::TaskStatus;

   #[component]
   pub fn TaskStatusBadge(status: TaskStatus) -> Element {
       let (color, text) = match status {
           TaskStatus::Draft => ("badge-secondary", "Draft"),
           TaskStatus::Saved => ("badge-info", "Saved"),
           TaskStatus::Applied => ("badge-primary", "Applied"),
           TaskStatus::Running => ("badge-accent", "Running"),
           TaskStatus::Failed => ("badge-error", "Failed"),
       };

       rsx! {
           span { class: "badge {color}", "{text}" }
       }
   }
   ```

2. **Export component** (`frontend/src/components/collection/mod.rs`):
   ```rust
   mod task_status_badge;
   pub use task_status_badge::TaskStatusBadge;
   ```

3. **Use in page** (`frontend/src/pages/collection_page.rs`):
   ```rust
   use crate::components::collection::TaskStatusBadge;

   rsx! {
       TaskStatusBadge { status: task.status }
   }
   ```

---

### Task 3: Add Database Query

**Example**: Query collection tasks by status

1. **Add method to repository** (`repositories/collection_repository.rs`):
   ```rust
   impl CollectionRepository {
       pub async fn find_by_status(
           &self,
           status: TaskStatus,
       ) -> Result<Vec<CollectTask>, sqlx::Error> {
           sqlx::query_as!(
               CollectTask,
               r#"
               SELECT id, name, description, category as "category: _",
                      collect_type as "collect_type: _", datasource_id,
                      resource_id, rule, status as "status: _",
                      created_at, updated_at, applied_at
               FROM df_c_collection
               WHERE status = ?
               ORDER BY created_at DESC
               "#,
               status
           )
           .fetch_all(&self.pool)
           .await
       }
   }
   ```

2. **Add index for performance** (if needed):
   ```sql
   CREATE INDEX idx_status ON df_c_collection(status);
   ```

3. **Write test**:
   ```rust
   #[tokio::test]
   async fn test_find_by_status() {
       let pool = test_pool().await;
       let repo = CollectionRepository::new(pool);

       // Insert test data
       let task = create_test_task(TaskStatus::Saved).await;

       // Query
       let results = repo.find_by_status(TaskStatus::Saved).await.unwrap();
       assert!(!results.is_empty());
       assert_eq!(results[0].status, TaskStatus::Saved);
   }
   ```

---

### Task 4: Call Backend API from Frontend

**Example**: Fetch collection task list

1. **Create API client function** (`frontend/src/utils/collection_api.rs`):
   ```rust
   use crate::models::collection::CollectTask;

   pub async fn fetch_collection_tasks() -> Result<Vec<CollectTask>, reqwest::Error> {
       let response = reqwest::get("http://localhost:3000/api/v1/collections")
           .await?
           .json::<ApiResponse<Vec<CollectTask>>>()
           .await?;
       Ok(response.data)
   }
   ```

2. **Use in component with dioxus-query**:
   ```rust
   use dioxus::prelude::*;
   use crate::utils::collection_api;

   #[component]
   pub fn CollectionList() -> Element {
       let tasks = use_resource(|| collection_api::fetch_collection_tasks());

       rsx! {
           match tasks.read().as_ref() {
               Some(Ok(tasks)) => rsx! {
                   ul {
                       for task in tasks {
                           li { "{task.name}" }
                       }
                   }
               },
               Some(Err(e)) => rsx! {
                   div { class: "alert alert-error", "Error: {e}" }
               },
               None => rsx! {
                   div { class: "loading loading-spinner" }
               }
           }
       }
   }
   ```

---

## Testing Strategy

### Backend Tests

#### Unit Tests (services, repositories)

```bash
cargo test --lib
```

**Example** (`services/collection_service_test.rs`):
```rust
#[tokio::test]
async fn test_create_task_validates_datasource_resource_compatibility() {
    let mock_repo = MockCollectionRepository::new();
    let service = CollectionServiceImpl::new(mock_repo);

    let request = CreateCollectTaskRequest {
        category: CollectionCategory::Database,
        collect_type: CollectType::Full,
        resource_id: "queue-resource-id".to_string(), // Invalid!
        // ... other fields
    };

    let result = service.create_task(request).await;
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().code, "INVALID_RESOURCE_TYPE");
}
```

#### Integration Tests (full API)

```bash
cargo test --test '*'
```

**Example** (`tests/backend/integration/collection_api_test.rs`):
```rust
#[tokio::test]
async fn test_create_collection_task_returns_201() {
    let app = test_app().await;
    let payload = json!({
        "name": "Test Task",
        "category": "database",
        "collect_type": "full",
        "datasource_id": "...",
        "resource_id": "...",
        "rule": { "type": "full_database", ... }
    });

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/collections")
                .method("POST")
                .header("content-type", "application/json")
                .body(Body::from(payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);
}
```

### Frontend Tests

```bash
cd frontend
cargo test
```

**Example** (component test):
```rust
#[test]
fn test_task_status_badge_displays_correct_color() {
    let mut vdom = VirtualDom::new(|| rsx! {
        TaskStatusBadge { status: TaskStatus::Running }
    });
    vdom.rebuild();

    let html = dioxus_ssr::render(&vdom);
    assert!(html.contains("badge-accent"));
    assert!(html.contains("Running"));
}
```

---

## Code Quality Checklist

Before submitting a PR, ensure:

- [ ] **Formatting**: `cargo fmt --check` passes
- [ ] **Linting**: `cargo clippy -- -D warnings` passes
- [ ] **Tests**: `cargo test` passes with 80%+ coverage on new code
- [ ] **Type Safety**: No `unwrap()` or `expect()` in production code paths
- [ ] **Error Handling**: All errors use `Result<T, E>` and proper error types
- [ ] **Documentation**: All public APIs have doc comments (`///`)
- [ ] **No God Objects**: No files >500 lines (refactor if needed)

---

## Troubleshooting

### Issue: Backend won't start - "Connection refused" to MySQL

**Solution**:
1. Check MySQL is running: `mysql -u root -p -e "SELECT 1"`
2. Verify database exists: `mysql -u root -p -e "SHOW DATABASES LIKE 'data_factory%'"`
3. Check `backend/config/Setting.toml` connection details

### Issue: Frontend shows blank page

**Solution**:
1. Check browser console for errors (F12)
2. Verify backend is running on port 3000
3. Check CORS configuration in backend (`main.rs`)
4. Clear browser cache and reload

### Issue: SQLx compile-time verification fails

**Solution**:
```bash
# Generate query metadata offline
cargo sqlx prepare --database-url "mysql://root:password@localhost/data_factory_template"
```

### Issue: Dioxus hot reload not working

**Solution**:
```bash
# Restart dx serve with verbose logging
dx serve --platform web --verbose
```

---

## Next Steps

1. **Read the spec**: Review `specs/001-collect-task-module/spec.md` for requirements
2. **Review the data model**: See `specs/001-collect-task-module/data-model.md`
3. **Check API contracts**: `specs/001-collect-task-module/contracts/*.yaml`
4. **Start implementing**: Follow `/speckit.tasks` for task breakdown
5. **Join discussions**: Ask questions in #data-factory-dev Slack channel

---

## Additional Resources

- **Constitution**: `.specify/memory/constitution.md` - Project principles and standards
- **Backend Guide**: `data-terminal/CLAUDE.md` - Detailed backend patterns
- **Dioxus Book**: https://dioxuslabs.com/learn/0.6/ - Framework documentation
- **Axum Examples**: https://github.com/tokio-rs/axum/tree/main/examples
- **SQLx Guide**: https://github.com/launchbadge/sqlx/blob/main/README.md

---

**Happy coding! If you have questions, reach out to the team.**
