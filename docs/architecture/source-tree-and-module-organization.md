# Source Tree and Module Organization

## Project Structure (Actual)

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

## Key Modules and Their Purpose

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
