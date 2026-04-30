# LMS-STACK: Tech Stack & Dependency Repository

> **Version:** 0.1.4
<br> **Status:** Engineering Standard

---

## I. Overview

To maintain the Bistun LMS as a world-class **System of Record**, all contributors must have a deep understanding of the underlying tools and standards. This repository provides direct links to the primary documentation for every core feature, dependency, and infrastructure component used in the project.

---

## II. Core Rust Language & Features

The project utilizes the modern Rust ecosystem to achieve memory safety and high performance.

* **Rust Edition 2024**: The latest language standard providing the foundation for our asynchronous and idiomatic logic.
    * [Edition Guide](https://doc.rust-lang.org/nightly/edition-guide/rust-2024/index.html)
* **Traits (Shared Vocabulary)**: Used to define shared behavior across the 5-phase pipeline and facilitate hermetic mocking.
    * [Rust Book: Traits](https://doc.rust-lang.org/book/ch10-02-traits.html)
* **Enums (Algebraic Data Types)**: Used for strict, type-safe linguistic classifications like `MorphType` and `SegType`.
    * [Rust Book: Enums](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html)

---

## III. Production Runtime Dependencies

These crates are critical for the execution of the capability engine and DTO synthesis.

* **`thiserror`**: Provides the framework for our phase-linked error handling standard.
    * [thiserror Documentation](https://docs.rs/thiserror)
* **`tracing`**: Enables high-performance, structured observability through diagnostic spans and logic traces.
    * [tracing Documentation](https://docs.rs/tracing)
* **`serde` & `serde_json`**: The industry standard for serializing and deserializing our immutable `CapabilityManifest` DTOs.
    * [Serde Website](https://serde.rs/)
* **`hashbrown`**: High-performance hash maps used to optimize our in-memory **Flyweight** pools.
    * [hashbrown Documentation](https://docs.rs/hashbrown)

---

## IV. Development & Verification Tools

These tools enforce the project's strict Quality Gates and performance budgets.

* **`just`**: Our unified task runner and command automation hub for consistent environment setup.
    * [just Repository](https://github.com/casey/just)
* **`mockall`**: Essential for creating hermetic test doubles to ensure module isolation during unit testing.
    * [mockall Documentation](https://docs.rs/mockall)
* **`criterion`**: Used for statistical benchmarking to scientifically prove we meet the **< 1ms** resolution target.
    * [Criterion.rs Book](https://bheisler.github.io/criterion.rs/book/index.html)
* **`rustfmt` & `clippy`**: Automated enforcers for project style and high-performance Rust idioms.
    * [Rustfmt](https://github.com/rust-lang/rustfmt) | [Clippy](https://rust-lang.github.io/rust-clippy/)

---

## V. Infrastructure & Deployment

The deployment architecture ensures consistent execution across all environments.

* **Docker**: Multi-stage production containerization used for sidecar and service deployment.
    * [Docker Documentation](https://docs.docker.com/)
* **GitHub Actions**: Powers our CI pipeline to enforce **LMS-DOC** and **LMS-TEST** standards on every push.
    * [GitHub Actions Docs](https://docs.github.com/en/actions)
* **Mermaid.js**: Utilized within documentation and code comments for architectural logic traces and state machine diagrams.
    * [Mermaid.js Website](https://mermaid.js.org/)

---

## VI. Authoritative Linguistic Standards

Our internal "Truth Hierarchy" is built upon these international specifications.

* **ISO 639-3 (Typology)**: The standard identifying all known natural languages.
    * [ISO 639-3 Standard Site](https://iso639-3.sil.org/)
* **ISO 15924 (Orthography)**: The standard for the names and codes of writing systems.
    * [ISO 15924 Standard Site](https://www.unicode.org/iso15924/)
* **BCP 47 & RFC 4647 (Taxonomy)**: The standard for language tags and matching logic used in resolution.
    * [BCP 47 Spec](https://tools.ietf.org/html/bcp47) | [RFC 4647 Spec](https://tools.ietf.org/html/rfc4647)

---

**Author**: Francis Xavier Wazeter IV  
**License**: GNU GPL v3  
**Date Created**: 04/30/2026  
**Date Updated**: 04/30/2026