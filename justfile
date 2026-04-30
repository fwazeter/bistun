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
new-module path:
    mkdir -p $(dirname {{path}})
    cp TEMPLATE.rs {{path}}
    @echo "Initialized {{path}} from TEMPLATE.rs"

# Configure local git hooks to use the .githooks directory
install-hooks:
    git config core.hooksPath .githooks
    chmod +x .githooks/pre-push
    @echo "Git hooks installed successfully according to v0.1.4 standard."

# Clean build artifacts
clean:
    cargo clean