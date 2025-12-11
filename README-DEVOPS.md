# DevOps Guide for rs-service-template

This guide covers all DevOps operations for the Rust microservice template, including Docker builds, local development, Kubernetes deployments, and CI/CD workflows.

## Table of Contents

- [Quick Start](#quick-start)
- [Prerequisites](#prerequisites)
- [Docker](#docker)
- [Local Development with Docker Compose](#local-development-with-docker-compose)
- [Kubernetes Deployment](#kubernetes-deployment)
- [Configuration Management](#configuration-management)
- [CI/CD Pipeline](#cicd-pipeline)
- [Monitoring and Logging](#monitoring-and-logging)
- [Troubleshooting](#troubleshooting)
- [Security Best Practices](#security-best-practices)

---

## Quick Start

### Local Development
```bash
# Start all services (PostgreSQL, Redis, API)
make docker-compose-up

# View logs
make docker-compose-logs

# Stop services
make docker-compose-down
```

### Deploy to Dev
```bash
make k8s-apply-dev
```

### Build and Push Docker Images
```bash
# Build specific service
make docker-build SERVICE=api

# Build all services
make docker-build-all

# Push to registry
make docker-push-all
```

---

## Prerequisites

### Required Tools
- **Docker** 24.0+ with BuildKit enabled
- **Docker Compose** 2.0+
- **kubectl** 1.28+ (for Kubernetes deployments)
- **Rust** 1.85+ (for local builds)
- **make** (for Makefile targets)
- **git** (for version tagging)

### Optional Tools
- **k9s** - Kubernetes CLI UI (recommended for monitoring)
- **sqlx-cli** - Database migrations
- **cargo-watch** - Auto-reload during development

### Installing Prerequisites

#### macOS
```bash
brew install docker docker-compose kubectl rust make
brew install --cask docker
```

#### Linux (Ubuntu/Debian)
```bash
# Docker
curl -fsSL https://get.docker.com | sh
sudo usermod -aG docker $USER

# kubectl
curl -LO "https://dl.k8s.io/release/$(curl -L -s https://dl.k8s.io/release/stable.txt)/bin/linux/amd64/kubectl"
sudo install -o root -g root -m 0755 kubectl /usr/local/bin/kubectl

# Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

---

## Docker

### Image Architecture

Our Docker images use a **multi-stage build** strategy:

1. **Builder Stage** (rust:1.85-alpine):
   - Installs musl toolchain for static linking
   - Builds Rust binary with full optimizations
   - Uses cargo cache mounts for faster rebuilds

2. **Runtime Stage** (distroless):
   - Uses gcr.io/distroless/static-debian12:nonroot
   - Contains only the static binary and config files
   - No shell or package manager (minimal attack surface)
   - Runs as non-root user (uid 65532)

**Final Image Size**: ~5-10MB per service

### Build Optimization

Build optimizations are configured in `.cargo/config.toml`:
- Full LTO (Link-Time Optimization)
- Single codegen unit for maximum optimization
- Strip debug symbols (~30% size reduction)
- Static linking with musl

### Building Images

#### Build Single Service
```bash
# Using Makefile (recommended)
make docker-build SERVICE=api

# Using Docker directly
docker build --build-arg SERVICE_NAME=api -t api:latest .
```

#### Build All Services
```bash
# Builds api, worker, grpc, and cli
make docker-build-all

# Or use the script
./scripts/build_all.sh
```

#### Push to Registry
```bash
# Configure registry (update in Makefile)
REGISTRY=ghcr.io/your-org

# Push specific service
make docker-push SERVICE=api

# Push all services
make docker-push-all
```

### Supported Services
- **api** - HTTP API service (Actix Web)
- **worker** - Background job processor (placeholder)
- **grpc** - gRPC service (placeholder)
- **cli** - Command-line tool (placeholder)

---

## Local Development with Docker Compose

### Architecture

docker-compose.yml provides:
- **PostgreSQL 16** (port 5432)
- **Redis 7** (port 6379)
- **API Service** (port 8080)

### Quick Commands

```bash
# Start all services
make docker-compose-up
# or
docker-compose up -d

# View logs (follow mode)
make docker-compose-logs

# Stop services
make docker-compose-down

# Rebuild and restart (after code changes)
make docker-compose-rebuild

# Stop and remove volumes (fresh start)
docker-compose down -v
```

### Configuration

Environment variables are loaded from:
1. docker-compose.yml
2. .env file (if exists)
3. docker-compose.override.yml (if exists)

**Example .env**:
```env
APP_ENV=dev
RUST_LOG=debug,actix_web=info
APP__DATABASE__CONNECTION_STRING=postgresql://postgres:postgres@postgres:5432/rs_service_dev
```

### Database Initialization

Migrations run automatically on API startup (if `APP__DATABASE__RUN_MIGRATIONS=true`).

Manual migration:
```bash
make migrate-up
```

### Accessing Services

- **API**: http://localhost:8080
- **Health Check**: http://localhost:8080/health
- **PostgreSQL**: localhost:5432
  - User: postgres
  - Password: postgres
  - Database: rs_service_dev
- **Redis**: localhost:6379

---

## Kubernetes Deployment

### Cluster Setup

Ensure you have access to a Kubernetes cluster:
```bash
# Verify connection
kubectl cluster-info

# Check current context
kubectl config current-context

# List available contexts
kubectl config get-contexts
```

### Environments

Three environments are supported:
- **dev** - Development (namespace: rs-service-dev)
- **staging** - Pre-production (namespace: rs-service-staging)
- **prod** - Production (namespace: rs-service-prod)

### Deploy to Dev

```bash
# Deploy everything
make k8s-apply-dev

# Or step-by-step
kubectl apply -f k8s/dev/namespace.yaml
kubectl apply -f k8s/dev/configmap.yaml
kubectl apply -f k8s/dev/secrets.yaml
kubectl apply -f k8s/dev/postgres/
kubectl apply -f k8s/dev/redis/
kubectl apply -f k8s/dev/migration-job.yaml
kubectl apply -f k8s/dev/api/
```

### Deploy to Staging

```bash
make k8s-apply-staging
```

**Note**: Staging includes an Ingress for external access at `api-staging.example.com`.

### Deploy to Production

```bash
# Requires confirmation
make k8s-apply-prod
```

**Production Differences**:
- Uses external managed databases (RDS, ElastiCache)
- Higher replica count (5 minimum)
- Stricter logging (warn level only)
- Manual approval required in CI/CD

### Monitoring Deployments

```bash
# View logs
make k8s-logs-dev

# Check pod status
kubectl get pods -n rs-service-dev

# Describe deployment
kubectl describe deployment api -n rs-service-dev

# View HPA status
kubectl get hpa -n rs-service-dev

# Check events
kubectl get events -n rs-service-dev --sort-by='.lastTimestamp'
```

### Scaling

```bash
# Manual scaling
kubectl scale deployment api --replicas=5 -n rs-service-dev

# HPA (Horizontal Pod Autoscaler) is configured to auto-scale:
# - Dev: 2-10 replicas
# - Staging: 3-20 replicas
# - Prod: 5-50 replicas
# Based on CPU (70%) and Memory (80%) usage
```

### Health Checks

Liveness and readiness probes are configured:
```yaml
livenessProbe:
  httpGet:
    path: /health
    port: 8080
  initialDelaySeconds: 10
  periodSeconds: 30

readinessProbe:
  httpGet:
    path: /health
    port: 8080
  initialDelaySeconds: 5
  periodSeconds: 10
```

### Updating Deployments

```bash
# Restart pods (rolling restart)
make k8s-restart-api-dev

# Update image version
kubectl set image deployment/api api=ghcr.io/your-org/rs-service-template-api:v1.2.3 -n rs-service-dev

# Rollback
kubectl rollout undo deployment/api -n rs-service-dev

# View rollout history
kubectl rollout history deployment/api -n rs-service-dev
```

### Cleaning Up

```bash
# Delete dev environment (WARNING: deletes all data)
make k8s-delete-dev

# Delete specific resources
kubectl delete deployment api -n rs-service-dev
kubectl delete service api-service -n rs-service-dev
```

---

## Configuration Management

### Configuration Hierarchy

1. **TOML Files** (config/dev.toml, staging.toml, prod.toml)
   - Default values for each environment
   - Committed to git

2. **Environment Variables** (APP__SECTION__KEY)
   - Override TOML values
   - Provided via Kubernetes ConfigMaps

3. **Secrets** (Kubernetes Secrets)
   - Sensitive data (database passwords, API keys)
   - NOT committed to git

### Environment-Specific Settings

| Setting | Dev | Staging | Prod |
|---------|-----|---------|------|
| Workers | 2 | 4 | 8 |
| Max Connections | 1000 | 10000 | 25000 |
| DB Pool | 10 | 20 | 20 |
| Logging | debug | info | warn |
| Run Migrations | true | false | false |

### Managing Secrets

#### Dev Environment
```bash
# Secrets are in k8s/dev/secrets.yaml (example values)
kubectl apply -f k8s/dev/secrets.yaml
```

#### Staging/Production
```bash
# Create secrets manually (NOT from file)
kubectl create secret generic app-secrets \
  --from-literal=DATABASE_CONNECTION_STRING='postgresql://...' \
  --from-literal=CACHE_URL='redis://...' \
  -n rs-service-prod

# Or use sealed-secrets
kubeseal < secrets.yaml > sealed-secrets.yaml
kubectl apply -f sealed-secrets.yaml
```

### Updating Configuration

#### ConfigMap Changes
```bash
# Edit ConfigMap
kubectl edit configmap app-config -n rs-service-dev

# Or apply updated file
kubectl apply -f k8s/dev/configmap.yaml

# Restart pods to pick up changes
kubectl rollout restart deployment/api -n rs-service-dev
```

---

## CI/CD Pipeline

### GitHub Actions Workflow

Workflow: `.github/workflows/cd.yml`

### Triggers

- **Push to `master`** → Build + Deploy to Dev
- **Push to `release`** → Build + Deploy to Staging
- **Tag push (`v*`)** → Build + Deploy to Prod + Create Release

### Workflow Steps

1. **Build and Push** (matrix: api, worker, grpc, cli)
   - Build Docker images with BuildKit caching
   - Push to ghcr.io (GitHub Container Registry)
   - Tag with git commit SHA and branch name

2. **Deploy to Dev** (on master push)
   - Apply namespace, configmaps, secrets
   - Deploy databases (PostgreSQL, Redis)
   - Run migration job
   - Deploy API service
   - Verify deployment

3. **Deploy to Staging** (on release push)
   - Similar to dev
   - Includes ingress configuration

4. **Deploy to Prod** (on tag push)
   - Requires manual approval (GitHub Environment)
   - Uses external databases
   - Creates GitHub Release

### Required GitHub Secrets

Configure in **Settings → Secrets and variables → Actions**:

```bash
# Kubernetes access
KUBECONFIG_DEV         # Base64-encoded kubeconfig for dev cluster
KUBECONFIG_STAGING     # Base64-encoded kubeconfig for staging cluster
KUBECONFIG_PROD        # Base64-encoded kubeconfig for prod cluster

# GITHUB_TOKEN is automatically provided
```

### Creating Kubeconfig Secrets

```bash
# Encode kubeconfig
cat ~/.kube/config | base64

# Or for specific context
kubectl config view --flatten --minify --context=dev-cluster | base64
```

### Manual Deployment

```bash
# Deploy using script
./scripts/deploy.sh dev api

# Or using Makefile
make k8s-apply-dev
```

---

## Monitoring and Logging

### Viewing Logs

#### Docker Compose
```bash
# All services
docker-compose logs -f

# Specific service
docker-compose logs -f api
```

#### Kubernetes
```bash
# Using Makefile
make k8s-logs-dev

# Using kubectl
kubectl logs -f -l app=api -n rs-service-dev

# Specific pod
kubectl logs -f api-xxxx-yyyy -n rs-service-dev

# Previous crashed pod
kubectl logs --previous api-xxxx-yyyy -n rs-service-dev
```

### Metrics

Horizontal Pod Autoscaler (HPA) tracks:
- CPU utilization (target: 70%)
- Memory utilization (target: 80%)

```bash
# View HPA status
kubectl get hpa -n rs-service-dev

# View detailed metrics
kubectl top pods -n rs-service-dev
kubectl top nodes
```

### Health Checks

```bash
# Local
curl http://localhost:8080/health

# Kubernetes (port-forward)
kubectl port-forward svc/api-service 8080:80 -n rs-service-dev
curl http://localhost:8080/health

# Using health check script
./scripts/health_check.sh dev
```

---

## Troubleshooting

### Common Issues

#### 1. Pod Stuck in Pending
```bash
# Check events
kubectl describe pod <pod-name> -n rs-service-dev

# Common causes:
# - Insufficient resources
# - PVC not bound
# - Image pull errors
```

#### 2. CrashLoopBackOff
```bash
# View logs
kubectl logs <pod-name> -n rs-service-dev

# Check previous logs
kubectl logs --previous <pod-name> -n rs-service-dev

# Common causes:
# - Database connection failure
# - Missing environment variables
# - Application crash on startup
```

#### 3. Database Connection Errors
```bash
# Verify database is running
kubectl get pods -l app=postgres -n rs-service-dev

# Test connection from API pod
kubectl exec -it <api-pod> -n rs-service-dev -- /bin/sh
# (distroless images don't have shell, use debug container)

# Check secrets
kubectl get secret app-secrets -n rs-service-dev -o yaml
```

#### 4. Image Pull Errors
```bash
# Check image exists
docker pull ghcr.io/your-org/rs-service-template-api:latest

# Verify imagePullSecrets (if using private registry)
kubectl get secret ghcr-secret -n rs-service-dev
```

#### 5. Migration Job Failed
```bash
# Check job status
kubectl get jobs -n rs-service-dev

# View logs
kubectl logs job/db-migration -n rs-service-dev

# Delete and retry
kubectl delete job db-migration -n rs-service-dev
kubectl apply -f k8s/dev/migration-job.yaml
```

### Debug Commands

```bash
# Get all resources in namespace
kubectl get all -n rs-service-dev

# Describe resource
kubectl describe <resource-type> <resource-name> -n rs-service-dev

# Interactive shell (if available)
kubectl exec -it <pod-name> -n rs-service-dev -- /bin/bash

# Port forward for debugging
kubectl port-forward <pod-name> 8080:8080 -n rs-service-dev

# Copy files from pod
kubectl cp <pod-name>:/app/config ./local-config -n rs-service-dev
```

---

## Security Best Practices

### Container Security

✅ **Implemented**:
- Distroless base image (no shell, minimal packages)
- Run as non-root user (uid 65532)
- Static binary (no runtime dependencies)
- Multi-stage build (build dependencies not in final image)

🔒 **Additional Recommendations**:
- Use specific image tags (not `latest`) in production
- Sign images with cosign
- Scan images with Trivy: `trivy image ghcr.io/your-org/rs-service-template-api:latest`
- Implement pod security policies/standards

### Secrets Management

✅ **Implemented**:
- Secrets stored in Kubernetes Secrets (not ConfigMaps)
- .gitignore excludes secret files
- Database passwords not hardcoded

🔒 **Production Recommendations**:
- Use Sealed Secrets or External Secrets Operator
- Rotate secrets regularly
- Use AWS Secrets Manager / GCP Secret Manager / Azure Key Vault
- Enable secret encryption at rest in etcd

### Network Security

🔒 **Recommendations**:
- Implement Network Policies for pod-to-pod isolation
- Use TLS for all external communication (Ingress configured)
- Enable mutual TLS (mTLS) with service mesh (Istio/Linkerd)
- Restrict API access with rate limiting (configured in Ingress)

### Access Control

🔒 **Recommendations**:
- Implement RBAC for Kubernetes access
- Use separate service accounts per deployment
- Enable audit logging
- Restrict kubectl access to specific namespaces
- Use temporary credentials (AWS IAM roles, GCP Workload Identity)

---

## Additional Resources

### Internal Documentation
- [Main README](README.md) - Project overview and development
- [CLAUDE.md](CLAUDE.md) - Project instructions for Claude Code

### External Resources
- [Kubernetes Documentation](https://kubernetes.io/docs/home/)
- [Docker Best Practices](https://docs.docker.com/develop/dev-best-practices/)
- [Rust Performance Book](https://nnethercote.github.io/perf-book/)
- [Actix Web Documentation](https://actix.rs/)

### Tools
- [k9s](https://k9scli.io/) - Kubernetes CLI UI
- [Lens](https://k8slens.dev/) - Kubernetes IDE
- [Stern](https://github.com/stern/stern) - Multi-pod log tailing
- [kubectx/kubens](https://github.com/ahmetb/kubectx) - Context/namespace switching

---

## Support

For issues or questions:
1. Check [Troubleshooting](#troubleshooting) section
2. Review logs: `make k8s-logs-dev`
3. Run health check: `./scripts/health_check.sh dev`
4. Check cluster events: `kubectl get events -n rs-service-dev`

---

**Last Updated**: 2025-12-10
