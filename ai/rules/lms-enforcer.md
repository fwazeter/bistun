# LMS-ENFORCER: Agent Alignment & Reasoning Constraints

Ref: [LMS-AI] | Location: `ai/rules/lms-enforcer.md` | Status: Authoritative

I. Reasoning Mandate
Any agent modifying the `crates/` tree must prioritize these logic gates:

1. System of Record: Every change is a permanent update to linguistic truth.
   Reject hacks.
2. Workspace Boundaries:
    - `bistun-core` must remain zero-dependency and immutable.
    - `bistun-lms` must never leak implementation details to the API layer.
3. High-Water Mark: Trait aggregation must always favor the highest rank
   complexity.
4. Performance Budget: Latency must remain < 1ms p99.

II. Execution Pipeline
You must reason through the 5-Phase Pipeline for every request:
Resolve -> Aggregate -> Override -> Integrity -> Telemetry.

III. Scaffolding
Always use the authoritative template at `ai/templates/module.rs` for
new Rust files.