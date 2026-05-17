# Streamforge API

Streamforge API is the backend service for the Streamforge platform, providing a robust RESTful interface consumed by the frontend application in `apps/web`.

## Technologies

- **Core**: [Rust](https://www.rust-lang.org/) (Edition 2024)
- **Web Framework**: [Axum](https://github.com/tokio-rs/axum)
- **Async Runtime**: [Tokio](https://tokio.rs/)
- **Database**: [PostgreSQL](https://www.postgresql.org/) (via [SQLx](https://github.com/launchbadge/sqlx))
- **Object Storage**: S3-compatible storage (connected via AWS SDK for Rust). In this project, we use [rustfs](https://github.com/wondelai/rustfs) as an S3 alternative compatible.
- **Logging**: [Tracing](https://github.com/tokio-rs/tracing)

## Architecture

This project adheres to **Clean Architecture** principles to ensure high maintainability, testability, and independence from external frameworks and databases.

### Layer Structure

- **Domain/Entities**: Located in `modules/*/models.rs`. Contains the core business logic and entities.
- **Use Cases/Services**: Located in `modules/*/service.rs`. Orchestrates the flow of data to and from entities.
- **Interface Adapters**:
    - **Controllers**: Located in `modules/*/controller.rs`. Translates HTTP requests into use case inputs.
    - **Repositories**: Located in `modules/*/repository.rs`. Abstracts data persistence.
- **Frameworks & Drivers**: The outermost layer, including Axum routers (`router.rs`), SQLx for database access, and AWS SDK for S3.

## Project Structure

```text
apps/api/
├── migrations/       # SQLx database migrations
├── src/
│   ├── config.rs     # Environment configuration
│   ├── lib.rs        # Library root (module exports)
│   ├── main.rs       # Binary entry point & composition root
│   ├── modules/      # Domain-specific modules (e.g., books, videos)
│   ├── shared/       # Shared utilities and AppState
│   └── storage/      # Database and S3 clients
└── target/           # Rust build artifacts
```

## Getting Started

### Prerequisites

- Rust (latest stable)
- Docker (for PostgreSQL and rustfs)
- `sqlx-cli` (optional, for managing migrations)

### Configuration

Copy the `example.env` file to `.env` and update the variables accordingly:

```bash
cp apps/api/example.env apps/api/.env
```

### Running the API

From the workspace root, use Nx:

```bash
pnpm nx dev api
```

Or using standard cargo commands inside `apps/api`:

```bash
cargo run
```

## Testing

The project uses a mix of unit tests for business logic and integration tests.

Run all tests via Nx:

```bash
pnpm nx test api
```

Or via cargo:

```bash
cargo test
```

## Building

To create a release build:

```bash
pnpm nx build api
```
