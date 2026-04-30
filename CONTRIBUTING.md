# Contributing to Bistun LMS

Thank you for contributing to the Linguistic Metadata Service (LMS). As a **System of Record** for Linguistic DNA, we maintain rigorous engineering standards to ensure 100% reliability and sub-millisecond performance.

## I. Environment Setup

To match our development standards, your local environment must be configured as follows:

### 1. Toolchain Requirements
* **Rust Stable**: Install via [rustup.rs](https://rustup.rs/).
* **Components**: You must have `clippy` and `rustfmt` installed:
  ```bash
  rustup component add clippy rustfmt
  ```
* **Mocking Utility**: We use `mockall` for hermetic testing. Ensure it is included in your `Cargo.toml` under `[dev-dependencies]`.

### 2. Formatter Configuration
We use standard Rust formatting. Before committing, always run:
```bash
cargo fmt --all
```

## II. Development Workflow

We follow a strict **Testing-First** and **Narrative-Code** workflow.

### 1. Initializing a Module
Never start a file from scratch. Always use the task runner to bootstrap your new module according to the **v0.1.4** standard:
```bash
just new-module src/path/to/file.rs
```

### 2. The "Red-Green-Narrative" Cycle
1. **Red**: Write a failing unit test in the module's `mod tests` block.
2. **Green**: Implement the minimum logic to pass the test using `// [STEP X]` inline comments.
3. **Narrative**: Complete the `///` documentation blocks, including the `# Logic Trace`, `# Examples`, and `# Golden I/O`.

## III. Testing Standards

All code must pass three levels of verification before a PR is opened:

* **Unit Tests**: `cargo test --lib` (Verifies isolated logic).
* **Doc Tests**: `cargo test --doc` (Verifies executable examples).
* **Integration Tests**: `cargo test --test '*'` (Verifies system physics).

## IV. Documentation Standard

We deny all documentation warnings to ensure the "Living Document" never rots. Verify your documentation with:
```bash
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --document-private-items
```
This ensures all intra-doc links to blueprints and types are valid.

## V. Commit & PR Protocols

* **Branching**: Use feature branches named after the Roadmap chunk (e.g., `feat/v0.1.0-chunk-1-models`).
* **PRs**: You must use the provided Pull Request Template. PRs without a **Logic Trace Mapping** will be automatically closed.

---

### Rationale for Step 3:
* **Lowering Entry Barriers**: Novices can follow the "Toolchain Requirements" to be productive within minutes without guessing which Rust version or components to use.
* **Consistency Enforcement**: By standardizing the "Red-Green-Narrative" cycle, we ensure that the **LMS-DOC** and **LMS-TEST** standards are built *into* the development process, not added as an afterthought.
* **Local Verification**: Providing the exact commands for testing and doc generation reduces the number of "Failed CI" cycles, keeping the development loop efficient.
