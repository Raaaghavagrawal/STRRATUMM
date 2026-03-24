# --- Stage 1: Builder ---
FROM rust:1.75-bookworm as builder

# Set working directory inside the container
WORKDIR /app

# Install system dependencies needed for building Tauri/Rust apps on Linux
RUN apt-get update && apt-get install -y \
    libwebkit2gtk-4.1-dev \
    build-essential \
    curl \
    wget \
    file \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

# Optimize for dependency caching
# Copy only the configuration files first
COPY Cargo.toml Cargo.lock ./

# Create a dummy source file to compile only dependencies
RUN mkdir src \
    && echo "fn main() {println!(\"if you see this, the build failed\")}" > src/main.rs \
    && cargo build --release \
    && rm -rf src/

# Copy the actual source code
COPY . .

# Build the final binary
# We use the binary name from Cargo.toml (ai-overlay)
RUN cargo build --release

# --- Stage 2: Runtime ---
FROM debian:bookworm-slim as runtime

# Install runtime dependencies (ca-certificates for HTTPS/APIs)
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libwebkit2gtk-4.1-0 \
    libgtk-3-0 \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy the binary from the builder stage
COPY --from=builder /app/target/release/ai-overlay /app/ai-overlay

# Expose the API port (as per requirements)
EXPOSE 3001

# Set default environment variables
ENV API_PORT=3001
ENV SERVER_ONLY=true
ENV RUST_LOG=info

# Run the backend API only
CMD ["./ai-overlay"]
