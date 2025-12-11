# Multi-stage Dockerfile for Rust microservices
# Builds statically-linked binaries using musl and deploys to distroless for minimal image size

# Stage 1: Builder
FROM rust:1.85-alpine AS builder

# Install musl development tools, GCC, and OpenSSL for static linking
RUN apk add --no-cache \
    musl-dev \
    gcc \
    openssl-dev \
    openssl-libs-static \
    pkgconfig \
    && rm -rf /var/cache/apk/*

# Add musl target for static linking
RUN rustup target add x86_64-unknown-linux-musl

# Configure cargo to use the musl linker
ENV CC_x86_64_unknown_linux_musl=gcc \
    AR_x86_64_unknown_linux_musl=ar \
    CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER=gcc

# Create app directory
WORKDIR /app

# Accept build argument for service name (api, worker, grpc, cli)
ARG SERVICE_NAME=api
ENV SERVICE_NAME=${SERVICE_NAME}

# Copy workspace manifests first for better layer caching
COPY Cargo.toml Cargo.lock ./

# Copy all crates and services
COPY crates/ ./crates/
COPY services/ ./services/

# Copy configuration files and migrations
COPY config/ ./config/
COPY crates/infrastructure/migrations/ ./migrations/

# Build the specified service in release mode with caching
# The --mount flags enable BuildKit cache mounts for faster rebuilds
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo build --release --target x86_64-unknown-linux-musl -p ${SERVICE_NAME} && \
    # Copy the built binary to a known location
    cp /app/target/x86_64-unknown-linux-musl/release/${SERVICE_NAME} /output-binary

# Stage 2: Runtime (distroless)
FROM gcr.io/distroless/static-debian12:nonroot

# Copy binary from builder
COPY --from=builder /output-binary /app/service

# Copy configuration files
COPY --from=builder /app/config /app/config

# Copy migrations
COPY --from=builder /app/migrations /app/migrations

# Set working directory
WORKDIR /app

# Expose default port (can be overridden at runtime)
EXPOSE 8080

# Set user to nonroot (uid 65532, already set by distroless :nonroot)
# USER 65532:65532

# Health check (requires service to implement health endpoint)
# Note: Distroless doesn't have shell, so we can't use curl
# Health checks should be done at the orchestration level (K8s liveness/readiness probes)
# HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
#     CMD ["/app/service", "--health"] || exit 1

# Run the service binary
ENTRYPOINT ["/app/service"]
