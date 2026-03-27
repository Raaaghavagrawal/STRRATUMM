# --- Stage 1: Builder ---
FROM rust:bookworm AS builder

# Set working directory
WORKDIR /app

# Install system dependencies
RUN apt-get update && apt-get install -y \
    libwebkit2gtk-4.1-dev \
    build-essential \
    pkg-config \
    libssl-dev \
    libgtk-3-dev \
    && rm -rf /var/lib/apt/lists/*

# Cache buster to force fresh build if needed
ARG CACHE_BUST=1

# Copy project files
COPY Cargo.toml Cargo.lock ./
COPY . .

# Build the binary
RUN cargo build --release

# --- Stage 2: Runtime ---
FROM debian:bookworm-slim

# Install runtime dependencies (Exact same OS version as builder)
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libwebkit2gtk-4.1-0 \
    libgtk-3-0 \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/ai-overlay /usr/local/bin/ai-overlay

# Config
EXPOSE 3001
ENV API_PORT=3001
ENV SERVER_ONLY=true
ENV RUST_LOG=info

ENTRYPOINT ["ai-overlay"]