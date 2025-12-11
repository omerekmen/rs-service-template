# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a production-ready Rust microservice template implementing Clean Architecture and Domain-Driven Design (DDD) principles. The project uses a Cargo workspace with multiple crates organized in layers (Domain, Application, Infrastructure, Presentation) plus a shared foundation layer.

**Includes Reference Implementation**: A complete User management system demonstrating all Clean Architecture layers working together.

**Key Architecture Principle**: Dependencies flow inward (Presentation → Application → Domain). Infrastructure implements ports/adapters defined in Application/Domain. The domain layer has no external dependencies.

## Tech Stack

- **Language**: Rust 1.85+ (edition 2024)
- **Web Framework**: Actix Web 4.12+ with async/await
- **Database**: PostgreSQL with SQLx (compile-time query verification)
- **Cache**: Redis with deadpool connection pooling
- **Configuration**: Config crate with TOML files
- **Error Handling**: Custom AppError enum with Actix Web integration
- **Logging**: Tracing with structured logging
- **Testing**: Tokio test runtime with 11 passing tests
- **Runtime**: Tokio async runtime

## Repository Structure

```
rs-service-template/
├── crates/                     # Core library crates (Clean Architecture layers)
│   ├── shared/                 # Foundation: errors, config, types, defaults
│   │   ├── error.rs           # ✅ AppError enum (11+ variants)
│   │   ├── types.rs           # ✅ UserId newtype
│   │   └── config/            # ✅ TOML-based configuration
│   ├── domain/                 # Business logic layer
│   │   ├── value_objects/     # ✅ Email, Username (validated)
│   │   ├── entities/          # ✅ User entity
│   │   └── repositories/      # ✅ Repository traits (ports)
│   ├── application/            # Use cases layer
│   │   ├── dtos/              # ✅ Request/Response DTOs
│   │   └── services/          # ✅ UserService (6 use cases)
│   ├── infrastructure/         # Technical implementation
│   │   ├── repositories/      # ✅ PostgresUserRepository
│   │   ├── migrations/        # ✅ Users table migration
│   │   ├── database/          # ✅ PostgreSQL + MSSQL support
│   │   └── cache/             # ✅ Redis connection pooling
│   └── presentation/           # HTTP interface
│       ├── handlers/          # ✅ User HTTP handlers
│       ├── routes/            # ✅ Route configuration
│       └── states/            # ✅ Actix Web app state
├── services/                   # Executable services
│   ├── api/                    # ✅ Main HTTP API (fully implemented)
│   ├── worker/                 # Placeholder
│   ├── grpc/                   # Placeholder
│   └── cli/                    # Placeholder
├── config/                     # Environment configurations
│   ├── dev.toml                # ✅ Development settings
│   ├── staging.toml            # ✅ Staging settings
│   └── prod.toml               # ✅ Production settings
├── k8s/                        # Kubernetes manifests
│   ├── dev/                    # ✅ 11 manifest files
│   ├── staging/                # ✅ 12 manifest files
│   └── prod/                   # ✅ 9 manifest files
├── scripts/                    # Helper scripts
│   ├── deploy.sh               # ✅ Deployment automation
│   ├── init_db.sh              # ✅ Database initialization
│   ├── health_check.sh         # ✅ Health verification
│   └── build_all.sh            # ✅ Multi-service build
├── .github/workflows/          # CI/CD
│   └── cd.yml                  # ✅ Matrix builds + auto-deploy
├── Dockerfile                  # ✅ Multi-stage (5-10MB images)
├── docker-compose.yml          # ✅ Local dev environment
├── Makefile                    # ✅ 30+ common commands
└── README-DEVOPS.md            # ✅ Complete DevOps guide
```

## Workspace Dependency Flow

```
domain <- application <- presentation
  ^           ^              |
  |           |              v
  +------- infrastructure ---+
```

All layers depend on `shared`. Infrastructure implements ports defined in Domain. Presentation (composition root) wires everything together.

## Common Development Commands

### Building and Running

```bash
# Build entire workspace
cargo build

# Build specific service
cargo build -p api
cargo build -p worker

# Run the API service (main entry point)
cargo run -p api

# Run with specific environment
APP_ENV=dev cargo run -p api
APP_ENV=staging cargo run -p api
APP_ENV=prod cargo run -p api

# Run with debug logging
RUST_LOG=debug cargo run -p api

# Build release
cargo build --release
```

### Testing

```bash
# Run all tests in workspace (11 tests)
cargo test

# Run tests with output
RUST_LOG=debug cargo test -- --nocapture

# Run tests for specific crate
cargo test -p domain        # 9 tests (value objects + entities)
cargo test -p application   # 2 tests (use cases with mock)
cargo test -p infrastructure
cargo test -p presentation

# Run specific test
cargo test test_create_user
cargo test test_valid_email
```

### Code Quality

```bash
# Format all code
cargo fmt

# Check formatting without modifying
cargo fmt -- --check

# Run Clippy linter
cargo clippy

# Clippy with all warnings
cargo clippy -- -W clippy::all

# Check compilation without building
cargo check

# Check specific package
cargo check -p api
```

### Database Operations

```bash
# Run migrations (auto-runs on startup if config.database.run_migrations = true)
# Migrations are in crates/infrastructure/migrations/

# Create new migration
sqlx migrate add <migration_name> --source crates/infrastructure/migrations

# Run migrations manually
sqlx migrate run --source crates/infrastructure/migrations --database-url <DATABASE_URL>

# Revert last migration
sqlx migrate revert --source crates/infrastructure/migrations --database-url <DATABASE_URL>

# For offline mode (CI/CD)
cargo sqlx prepare --workspace
```

### Docker & Kubernetes

```bash
# Docker Compose (local development)
make docker-compose-up      # Start postgres, redis, api
make docker-compose-logs    # View logs
make docker-compose-down    # Stop services

# Docker builds
make docker-build SERVICE=api
make docker-build-all       # Build all 4 services

# Kubernetes
make k8s-apply-dev          # Deploy to dev
make k8s-logs-dev           # View logs
make k8s-restart-api-dev    # Restart deployment
```

### Managing Dependencies

```bash
# Add workspace dependency (preferred)
cargo add <crate> -p <package-name>

# Examples:
cargo add tokio -p api
cargo add serde -p shared

# Add dev dependency
cargo add --dev <crate> -p <package-name>

# Update dependencies
cargo update
```

## Configuration System

Configuration is loaded from TOML files in the `config/` directory based on the `APP_ENV` environment variable:

- `APP_ENV=dev` → loads `config/dev.toml` (default)
- `APP_ENV=staging` → loads `config/staging.toml`
- `APP_ENV=prod` → loads `config/prod.toml`

**Configuration precedence** (highest to lowest):
1. Environment variables (override TOML values) - Use `APP__SECTION__KEY` format
2. TOML configuration files
3. Default values from `shared::defaults`

### Configuration Example

```bash
# Environment variable override
APP__SERVER__PORT=3000 cargo run -p api

# Database connection override
APP__DATABASE__CONNECTION_STRING=postgresql://user:pass@localhost/mydb cargo run -p api
```

### Configuration Modules (shared crate)

The `shared` crate provides configuration structs:
- `AppConfig` - Root configuration (composes all other configs)
- `ServerConfig` - HTTP server settings (host, port, workers)
- `DatabaseConfig` - PostgreSQL/MSSQL connection settings
- `CacheConfig` - Redis connection settings

All configs implement a `load(env: &str)` method that merges TOML files with defaults.

## Shared Crate (Foundation)

The `shared` crate is the foundation for all other crates and contains:

### Error Handling (`shared::error`)
**Status**: ✅ **FULLY IMPLEMENTED**

- `AppError` enum with 11+ error variants:
  - `ValidationError`, `InvalidEmail`, `InvalidUsername`
  - `NotFound`, `AlreadyExists`, `Unauthorized`, `Forbidden`
  - `DatabaseError`, `CacheError`, `InternalError`, `ConfigurationError`
- `AppResult<T>` type alias for `Result<T, AppError>`
- Automatic conversion from infrastructure errors (sqlx::Error, redis::RedisError)
- Actix Web integration: HTTP status mapping + JSON error responses
- Feature flags: `actix-integration`, `sqlx-integration`, `redis-integration`

**Import Pattern**:
```rust
use shared::{AppError, AppResult, UserId};
```

### Types (`shared::types`)
- `UserId` - Newtype wrapper around UUID with serde support
- Additional ID types can be added following the same pattern

### Configuration (`shared::config`)
- Type-safe configuration structs with serde
- Environment-based loading (dev/staging/prod)
- Merges TOML files with sensible defaults

### Defaults (`shared::defaults`)
- Default values for all configuration sections
- Used as fallback when TOML values are missing

## Domain Layer

**Status**: ✅ **FULLY IMPLEMENTED** with User management example

### Value Objects (`domain::value_objects`)

#### Email
- Regex validation (RFC-compliant)
- Case normalization (lowercase)
- Max 255 characters
- Methods: `domain()`, `local_part()`
- **Tests**: 4 tests covering validation and normalization

#### Username
- 3-30 characters
- Alphanumeric + underscore/hyphen only
- No spaces or special characters
- **Tests**: 2 tests covering validation rules

### Entities (`domain::entities`)

#### User
- Properties: `id`, `username`, `email`, `full_name`, `status`, `created_at`, `updated_at`
- Status enum: `Active`, `Inactive`, `Suspended`
- Business methods:
  - `new()` - Create new user
  - `from_persistence()` - Reconstruct from database
  - `update_username()`, `update_email()`, `update_full_name()`
  - `activate()`, `deactivate()`, `suspend()`
  - `is_active()` - Status check
- All mutations update `updated_at` timestamp
- **Tests**: 3 tests covering creation, updates, status changes

### Repository Traits (`domain::repositories`)

#### UserRepository
```rust
#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, user: &User) -> AppResult<()>;
    async fn find_by_id(&self, id: UserId) -> AppResult<Option<User>>;
    async fn find_by_username(&self, username: &Username) -> AppResult<Option<User>>;
    async fn find_by_email(&self, email: &Email) -> AppResult<Option<User>>;
    async fn update(&self, user: &User) -> AppResult<()>;
    async fn delete(&self, id: UserId) -> AppResult<()>;
    async fn username_exists(&self, username: &Username) -> AppResult<bool>;
    async fn email_exists(&self, email: &Email) -> AppResult<bool>;
    async fn list(&self, limit: i64, offset: i64) -> AppResult<Vec<User>>;
    async fn count(&self) -> AppResult<i64>;
}
```

This is a **port** - the domain defines the interface, infrastructure provides the implementation.

## Application Layer

**Status**: ✅ **FULLY IMPLEMENTED** with User management use cases

### DTOs (`application::dtos`)

- `CreateUserRequest` - username, email, full_name (optional)
- `UpdateUserRequest` - All fields optional
- `UserResponse` - Complete user data with timestamps
- `UserListResponse` - Paginated list with metadata (total, limit, offset)

### Services (`application::services`)

#### UserService
Six use cases orchestrating domain logic:

1. **create_user**: Create new user
   - Validates username/email via value objects
   - Enforces uniqueness (business rule)
   - Returns `UserResponse`

2. **get_user**: Retrieve by ID
   - Returns 404 if not found

3. **get_user_by_username**: Retrieve by username
   - Returns 404 if not found

4. **update_user**: Update user fields
   - Enforces uniqueness on username/email changes
   - Only updates provided fields

5. **delete_user**: Delete by ID
   - Verifies existence before deletion

6. **list_users**: Paginated listing
   - Validates pagination parameters (limit 1-100)
   - Returns total count + users

**Business Rules Enforced**:
- Username must be unique
- Email must be unique
- Pagination limits (1-100 items per page)
- All validation via value objects

**Tests**: 2 integration tests with mock repository

## Infrastructure Layer

**Status**: ✅ **FULLY IMPLEMENTED**

### Database
- PostgreSQL support via sqlx (primary)
- SQL Server support via sqlx mssql backend (existing, not used in example)
- Connection pooling with `PgPool` and `MssqlPool`
- Migrations in `crates/infrastructure/migrations/`

**Database Pool Creation**:
```rust
use infrastructure::database::postgres::create_postgres_pool;
use infrastructure::database::mssql::create_mssql_pool;
```

### Repositories (`infrastructure::repositories`)

#### PostgresUserRepository
**Status**: ✅ **FULLY IMPLEMENTED**

Implements `UserRepository` trait from domain:
- All 10 repository methods implemented
- Uses sqlx for compile-time query verification
- Converts between domain entities and database rows
- Status enum mapping (active/inactive/suspended)
- Proper error conversion (sqlx::Error → AppError)

### Migrations

**File**: `crates/infrastructure/migrations/20250101000000_create_users_table.sql`

Creates users table with:
- UUID primary key
- Unique constraints on username and email
- Status enum with CHECK constraint
- Indexes on username, email, status, created_at
- Auto-update trigger for `updated_at` timestamp

### Cache
- Redis support via deadpool-redis
- Pool creation in `infrastructure::cache::redis::create_redis_pool`

## Presentation Layer (HTTP)

**Status**: ✅ **FULLY IMPLEMENTED** with User endpoints

### Routes (`presentation::routes`)

**Implemented**:
- `health.rs` - Health check endpoint (`GET /health`)
- `user.rs` - User management endpoints (6 endpoints under `/api/v1/users`)
- `tenant.rs` - Tenant management endpoints (placeholder)

### Handlers (`presentation::handlers`)

#### User Handlers (`user_handlers.rs`)

| Method | Endpoint | Handler | Response |
|--------|----------|---------|----------|
| POST | `/api/v1/users` | create_user | 201 Created |
| GET | `/api/v1/users` | list_users | 200 OK |
| GET | `/api/v1/users/:id` | get_user | 200 OK |
| GET | `/api/v1/users/username/:username` | get_user_by_username | 200 OK |
| PUT | `/api/v1/users/:id` | update_user | 200 OK |
| DELETE | `/api/v1/users/:id` | delete_user | 204 No Content |

**Query Parameters**:
- `limit` (default: 20, max: 100)
- `offset` (default: 0)

**Error Responses**:
- 400 Bad Request - Validation error
- 404 Not Found - User not found
- 409 Conflict - Username/email already exists
- 500 Internal Server Error - Server error

All errors return JSON:
```json
{
  "error": {
    "message": "User with ID abc not found",
    "code": 404
  }
}
```

### States
Actix Web application state management in `crates/presentation/src/states/`:
- `database.rs` - Database pool state
- `cache.rs` - Redis pool state
- `jwt.rs` - JWT configuration state (placeholder)
- `email.rs` - Email configuration state (placeholder)

### Main Entry Point

The `services/api` crate is the main HTTP service:

1. Loads configuration via `AppConfig::load(&env)`
2. Initializes database pool
3. Creates repository implementations (PostgresUserRepository)
4. Creates application services (UserService)
5. Injects services into Actix Web via `app_data`
6. Starts HTTP server with all routes

**Dependency Injection**:
```rust
// services/api/src/http_server.rs
let db_pool = create_postgres_pool(config.database.clone()).await?;
let user_repository = Arc::new(PostgresUserRepository::new(db_pool));
let user_service = web::Data::new(UserService::new(user_repository));

App::new()
    .app_data(user_service.clone())
    .configure(configure_routes)
```

## DevOps Infrastructure

**Status**: ✅ **FULLY IMPLEMENTED**

### Docker
- **Dockerfile**: Multi-stage build (rust:1.85-alpine → distroless)
- **Final image size**: 5-10MB (99% reduction)
- **Build command**: `docker build --build-arg SERVICE_NAME=api -t api:latest .`
- **Features**: BuildKit caching, static linking (musl), security (distroless + nonroot)

### Docker Compose
- **File**: `docker-compose.yml`
- **Services**: postgres (16-alpine), redis (7-alpine), api
- **Features**: Health-based startup ordering, persistent volumes, auto-migrations
- **Usage**: `make docker-compose-up`

### Kubernetes
- **32 manifest files** across 3 environments (dev/staging/prod)
- **Dev**: In-cluster PostgreSQL/Redis, 2 API replicas
- **Staging**: In-cluster databases, 3 replicas, ingress with TLS
- **Prod**: External RDS/ElastiCache, 5-50 replicas (HPA), production ingress
- **Features**: Migrations as Jobs, HPA, health checks, ConfigMaps/Secrets

### CI/CD
- **File**: `.github/workflows/cd.yml`
- **Matrix builds**: All 4 services in parallel
- **Auto-deploy**: master→dev, release→staging, tags→prod (manual approval)
- **Features**: Layer caching, migration automation, rollout verification

### Scripts
- `scripts/deploy.sh` - Build, push, deploy
- `scripts/init_db.sh` - Run migrations
- `scripts/health_check.sh` - Verify deployment
- `scripts/build_all.sh` - Build all services

### Makefile
30+ targets for common operations:
- Build, test, format, clippy
- Docker operations (build, push, build-all)
- Docker Compose (up, down, logs, rebuild)
- Kubernetes (apply, delete, logs, restart for all envs)
- Database migrations (up, down, create)

## Port/Adapter Pattern

**Implemented Example**: User Repository

**Port** (defined in Domain layer):
```rust
// crates/domain/src/repositories/user_repository.rs
#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, user: &User) -> AppResult<()>;
    // ... 9 more methods
}
```

**Adapter** (implemented in Infrastructure layer):
```rust
// crates/infrastructure/src/repositories/postgres_user_repository.rs
pub struct PostgresUserRepository {
    pool: PgPool,
}

impl UserRepository for PostgresUserRepository {
    async fn create(&self, user: &User) -> AppResult<()> {
        // PostgreSQL-specific implementation
    }
}
```

**Composition** (in Services):
```rust
// services/api/src/http_server.rs
let user_repository = Arc::new(PostgresUserRepository::new(db_pool));
let user_service = web::Data::new(UserService::new(user_repository));
```

**Benefits**:
- Domain is database-agnostic
- Easy to swap implementations (PostgreSQL → MSSQL → In-Memory)
- Testability (use mock repository in tests)

## Authentication & Authorization

**Current Status**: Infrastructure ready but not implemented

**Planned features**:
- JWT access/refresh tokens
- JWKS-based signature verification
- OAuth2/OIDC flows (Auth Code + PKCE)
- RBAC (Role-Based Access Control)
- Multi-tenant isolation via `tenant_id` claims
- Rate limiting and brute-force protection

**Recommendation**: Follow the User example when implementing:
1. Define Auth domain entities/value objects
2. Create authentication use cases in application layer
3. Implement JWT/OAuth adapters in infrastructure
4. Add auth middleware in presentation layer

## API Versioning

**Strategy**: Path-based versioning `/api/v1/...`

**Current Implementation**:
- All user endpoints under `/api/v1/users`
- Configured in `services/api/src/route_configuration.rs`

**Future**:
- Breaking changes require new major version (`/api/v2/...`)
- Deprecation warnings via response headers
- Optional RBAC-based version access control

## Testing Strategy

**Current Implementation**:
- **Domain tests**: 9 tests (value objects + entities)
- **Application tests**: 2 tests (use cases with mock repository)
- **Total**: 11 tests, all passing ✅

**Test Categories**:
- **Domain tests**: Pure unit tests, no external dependencies
- **Application tests**: Use in-memory fakes for ports
- **Infrastructure tests**: Integration tests with real databases (not yet implemented)
- **E2E tests**: Full HTTP tests (not yet implemented)

**Recommended additions**:
- `testcontainers` for database/service containers
- `reqwest` for HTTP client in E2E tests
- `insta` for snapshot testing
- `proptest` for property-based testing

## Project State

**Current Status**: ✅ **PRODUCTION READY** with complete reference implementation

✅ **Fully Implemented**:
- Clean Architecture with all layers working together
- User management system (domain → application → infrastructure → presentation)
- Comprehensive error handling with AppError
- Database migrations with auto-update triggers
- Repository pattern with PostgreSQL implementation
- HTTP API with 6 endpoints
- Configuration system (dev/staging/prod)
- DevOps infrastructure (Docker, K8s, CI/CD)
- 11 passing tests (domain + application)

📦 **Available as Template**:
- Worker, gRPC, CLI services (placeholder structure)
- Authentication/authorization (infrastructure ready)
- Event bus/message queue (planned)
- Additional entities (follow User example)

## Adding New Features

Follow the User management implementation as a reference:

### 1. Domain Layer
```rust
// crates/domain/src/value_objects/product_name.rs
pub struct ProductName(String);
impl ProductName {
    pub fn new(name: impl Into<String>) -> Result<Self, AppError> { ... }
}

// crates/domain/src/entities/product.rs
pub struct Product { ... }

// crates/domain/src/repositories/product_repository.rs
#[async_trait]
pub trait ProductRepository: Send + Sync {
    async fn create(&self, product: &Product) -> AppResult<()>;
    // ...
}
```

### 2. Application Layer
```rust
// crates/application/src/dtos/product_dto.rs
pub struct CreateProductRequest { ... }
pub struct ProductResponse { ... }

// crates/application/src/services/product_service.rs
pub struct ProductService {
    product_repository: Arc<dyn ProductRepository>,
}

impl ProductService {
    pub async fn create_product(&self, request: CreateProductRequest) -> AppResult<ProductResponse> {
        // Validate input
        // Check business rules
        // Create entity
        // Save via repository
    }
}
```

### 3. Infrastructure Layer
```rust
// crates/infrastructure/migrations/YYYYMMDDHHMMSS_create_products_table.sql
CREATE TABLE products ( ... );

// crates/infrastructure/src/repositories/postgres_product_repository.rs
pub struct PostgresProductRepository { pool: PgPool }

impl ProductRepository for PostgresProductRepository {
    async fn create(&self, product: &Product) -> AppResult<()> {
        sqlx::query("INSERT INTO products ...").execute(&self.pool).await?;
        Ok(())
    }
}
```

### 4. Presentation Layer
```rust
// crates/presentation/src/handlers/product_handlers.rs
pub async fn create_product(
    service: web::Data<ProductService>,
    request: web::Json<CreateProductRequest>,
) -> Result<HttpResponse> {
    let product = service.create_product(request.into_inner()).await?;
    Ok(HttpResponse::Created().json(product))
}

// crates/presentation/src/routes/product.rs
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/products")
            .route("", web::post().to(product_handlers::create_product))
    );
}
```

### 5. Wire Dependencies
```rust
// services/api/src/http_server.rs
let product_repository = Arc::new(PostgresProductRepository::new(db_pool.clone()));
let product_service = web::Data::new(ProductService::new(product_repository));

// services/api/src/route_configuration.rs
cfg.service(
    web::scope("/api/v1")
        .configure(user::configure)
        .configure(product::configure)
);
```

## Quick Reference: Common Patterns

### Error Handling
```rust
// Domain layer
Email::new("invalid")?  // Returns AppError::InvalidEmail

// Application layer
self.repository.find_by_id(id).await?
    .ok_or_else(|| AppError::NotFound(format!("User {} not found", id)))?

// Infrastructure layer
// sqlx::Error automatically converts to AppError::DatabaseError
```

### Repository Pattern
```rust
// Define port in domain
#[async_trait]
pub trait MyRepository: Send + Sync {
    async fn find(&self, id: MyId) -> AppResult<Option<MyEntity>>;
}

// Implement adapter in infrastructure
pub struct PostgresMyRepository { pool: PgPool }
impl MyRepository for PostgresMyRepository { ... }

// Use in application
pub struct MyService {
    repo: Arc<dyn MyRepository>,
}
```

### Value Objects
```rust
// Always validated on creation
pub struct Email(String);
impl Email {
    pub fn new(email: impl Into<String>) -> Result<Self, AppError> {
        let email = email.into();
        // Validate
        Ok(Self(email))
    }
}
```

### DTOs
```rust
// Request (deserialize from JSON)
#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
}

// Response (serialize to JSON)
#[derive(Serialize)]
pub struct UserResponse {
    pub id: UserId,
    pub username: String,
    // ...
}
```

## Documentation

- **README.md**: Getting started, quick reference
- **README-DEVOPS.md**: Complete DevOps guide (400+ lines)
- **IMPLEMENTATION_SUMMARY.md**: Reference implementation walkthrough
- **CLAUDE.md**: This file - comprehensive development guide
- **.env.example**: Environment variable template

## Support

For questions about:
- **Architecture**: See this file (CLAUDE.md)
- **DevOps**: See README-DEVOPS.md
- **Implementation**: See IMPLEMENTATION_SUMMARY.md
- **Quick Start**: See README.md
