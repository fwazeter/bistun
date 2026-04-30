# --- Stage 1: Build ---
# Decision: Use official 'slim' Rust image for version 1.80 (Stable 2024 Edition).
# Rationale: Ensures environmental consistency with development toolchain while 
# minimizing the initial download size.
FROM rust:slim-bookworm AS builder

# Metadata labeling within the image
LABEL author="Francis Xavier Wazeter IV"
LABEL license="GNU GPL v3"

WORKDIR /usr/src/bistun
COPY . .

# Decision: Perform a '--release' build.
# Rationale: Debug builds contain overhead that would breach the strict < 1ms 
# performance budget required for production.
RUN cargo build --release

# --- Stage 2: Runtime ---
# Decision: Multi-stage switch to a minimal Debian environment.
# Rationale: This is a security practice. The final image
# contains ONLY the binary, excluding the Rust toolchain and source code, 
# which drastically reduces the attack surface.
FROM debian:bookworm-slim

# Re-apply label to the final runtime stage
LABEL author="Francis Xavier Wazeter IV"

# Decision: Install 'ca-certificates'.
# Rationale: Necessary for the SDK to perform authenticated synchronization 
# over HTTPS with the LMS Sidecar/Registry.
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/local/bin

# Decision: Copy ONLY the compiled artifact from the builder stage.
COPY --from=builder /usr/src/bistun/target/release/bistun .

# Decision: Expose port 8080.
# Rationale: Aligns with the default service endpoint defined in the 
# core interface specification.
EXPOSE 8080

CMD ["./bistun"]

# ---
# Author: Francis Xavier Wazeter IV
# License: GNU GPL v3
# Date Created: 04/30/2026
# Date Updated: 04/30/2026