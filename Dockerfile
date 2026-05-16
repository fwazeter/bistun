# [Stage 1]: Builder
FROM rust:1.81-slim-bookworm AS builder

WORKDIR /usr/src/bistun
COPY . .

# Build only the API sidecar with production features
# We disable default features to keep the binary lean
RUN cargo build --release -p bistun-api --no-default-features --features "fs"

# =========================================================================
# MILESTONE E.2: Distroless Hardening & Non-Root Execution
# =========================================================================
# [Stage 2]: Production Runner
# Utilizing Google's distroless base strips out all shells and package managers,
# drastically reducing the attack surface and image size.
FROM gcr.io/distroless/cc-debian12:nonroot

WORKDIR /app

# Copy the binary from the builder and assign explicit nonroot ownership
COPY --from=builder --chown=nonroot:nonroot /usr/src/bistun/target/release/bistun-api /app/bistun-api

# Inject the curated WORM snapshots with nonroot ownership
COPY --chown=nonroot:nonroot data/snapshot.json /app/data/snapshot.json
COPY --chown=nonroot:nonroot data/snapshot.sig /app/data/snapshot.sig

# Expose the Hot-Path port
EXPOSE 8080

# Enforce unprivileged user execution (Kubernetes Security Context Alignment)
USER nonroot

# The .env variables should be injected by the orchestrator (e.g., K8s ConfigMap)
ENTRYPOINT ["/app/bistun-api"]
