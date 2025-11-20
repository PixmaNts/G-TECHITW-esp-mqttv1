FROM rust:1.91.1-slim-bookworm as builder
WORKDIR /app

# Install build dependencies for SSL/TLS (required for HTTPS/OpenAI API)
RUN apt-get update && apt-get install -y \
    libssl-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

# Copy workspace files
COPY Cargo.toml Cargo.lock* ./
COPY . .

# Build the project
RUN cargo build --release --bin mqtt_client

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/mqtt_client /usr/local/bin/mqtt_client
CMD ["mqtt_client"]