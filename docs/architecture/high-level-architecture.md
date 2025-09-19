# High Level Architecture

## Technical Summary

**Current Reality**: Data-terminal serves as the **management interface** for the broader data-factory platform. It provides foundational data management capabilities but delegates actual ETL processing to the Java-based data-engine component.

## Actual Tech Stack (from Cargo.toml files)

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

## Repository Structure Reality Check

- **Type**: Cargo workspace with 3 members (backend, frontend, docsite)
- **Package Manager**: Cargo (Rust native)
- **Architecture Pattern**: Clean Architecture with dependency injection
- **Database**: MySQL with table prefix `data_factory_`
