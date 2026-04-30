# LMS-PROCESS-LABELS: Architectural Taxonomy

> **Version:** 0.1.4  
> **Author:** Francis Xavier Wazeter IV  
> **Date:** 04/30/2026  
> **Status:** Operational Process Guide

---

## I. Overview
To maintain the integrity of the **System of Record**, we categorize all work by its architectural domain. This ensures that **Orthographic** changes do not accidentally leak into **Taxonomic** PRs, preserving the **Single Responsibility Principle**.

---

## II. Primary Domain Labels
These labels represent the core pillars of the LMS architecture.

* **`domain:orthography`**: (Color: `#1d76db`)  
  Issues related to writing systems, technical rendering, and ISO 15924 standards.
* **`domain:typology`**: (Color: `#0e8a16`)  
  Issues related to language identity, morphological traits, and ISO 639-3 standards.
* **`domain:taxonomy`**: (Color: `#fbca04`)  
  Issues related to BCP 47 resolution, fallback logic, and registry classification.

---

## III. Quality Gate Labels
* **`gate:performance`**: Violations of the **< 1ms** resolution budget.
* **`gate:narrative`**: Violations of the **LMS-DOC** logic trace standard.
* **`gate:test`**: Violations of the **LMS-TEST** isolation standard.