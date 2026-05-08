# [Stage 1]: Builder
FROM rust:1.81-slim-bookworm AS builder

WORKDIR /usr/src/bistun
COPY . .

# Build only the API sidecar with production features
# We disable default features to keep the binary lean
RUN cargo build --release -p bistun-api --no-default-features --features "fs"

# [Stage 2]: Production Runner
FROM debian:bookworm-slim

# Install minimal SSL certs for potential network hydration
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary from the builder
COPY --from=builder /usr/src/bistun/target/release/bistun-api /app/bistun-api

# Setup the data directory for the WORM snapshots
RUN mkdir -p /app/data
COPY data/snapshot.json /app/data/snapshot.json
COPY data/snapshot.sig /app/data/snapshot.sig

# Expose the Hot-Path port
EXPOSE 8080

# The .env file should be mounted as a volume or injected via ENV variables
ENTRYPOINT ["/app/bistun-api"]