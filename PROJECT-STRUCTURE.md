# PROJECT-STRUCTURE: Architectural Logic & Layout

Version: 0.9.0  Status: Engineering Standard (Workspace Pivot)

I. Repository Overview
This project follows a "Separation of Writing Systems" philosophy across a decoupled Workspace Monorepo.

II. Directory Structure
- crates/              # Primary Workspace Units
    - bistun-core/       # Shared Linguistic DNA (Traits, DTOs, Registry)
    - bistun-lms/        # High-Performance SDK (Manager, Resolvers, Aggregators)
    - bistun-api/        # Delivery Layer (REST API, Sidecar Handlers)
- ai/                  # MACHINE LAYER (Agent Instruction & Alignment)
- docs/                # HUMAN LAYER (Architecture & Engineering Standards)
- data/                # PERSISTENCE LAYER (WORM Postcard Snapshots)

III. Architectural Rationale
[Logic for Taxonomy, Typology, and Orthography domains preserved...]

IV. Documentation & Machine Hierarchy
The repository is split into six layers to serve human stakeholders and AI agents:

1. Foundations (docs/foundations/): The "Executive" layer.
2. Blueprints (docs/blueprints/): The "Implementation" layer.
3. Standards (docs/standards/): The "Engineering" layer.
4. Interfaces (docs/interfaces/): The "Admin" layer.
5. Processes (docs/processes/): The "Operational" layer.
6. Machine Layer (ai/): The "Agent Instruction" layer. High-density context and
   authoritative templates for AI-assisted development.