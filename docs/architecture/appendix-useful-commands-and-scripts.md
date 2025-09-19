# Appendix - Useful Commands and Scripts

## Development Commands

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

## Database Operations

```bash
# Connect to MySQL
mysql -u root -p data_factory_config

# Check table structure
SHOW TABLES LIKE 'data_factory_%';
DESCRIBE data_factory_projects;
```

## Common Issues and Solutions

- **CORS Errors**: Ensure frontend runs on localhost:8080
- **Database Connection**: Verify MySQL service and credentials in Setting.toml
- **CSS Not Loading**: Run `npm run build:css` after Tailwind changes
- **WebAssembly Errors**: Clear browser cache and rebuild frontend

---
