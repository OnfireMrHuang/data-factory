# Development and Deployment

## Local Development Setup

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

## Build and Deployment Process

**Production Build:**
```bash
# Backend
cargo build --release

# Frontend
cd frontend && dx build --platform web
```

**Current Deployment**: Manual deployment, no CI/CD pipeline configured
