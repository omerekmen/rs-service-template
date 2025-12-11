# Rust Microservice Template

A production-ready Rust microservice template implementing Clean Architecture and Domain-Driven Design (DDD) principles, complete with DevOps infrastructure and a reference User management implementation.

## 🚀 Features

### Architecture & Design
- **Clean Architecture**: Proper layer separation (Domain → Application → Infrastructure → Presentation)
- **Domain-Driven Design**: Value objects, entities, aggregates, and repository pattern
- **Port/Adapter Pattern**: Infrastructure-agnostic domain layer with pluggable adapters
- **Dependency Injection**: Proper IoC with Actix Web's app_data

### Tech Stack
- **Language**: Rust 1.85+ (edition 2024)
- **Web Framework**: Actix Web 4.12+ with async/await
- **Database**: PostgreSQL with SQLx (compile-time query verification)
- **Cache**: Redis with deadpool connection pooling
- **Serialization**: Serde for JSON/TOML
- **Logging**: Tracing with structured logging
- **Testing**: Built-in unit and integration tests

### DevOps Ready
- **Docker**: Multi-stage builds with musl for 5-10MB images
- **Kubernetes**: Complete manifests for dev/staging/prod environments
- **CI/CD**: GitHub Actions with matrix builds and auto-deployment
- **Database Migrations**: SQLx migrations with auto-update triggers
- **Monitoring**: Health checks, logging, metrics-ready

## 📂 Project Structure

```
rs-service-template/
├── crates/                     # Core library crates (Clean Architecture layers)
│   ├── shared/                 # Foundation: errors, config, types, defaults
│   ├── domain/                 # Business logic: entities, value objects, repository traits
│   ├── application/            # Use cases: DTOs, services, business orchestration
│   ├── infrastructure/         # Adapters: PostgreSQL, Redis, database migrations
│   └── presentation/           # HTTP: handlers, routes, request/response types
├── services/                   # Executable services
│   ├── api/                    # HTTP API service (main entry point)
│   ├── worker/                 # Background worker (placeholder)
│   ├── grpc/                   # gRPC service (placeholder)
│   └── cli/                    # CLI tool (placeholder)
├── config/                     # Environment-specific configurations
│   ├── dev.toml                # Development settings
│   ├── staging.toml            # Staging settings
│   └── prod.toml               # Production settings
├── k8s/                        # Kubernetes manifests
│   ├── dev/                    # Development environment
│   ├── staging/                # Staging environment
│   └── prod/                   # Production environment
├── scripts/                    # Helper scripts
├── .github/workflows/          # CI/CD pipelines
├── Dockerfile                  # Multi-stage Docker build
├── docker-compose.yml          # Local development environment
└── Makefile                    # Common commands
```

## 🎯 Reference Implementation: User Management

The template includes a complete User management system as a reference:

### API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | `/api/v1/users` | Create a new user |
| GET | `/api/v1/users` | List users (paginated) |
| GET | `/api/v1/users/:id` | Get user by ID |
| GET | `/api/v1/users/username/:username` | Get user by username |
| PUT | `/api/v1/users/:id` | Update user |
| DELETE | `/api/v1/users/:id` | Delete user |

### Example Usage

```bash
# Create a user
curl -X POST http://localhost:8080/api/v1/users \
  -H "Content-Type: application/json" \
  -d '{
    "username": "johndoe",
    "email": "john@example.com",
    "full_name": "John Doe"
  }'

# Get user by ID
curl http://localhost:8080/api/v1/users/<user-id>

# List users (paginated)
curl "http://localhost:8080/api/v1/users?limit=20&offset=0"

# Update user
curl -X PUT http://localhost:8080/api/v1/users/<user-id> \
  -H "Content-Type: application/json" \
  -d '{"full_name": "John H. Doe"}'

# Delete user
curl -X DELETE http://localhost:8080/api/v1/users/<user-id>
```

## 🏃 Quick Start

### Prerequisites
- Rust 1.85+ (`rustup`)
- PostgreSQL 14+
- Redis 7+
- Docker & Docker Compose (for containerized setup)

### Local Development

1. **Clone and setup:**
   ```bash
   git clone <repository-url>
   cd rs-service-template
   cp config/dev.toml.example config/dev.toml  # If needed
   ```

2. **Start dependencies:**
   ```bash
   # Option 1: Using docker-compose (recommended)
   make docker-compose-up

   # Option 2: Manually
   # Start PostgreSQL on localhost:5432
   # Start Redis on localhost:6379
   ```

3. **Run migrations:**
   ```bash
   make migrate-up
   # Or manually:
   # sqlx migrate run --source crates/infrastructure/migrations
   ```

4. **Start the API:**
   ```bash
   cargo run -p api
   # API will be available at http://localhost:8080
   ```

5. **Test the API:**
   ```bash
   curl http://localhost:8080/health
   curl http://localhost:8080/api/v1/users
   ```

### Using Makefile

```bash
# Build commands
make build                  # Build entire workspace
make build-service SERVICE=api
make test                   # Run all tests
make fmt                    # Format code
make clippy                 # Run linter

# Docker commands
make docker-build SERVICE=api
make docker-push SERVICE=api
make docker-build-all       # Build all services

# Docker Compose
make docker-compose-up      # Start all services
make docker-compose-logs    # View logs
make docker-compose-down    # Stop services

# Kubernetes
make k8s-apply-dev          # Deploy to dev
make k8s-logs-dev           # View logs
make k8s-restart-api-dev    # Restart API

# Database migrations
make migrate-up             # Run migrations
make migrate-down           # Rollback
make migrate-create NAME=add_feature
```

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run specific crate tests
cargo test -p domain
cargo test -p application
cargo test -p infrastructure

# Run with output
cargo test -- --nocapture

# Run with logging
RUST_LOG=debug cargo test
```

**Current Test Coverage:**
- Domain: 9 tests (value objects, entities)
- Application: 2 tests (use cases with mock repository)
- Total: 11 tests, all passing ✅

## 🐳 Docker

### Build Docker Image

```bash
# Build specific service
docker build --build-arg SERVICE_NAME=api -t rs-service-api:latest .

# Or using Makefile
make docker-build SERVICE=api
```

### Image Sizes
- **Builder stage**: ~1.5GB (rust:1.85-alpine + dependencies)
- **Final image**: 5-10MB (distroless with static binary)
- **Size reduction**: 99% from standard rust image

### Multi-Service Support

```bash
# Build all services
for service in api worker grpc cli; do
  docker build --build-arg SERVICE_NAME=$service -t rs-service-$service:latest .
done
```

## ☸️ Kubernetes

### Deployment

```bash
# Deploy to development
make k8s-apply-dev

# Deploy to staging
make k8s-apply-staging

# Deploy to production (requires confirmation)
make k8s-apply-prod
```

### Environments

- **Dev**: In-cluster PostgreSQL/Redis, 2 API replicas, minimal resources
- **Staging**: In-cluster databases, 3 API replicas, ingress with TLS
- **Prod**: External RDS/ElastiCache, 5-50 API replicas (HPA), production ingress

### Health Checks

```bash
# Check deployment health
./scripts/health_check.sh dev

# View logs
make k8s-logs-dev

# Restart deployment
make k8s-restart-api-dev
```

## 🔧 Configuration

Configuration is loaded with the following precedence (highest to lowest):

1. **Environment variables**: `APP__SECTION__KEY` format
2. **TOML files**: `config/{dev|staging|prod}.toml`
3. **Default values**: Defined in `crates/shared/src/defaults/`

### Key Environment Variables

```bash
# Environment
APP_ENV=dev                    # dev, staging, prod

# Server
APP__SERVER__HOST=0.0.0.0
APP__SERVER__PORT=8080
APP__SERVER__WORKERS=4

# Database
APP__DATABASE__CONNECTION_STRING=postgresql://user:pass@localhost/db
APP__DATABASE__MAX_CONNECTIONS=20
APP__DATABASE__RUN_MIGRATIONS=true

# Cache
APP__CACHE__URL=redis://localhost:6379

# Logging
RUST_LOG=info,api=debug
```

See `.env.example` for complete list.

## 📦 Adding New Features

Follow Clean Architecture principles:

### 1. Domain Layer
```rust
// crates/domain/src/entities/product.rs
pub struct Product { /* ... */ }

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
pub struct CreateProductRequest { /* ... */ }
pub struct ProductResponse { /* ... */ }

// crates/application/src/services/product_service.rs
pub struct ProductService {
    product_repository: Arc<dyn ProductRepository>,
}
```

### 3. Infrastructure Layer
```rust
// crates/infrastructure/src/repositories/postgres_product_repository.rs
pub struct PostgresProductRepository { /* ... */ }

impl ProductRepository for PostgresProductRepository { /* ... */ }

// crates/infrastructure/migrations/YYYYMMDDHHMMSS_create_products_table.sql
CREATE TABLE products ( /* ... */ );
```

### 4. Presentation Layer
```rust
// crates/presentation/src/handlers/product_handlers.rs
pub async fn create_product(/* ... */) -> Result<HttpResponse> { /* ... */ }

// crates/presentation/src/routes/product.rs
pub fn configure(cfg: &mut web::ServiceConfig) { /* ... */ }
```

### 5. Wire Dependencies
```rust
// services/api/src/http_server.rs
let product_repository = Arc::new(PostgresProductRepository::new(db_pool.clone()));
let product_service = web::Data::new(ProductService::new(product_repository));

App::new()
    .app_data(product_service.clone())
    .configure(configure_routes)
```

## 🔐 Security Features

- **Distroless runtime**: No shell, minimal attack surface
- **Non-root user**: Runs as uid 65532
- **Static linking**: No runtime dependencies
- **Input validation**: Via value objects and DTOs
- **SQL injection protection**: Parameterized queries with SQLx
- **Secrets management**: K8s Secrets, never committed to git
- **TLS termination**: At ingress layer with cert-manager
- **Rate limiting**: Configured at ingress layer

## 📊 Performance

- **Image size**: 5-10MB (99% reduction from standard)
- **Build time**: 3-5min first build, 30-60s cached rebuild
- **Startup time**: <1 second
- **Request latency**: Sub-millisecond for cached queries
- **Throughput**: 10K+ req/s per instance (benchmark with your workload)

## 🚀 CI/CD

### GitHub Actions Workflows

- **Matrix builds**: All services built in parallel
- **Layer caching**: 50-80% faster rebuilds
- **Automated deployments**:
  - `master` branch → dev environment
  - `release` branch → staging environment
  - `v*` tags → production (manual approval)

### Required Secrets

- `KUBECONFIG_DEV`: Base64-encoded kubeconfig for dev cluster
- `KUBECONFIG_STAGING`: Base64-encoded kubeconfig for staging
- `KUBECONFIG_PROD`: Base64-encoded kubeconfig for prod

## 📚 Documentation

- **CLAUDE.md**: Detailed guidance for Claude Code (AI assistant)
- **README-DEVOPS.md**: Complete DevOps guide (400+ lines)
- **IMPLEMENTATION_SUMMARY.md**: Reference implementation walkthrough
- **API Documentation**: (TODO: Add OpenAPI/Swagger)

## 🤝 Contributing

1. Follow Clean Architecture principles
2. Write tests for new features
3. Update documentation
4. Run `make fmt` and `make clippy` before committing
5. Ensure all tests pass: `make test`

## 📝 License

[Your License Here]

## 🙏 Credits

Built with:
- [Actix Web](https://actix.rs/) - Web framework
- [SQLx](https://github.com/launchbadge/sqlx) - SQL toolkit
- [Serde](https://serde.rs/) - Serialization framework
- [Tokio](https://tokio.rs/) - Async runtime
- And many other amazing Rust crates

---

**For detailed implementation guidance, see [CLAUDE.md](CLAUDE.md)**

**For DevOps setup and deployment, see [README-DEVOPS.md](README-DEVOPS.md)**

**For reference implementation walkthrough, see [IMPLEMENTATION_SUMMARY.md](IMPLEMENTATION_SUMMARY.md)**
