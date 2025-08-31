# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Data Terminal is a full-stack application built with Rust, featuring a Dioxus frontend and Axum backend for data factory management. The project uses a workspace architecture with separate backend and frontend modules.

## Development Commands

### Backend (Rust + Axum)
```bash
# Run backend server (listens on port 3000)
cargo run --bin backend

# Run tests
cargo test

# Check code
cargo check

# Format code
cargo fmt

# Lint code
cargo clippy
```

### Frontend (Dioxus + WebAssembly)
```bash
# Serve frontend for development (web platform, default port 8080)
cd frontend && dx serve --platform web

# Serve for desktop
cd frontend && dx serve --platform desktop

# Build CSS from Tailwind
cd frontend && npm run build:css

# Watch CSS changes
cd frontend && npm run watch:css

# Optimize SVG assets
cd frontend && svgo -o assets/icons/icon.svg assets/icons/icon.svg
```

## Architecture

### Backend Architecture (Clean Architecture Pattern)
The backend follows a layered architecture with dependency injection using Shaku:

- **`routes/`**: HTTP route handlers using Axum framework. Keep simple, delegate to services
- **`services/`**: Business logic layer. Orchestrates repositories and implements core functionality  
- **`repositories/`**: Data access layer using SQLx with MySQL
- **`models/`**: Data transfer objects and database models
- **`utils/`**: Independent utility functions and configuration
- **`autofac.rs`**: Dependency injection container using Shaku

**Key Patterns:**
- Global app state managed through `autofac.rs` with singleton pattern
- JWT authentication with protected/public route separation
- CORS configuration for frontend communication on localhost:8080
- Database connection managed in `utils/database.rs`
- Configuration loaded from `config/Setting.toml`

### Frontend Architecture (Dioxus Components)
The frontend uses Dioxus (React-like) with component-based architecture:

- **`pages/`**: Route-level components (Home, Login, ResourcePage, DatasourcePage)
- **`components/`**: Reusable UI components (framework, navbar, dialogs, cards)
- **`models/`**: Data models matching backend DTOs
- **`utils/`**: Client utilities (cookies, validation, HTTP requests)
- **`routes.rs`**: Router configuration with Dioxus Router

**Key Libraries:**
- **UI Components**: daisy-rsx for DaisyUI components
- **Logging**: dioxus-logger for console output
- **Error Handling**: dioxus-toast for user notifications
- **Animation**: dioxus-motion for animations
- **State Management**: dioxus-query for data fetching
- **Icons**: dioxus-free-icons (Bootstrap, Hero Icons, Font Awesome, Lucide, Material Design)
- **SDK**: dioxus-sdk for device APIs (storage, clipboard, etc.)

## Configuration

### Database Configuration
- MySQL database configured in `backend/config/Setting.toml`
- Connection details: localhost:3306, database `data_factory_config`
- Uses table prefix `data_factory_`

### Frontend Styling
- Tailwind CSS v4+ with DaisyUI plugin
- Build: `npx @tailwindcss/cli -i ./input.css -o ./assets/tailwind.css`
- Watch mode available for development

## Key Development Guidelines

### Backend Rules (from .cursor/rules)
- Use Axum for HTTP routing, keep routes simple
- Put complex logic in services layer
- Use SQLx for database operations in repositories  
- Use Tokio for async runtime
- Follow clean architecture with proper dependency injection

### Frontend Rules (from .cursor/rules) 
- Use Dioxus framework with component architecture
- Prefer daisy-rsx for UI components when possible
- Use dioxus-logger for console logging
- Use dioxus-toast for error messaging
- Use dioxus-motion for animations
- Use dioxus-query for state management
- Use dioxus-free-icons for iconography

## Project Structure

```
data-terminal/
├── backend/           # Axum REST API server
├── frontend/          # Dioxus WebAssembly frontend  
├── docsite/          # Documentation site
└── Cargo.toml        # Workspace configuration
```

The backend serves API endpoints at `/api/v1/*` while the frontend connects via HTTP requests to localhost:3000.