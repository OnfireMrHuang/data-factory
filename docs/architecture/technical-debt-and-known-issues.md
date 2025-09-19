# Technical Debt and Known Issues

## Critical Technical Debt

1. **Missing Core Functionality**: The ETL pipeline, DAG workflows, and AI integration are architectural gaps
2. **No Data Processing Engine**: Currently only manages metadata, no actual data transformation
3. **Authentication Limitations**: JWT implementation lacks proper token refresh and session management
4. **Configuration Management**: Database credentials hardcoded in TOML files
5. **Error Handling**: Basic error types, needs comprehensive error taxonomy for data processing failures

## Current Implementation Constraints

- **Database**: Hardcoded to MySQL, limited multi-database support
- **Authentication**: Simple JWT without OAuth or enterprise SSO integration
- **Scalability**: Single-instance architecture, no clustering or load balancing
- **Monitoring**: No observability, logging, or metrics collection for ETL workflows

## Known Workarounds

- **CORS Configuration**: Hardcoded to `localhost:8080` for frontend communication
- **Database Connection**: Single connection pool, no connection retries or failover
- **Static Assets**: Manual Tailwind CSS compilation required for styling updates
