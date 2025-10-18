# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

A Rust REST API backend built with Axum and PostgreSQL. The application provides CRUD operations for a trading system.

## Development Commands

### Build and Run
```bash
cargo build              # Build the project
cargo run                # Run the server (default: 127.0.0.1:7878)
cargo watch -c -x check  # Auto-rebuild on file changes (requires cargo-watch)
```

### Testing
```bash
cargo test                    # Run all tests
cargo test -- --nocapture     # Run tests with output
```

Tests use `tower::ServiceExt::oneshot()` to test the router without starting a real HTTP server. The test database is configured via `DATABASE_URL` in `.env`.

### Database Migrations
```bash
sqlx migrate add <name>       # Create a new migration
sqlx migrate run              # Apply pending migrations
sqlx prepare                  # Generate query metadata for offline mode (fixes IDE errors with sqlx macros)
```

The project uses sqlx compile-time checked queries. If rust-analyzer shows errors on `sqlx::query_as!` macros, either:
1. Ensure `.cargo/config.toml` has the correct `DATABASE_URL`
2. Run `sqlx prepare` to generate offline query metadata

## Architecture

### Module Structure

- **`main.rs`**: Entry point - sets up database connection pool, TCP listener, and starts the Axum server
- **`create_app.rs`**: Router configuration - defines all HTTP routes and wires them to handlers
- **`models.rs`**: Shared data structures (request/response DTOs) used across handlers
- **`handlers/`**: HTTP handler functions organized by resource
  - Each handler module (e.g., `trades.rs`) contains CRUD operations for that resource
  - Handlers use `State<PgPool>` extraction for database access
  - All responses follow `{"success": bool, "data": ..., "message": ...}` format

### Database Layer

- Uses sqlx with PostgreSQL
- Connection pool (`PgPool`) is passed via Axum's state management
- Queries use `sqlx::query_as!` macro for compile-time SQL verification
- Schema managed through sqlx migrations in `migrations/` directory

### Error Handling

Handlers return `Result<(StatusCode, String), (StatusCode, String)>` where both success and error cases return JSON strings. Database errors are mapped to HTTP 500 with error details in the response body.

## Configuration

Environment variables (`.env`):
- `DATABASE_URL`: PostgreSQL connection string (format: `postgres://user:pass@host:port/db`)
- `SERVER_ADDRESS`: Server bind address (default: `127.0.0.1:3000`)

The `.cargo/config.toml` also contains `DATABASE_URL` for rust-analyzer to validate sqlx macros during development.

## Adding New Handlers

1. Create a new module file in `src/handlers/` (e.g., `src/handlers/users.rs`)
2. Add request/response structs to `src/models.rs`
3. Declare the module in `src/handlers/mod.rs`: `pub mod users;`
4. Wire routes in `src/create_app.rs`: `.route("/users", get(handlers::users::get))`
5. Handlers should follow the pattern: extract `State<PgPool>`, run sqlx query, return JSON response

## Database Note

PostgreSQL identifiers with dashes (e.g., `current-c`) require quoting. Use underscores instead (e.g., `current_c`) to avoid issues.
