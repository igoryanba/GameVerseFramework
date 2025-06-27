# GameVerse Framework Server - Multi-stage Docker Build
# Stage 1: Dependencies and Build Environment
FROM rust:1.75-alpine AS dependencies

# Install build dependencies
RUN apk add --no-cache \
    musl-dev \
    pkgconfig \
    openssl-dev \
    curl \
    git

WORKDIR /app

# Copy dependency files first for better caching
COPY Cargo.toml Cargo.lock ./
COPY core/Cargo.toml ./core/
COPY sdk/cli-tools/Cargo.toml ./sdk/cli-tools/
COPY networking/Cargo.toml ./networking/
COPY services/*/Cargo.toml ./services/*/

# Create dummy source files to build dependencies
RUN mkdir -p core/src sdk/cli-tools/src networking/src services/*/src && \
    echo "fn main() {}" > core/src/main.rs && \
    echo "fn main() {}" > sdk/cli-tools/src/main.rs && \
    echo "pub fn dummy() {}" > networking/src/lib.rs && \
    find services -name src -type d -exec sh -c 'echo "pub fn dummy() {}" > "$1"/lib.rs' _ {} \;

# Build dependencies only
RUN cargo build --release
RUN rm -rf core/src sdk/cli-tools/src networking/src services/*/src

# Stage 2: Application Build
FROM dependencies AS builder

# Copy source code
COPY . .

# Build the application
RUN cargo build --release --bin gameverse_server

# Stage 3: Runtime
FROM alpine:3.19 AS runtime

# Install runtime dependencies
RUN apk add --no-cache \
    ca-certificates \
    curl \
    && rm -rf /var/cache/apk/*

# Create non-root user
RUN addgroup -g 1001 gameverse && \
    adduser -D -s /bin/sh -u 1001 -G gameverse gameverse

# Create necessary directories
RUN mkdir -p /app/config /app/logs /app/resources && \
    chown -R gameverse:gameverse /app

# Copy binary from builder stage
COPY --from=builder /app/target/release/gameverse_server /usr/local/bin/gameverse_server
RUN chmod +x /usr/local/bin/gameverse_server

# Copy default configuration
COPY --from=builder /app/core/config/ /app/config/
RUN chown -R gameverse:gameverse /app/config

# Switch to non-root user
USER gameverse

# Set working directory
WORKDIR /app

# Expose ports
EXPOSE 8080 30121 9090

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:30121/api/health || exit 1

# Default command
CMD ["gameverse_server", "--config", "/app/config/server-config.toml"] 