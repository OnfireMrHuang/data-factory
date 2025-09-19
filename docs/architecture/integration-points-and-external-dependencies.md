# Integration Points and External Dependencies

## Current External Services

| Service | Purpose | Integration Type | Key Files |
| ------- | ------- | ---------------- | --------- |
| MySQL Database | Data persistence | SQLx async | `backend/src/utils/database.rs` |
| None (Yet) | ETL Processing | Future: data-engine integration | TBD |
| None (Yet) | AI Workflows | Future: data-agent integration | TBD |

## Internal Integration Points

- **Frontend ↔ Backend**: REST API on port 3000, JSON communication
- **Database Layer**: SQLx with compile-time query validation
- **Static Assets**: Dioxus serves WebAssembly to browser
- **Future Integration**: HTTP/gRPC communication with data-engine for ETL execution
