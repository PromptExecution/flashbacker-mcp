# 🦀 CommerceRack Rust - Multi-stage Docker build
FROM rust:1.82-slim as builder

WORKDIR /app

# Install dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy workspace configuration
COPY Cargo.toml ./
COPY crates ./crates

# Build release binary
RUN cargo build --release --bin commercerack-api

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Copy binary from builder
COPY --from=builder /app/target/release/commercerack-api /usr/local/bin/

# Expose API port
EXPOSE 8000

# Run as non-root
RUN useradd -m -u 1000 appuser && chown -R appuser:appuser /app
USER appuser

CMD ["commercerack-api"]
