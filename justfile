# LMS Task Runner
# Ref: [PROJECT-STRUCTURE]
# This file provides single-command access to all Engineering Standards.

# Default: List all available commands
default:
    @just --list

# --- QUALITY GATES (LMS-DOC & LMS-TEST) ---

# Run the complete Quality Gate (Fmt -> Test -> Clippy -> Doc)
verify-all: fmt test-hermetic lint verify-docs
    @echo "All Engineering Standards passed."

# Enforce standard formatting
fmt:
    cargo fmt --all

# Run all unit and integration tests in isolation
test-hermetic:
    cargo test --workspace --all-targets

# Run the linter with warning denial
lint:
    cargo clippy --workspace --all-targets -- -D warnings

# Verify narrative documentation and intra-doc links
verify-docs:
    RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --document-private-items

# --- PERFORMANCE GATES (LMS-OPS) ---

# Run the resolution pipeline benchmarks
bench-critical:
    cargo bench --bench performance_verification

# --- UTILITIES ---

# Initialize a new source file from the TEMPLATE
# Usage: just new-module path/to/file.rs
# Bootstraps a new Rust module and its directory-level README.md from standards
new-module path:
    @mkdir -p $(dirname {{path}})
    @cp docs/standards/TEMPLATE.rs {{path}}
    @cp docs/standards/LMS-MODULE-README-TEMPLATE.md $(dirname {{path}})/README.md
    @echo "[LMS-BOOTSTRAP]: Initialized {{path}} and $(dirname {{path}})/README.md"
    @echo "[LMS-BOOTSTRAP]: REMINDER: Update Blueprint Ref and Location tags immediately."

# Configure local git hooks to use the .githooks directory
install-hooks:
    git config core.hooksPath .githooks
    chmod +x .githooks/pre-push
    @echo "Git hooks installed successfully according to v0.1.4 standard."

# Clean build artifacts
clean:
    cargo clean

# Build the production Docker image
docker-build:
    docker build -t bistun-lms:latest .

# Run the project within a local Docker container with an optional port override
# Usage: just docker-run (defaults to 8080) OR just docker-run 8081
docker-run port="8080":
    docker run -p {{port}}:8080 bistun-lms:latest