.PHONY: help build build-service test test-crate clean fmt clippy check-docker docker-build docker-push docker-build-all docker-push-all docker-compose-up docker-compose-down docker-compose-logs docker-compose-rebuild k8s-apply-dev k8s-apply-staging k8s-apply-prod k8s-delete-dev k8s-delete-staging k8s-delete-prod k8s-logs-dev k8s-logs-staging k8s-logs-prod k8s-restart-api-dev k8s-restart-api-staging k8s-restart-api-prod migrate-up migrate-down migrate-create

# Default target
.DEFAULT_GOAL := help

# Variables
REGISTRY := ghcr.io/your-org
IMAGE_NAME := rs-service-template
VERSION := $(shell git describe --tags --always --dirty 2>/dev/null || echo "dev")
SERVICE ?= api
CRATE ?= shared
NAME ?= migration

# Colors for output
GREEN := \033[0;32m
YELLOW := \033[0;33m
NC := \033[0m # No Color

# Help target
help:
	@echo "$(GREEN)Available targets:$(NC)"
	@echo ""
	@echo "$(YELLOW)Build targets:$(NC)"
	@echo "  build                 - Build all Rust services in release mode"
	@echo "  build-service         - Build specific service (SERVICE=api|worker|grpc|cli)"
	@echo "  test                  - Run all tests"
	@echo "  test-crate            - Run tests for specific crate (CRATE=shared|domain|...)"
	@echo "  clean                 - Clean build artifacts"
	@echo "  fmt                   - Format code with rustfmt"
	@echo "  clippy                - Run Clippy linter"
	@echo ""
	@echo "$(YELLOW)Docker targets:$(NC)"
	@echo "  docker-build          - Build Docker image for SERVICE (default: api)"
	@echo "  docker-push           - Push Docker image to registry"
	@echo "  docker-build-all      - Build all 4 service images"
	@echo "  docker-push-all       - Push all service images to registry"
	@echo ""
	@echo "$(YELLOW)Docker Compose targets:$(NC)"
	@echo "  docker-compose-up     - Start local dev environment with docker-compose"
	@echo "  docker-compose-down   - Stop docker-compose services"
	@echo "  docker-compose-logs   - View logs from docker-compose services"
	@echo "  docker-compose-rebuild - Rebuild and restart docker-compose services"
	@echo ""
	@echo "$(YELLOW)Kubernetes targets:$(NC)"
	@echo "  k8s-apply-dev         - Deploy to dev environment"
	@echo "  k8s-apply-staging     - Deploy to staging environment"
	@echo "  k8s-apply-prod        - Deploy to production environment"
	@echo "  k8s-delete-dev        - Delete dev namespace"
	@echo "  k8s-delete-staging    - Delete staging namespace"
	@echo "  k8s-delete-prod       - Delete prod namespace"
	@echo "  k8s-logs-dev          - View logs from dev API pods"
	@echo "  k8s-logs-staging      - View logs from staging API pods"
	@echo "  k8s-logs-prod         - View logs from prod API pods"
	@echo "  k8s-restart-api-dev   - Restart API deployment in dev"
	@echo "  k8s-restart-api-staging - Restart API deployment in staging"
	@echo "  k8s-restart-api-prod  - Restart API deployment in prod"
	@echo ""
	@echo "$(YELLOW)Database migration targets:$(NC)"
	@echo "  migrate-up            - Run pending migrations"
	@echo "  migrate-down          - Revert last migration"
	@echo "  migrate-create        - Create new migration (NAME=migration_name)"

# Build targets
build:
	@echo "$(GREEN)Building all services in release mode...$(NC)"
	cargo build --release

build-service:
	@echo "$(GREEN)Building $(SERVICE) service...$(NC)"
	cargo build --release -p $(SERVICE)

test:
	@echo "$(GREEN)Running all tests...$(NC)"
	cargo test --workspace --all-features

test-crate:
	@echo "$(GREEN)Running tests for $(CRATE) crate...$(NC)"
	@echo "$(YELLOW)Note: Use 'make test' for full workspace testing with proper feature flags$(NC)"
	cargo test -p $(CRATE) --all-features

clean:
	@echo "$(GREEN)Cleaning build artifacts...$(NC)"
	cargo clean

fmt:
	@echo "$(GREEN)Formatting code...$(NC)"
	cargo fmt

clippy:
	@echo "$(GREEN)Running Clippy...$(NC)"
	cargo clippy -- -W clippy::all

# Docker check helper
check-docker:
	@docker info > /dev/null 2>&1 || (echo "$(YELLOW)Error: Docker is not running. Please start Docker Desktop and try again.$(NC)" && exit 1)

# Docker targets
docker-build: check-docker
	@echo "$(GREEN)Building Docker image for $(SERVICE)...$(NC)"
	docker build \
		--build-arg SERVICE_NAME=$(SERVICE) \
		-t $(REGISTRY)/$(IMAGE_NAME)-$(SERVICE):$(VERSION) \
		-t $(REGISTRY)/$(IMAGE_NAME)-$(SERVICE):latest \
		.

docker-push: check-docker
	@echo "$(GREEN)Pushing $(SERVICE) image to registry...$(NC)"
	docker push $(REGISTRY)/$(IMAGE_NAME)-$(SERVICE):$(VERSION)
	docker push $(REGISTRY)/$(IMAGE_NAME)-$(SERVICE):latest

docker-build-all:
	@echo "$(GREEN)Building all service images...$(NC)"
	@for service in api worker grpc cli; do \
		$(MAKE) docker-build SERVICE=$$service; \
	done

docker-push-all:
	@echo "$(GREEN)Pushing all service images...$(NC)"
	@for service in api worker grpc cli; do \
		$(MAKE) docker-push SERVICE=$$service; \
	done

# Docker Compose targets
docker-compose-up: check-docker
	@echo "$(GREEN)Starting local dev environment...$(NC)"
	docker-compose up -d

docker-compose-down: check-docker
	@echo "$(GREEN)Stopping docker-compose services...$(NC)"
	docker-compose down

docker-compose-logs: check-docker
	@echo "$(GREEN)Viewing docker-compose logs...$(NC)"
	docker-compose logs -f

docker-compose-rebuild: check-docker
	@echo "$(GREEN)Rebuilding and restarting services...$(NC)"
	docker-compose up -d --build

# Kubernetes targets
k8s-apply-dev:
	@echo "$(GREEN)Deploying to dev environment...$(NC)"
	kubectl apply -f k8s/dev/namespace.yaml
	kubectl apply -f k8s/dev/configmap.yaml
	kubectl apply -f k8s/dev/secrets.yaml
	kubectl apply -f k8s/dev/postgres/
	kubectl apply -f k8s/dev/redis/
	@echo "Waiting for databases to be ready..."
	@sleep 30
	kubectl delete job db-migration -n rs-service-dev --ignore-not-found
	kubectl apply -f k8s/dev/migration-job.yaml
	kubectl wait --for=condition=complete --timeout=300s job/db-migration -n rs-service-dev || true
	kubectl apply -f k8s/dev/api/
	kubectl rollout status deployment/api -n rs-service-dev --timeout=300s

k8s-apply-staging:
	@echo "$(GREEN)Deploying to staging environment...$(NC)"
	kubectl apply -f k8s/staging/namespace.yaml
	kubectl apply -f k8s/staging/configmap.yaml
	kubectl apply -f k8s/staging/secrets.yaml
	kubectl apply -f k8s/staging/postgres/
	kubectl apply -f k8s/staging/redis/
	@echo "Waiting for databases to be ready..."
	@sleep 30
	kubectl delete job db-migration -n rs-service-staging --ignore-not-found
	kubectl apply -f k8s/staging/migration-job.yaml
	kubectl wait --for=condition=complete --timeout=300s job/db-migration -n rs-service-staging || true
	kubectl apply -f k8s/staging/api/
	kubectl rollout status deployment/api -n rs-service-staging --timeout=300s

k8s-apply-prod:
	@echo "$(GREEN)Deploying to production environment...$(NC)"
	@echo "$(YELLOW)Warning: Deploying to production!$(NC)"
	@read -p "Are you sure? (yes/no): " confirm && [ "$$confirm" = "yes" ]
	kubectl apply -f k8s/prod/namespace.yaml
	kubectl apply -f k8s/prod/configmap.yaml
	kubectl apply -f k8s/prod/external-services/
	kubectl delete job db-migration -n rs-service-prod --ignore-not-found
	kubectl apply -f k8s/prod/migration-job.yaml
	kubectl wait --for=condition=complete --timeout=600s job/db-migration -n rs-service-prod || true
	kubectl apply -f k8s/prod/api/
	kubectl rollout status deployment/api -n rs-service-prod --timeout=600s

k8s-delete-dev:
	@echo "$(YELLOW)Deleting dev namespace...$(NC)"
	kubectl delete namespace rs-service-dev

k8s-delete-staging:
	@echo "$(YELLOW)Deleting staging namespace...$(NC)"
	kubectl delete namespace rs-service-staging

k8s-delete-prod:
	@echo "$(YELLOW)Warning: Deleting production namespace!$(NC)"
	@read -p "Are you sure? (yes/no): " confirm && [ "$$confirm" = "yes" ]
	kubectl delete namespace rs-service-prod

k8s-logs-dev:
	@echo "$(GREEN)Viewing logs from dev API pods...$(NC)"
	kubectl logs -f -l app=api -n rs-service-dev

k8s-logs-staging:
	@echo "$(GREEN)Viewing logs from staging API pods...$(NC)"
	kubectl logs -f -l app=api -n rs-service-staging

k8s-logs-prod:
	@echo "$(GREEN)Viewing logs from prod API pods...$(NC)"
	kubectl logs -f -l app=api -n rs-service-prod

k8s-restart-api-dev:
	@echo "$(GREEN)Restarting API deployment in dev...$(NC)"
	kubectl rollout restart deployment/api -n rs-service-dev

k8s-restart-api-staging:
	@echo "$(GREEN)Restarting API deployment in staging...$(NC)"
	kubectl rollout restart deployment/api -n rs-service-staging

k8s-restart-api-prod:
	@echo "$(YELLOW)Restarting API deployment in production...$(NC)"
	@read -p "Are you sure? (yes/no): " confirm && [ "$$confirm" = "yes" ]
	kubectl rollout restart deployment/api -n rs-service-prod

# Database migration targets
migrate-up:
	@echo "$(GREEN)Running database migrations...$(NC)"
	sqlx migrate run --source crates/infrastructure/migrations

migrate-down:
	@echo "$(GREEN)Reverting last migration...$(NC)"
	sqlx migrate revert --source crates/infrastructure/migrations

migrate-create:
	@echo "$(GREEN)Creating new migration: $(NAME)...$(NC)"
	sqlx migrate add -r $(NAME) --source crates/infrastructure/migrations
