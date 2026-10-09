# Institutional Executive Summary Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Produce a sourced, standalone Spanish executive summary that explains Ecuador's real-time payment opportunity and Caudal's gateway-to-red strategy to fintechs, cooperatives, and core-banking vendors.

**Architecture:** The document synthesizes the normative corpus and existing workstream trackers without replacing them. It uses two introductory diagrams, concise evidence tables, inline citations, and explicit labels for estimates and assumptions.

**Tech Stack:** Markdown, Mermaid, vendored BCE primary sources, public corporate and financial sources.

## Global Constraints

- Do not modify `workstream/authorized-red/business-plan.md`.
- Write native Ecuadorian Spanish following `docs/spanish-style.md`.
- Cite every regulatory claim by resolution and article, or Catálogo Operacional section.
- Mark unverified regulatory readings **[supuesto]** and market estimates **[est.]**.
- Do not infer field-level or transport rules from the unpublished Catálogo Técnico.
- Keep regional evidence brief; omit Bold and Conekta.
- Distinguish clearly between existing, designed, and not-yet-built Caudal capabilities.
- Do not commit signed agreements, personal data, credentials, or private company artifacts.

---

### Task 1: Build the evidence matrix

**Files:**
- Read: `research/bce/BCE-GG-024-2024.md`
- Read: `research/bce/JPRM-2024-018-M.md`
- Read: `research/bce/JPRM-2024-029-M.md`
- Read: `research/bce/BCE-GG-008-2025.md`
- Read: `research/bce/BCE-GG-007-2026.md`
- Read: `research/bce/Catalogo-Operacional.md`
- Read: matching PDFs under `research/bce/`
- Read: `workstream/authorized-red/regulatory/open-questions.md`

- [ ] Locate the exact articles and sections supporting the scheme topology, deadline, eligible integrantes, authorization dossier, operational duties, security baseline, and supervision.
- [ ] Verify each citation against the matching PDF, using the Markdown only to locate the passage.
- [ ] Record unresolved points as assumptions aligned with P1–P7 in `regulatory/open-questions.md`.
- [ ] Reject any claim that depends on the unpublished Catálogo Técnico.

### Task 2: Verify concise regional proof

**Files:**
- Read: `workstream/authorized-red/business-plan.md`
- Create content for: `workstream/authorized-red/resumen-ejecutivo.md`

- [ ] Select three or four infrastructure comparables from STP, Celcoin, Transfeera, and Minka.
- [ ] Find public sources for approximate customers, transaction volume, revenue or funding, and team size where reliably available.
- [ ] Prefer company disclosures, regulator documents, investor releases, and recognized financial publications.
- [ ] Omit any metric that cannot be sourced consistently.
- [ ] Reduce the findings to one table and no more than three conclusions relevant to Ecuador.

### Task 3: Draft the standalone narrative and diagrams

**Files:**
- Create: `workstream/authorized-red/resumen-ejecutivo.md`
- Read: `workstream/authorized-red/resumen-ejecutivo-design.md`
- Read: `docs/research/catalogo-operacional.md`
- Read: `docs/architecture/red.md`
- Read: `workstream/authorized-red/regulatory/dossier-psap.md`
- Read: `workstream/authorized-red/technical/readiness.md`

- [ ] Write the one-page summary and define SIP, RPI, red de pagos, integrante, llave, gateway, and partícipe on first use.
- [ ] Add a Mermaid topology diagram showing integrante → red → SIP → red → integrante and the Sistema Central de Llaves.
- [ ] Explain the needs of fintechs/SEDPES, cooperatives/cajas centrales, and core-banking vendors in that order.
- [ ] Add a Mermaid phase diagram separating hosted connectivity through an existing red from a future authorized red.
- [ ] Describe services available before authorization, services requiring partícipe authorization, and functions reserved for an authorized red.
- [ ] Summarize legal, regulatory, operational, and technical requirements without reproducing the dossier checklist.
- [ ] Insert the concise regional evidence from Task 2.
- [ ] Describe Caudal's evidence and capabilities with explicit `Existe`, `Diseñado`, and `Por construir` statuses.
- [ ] Close with the open questions and source methodology, without a sales call to action.

### Task 4: Integrate the document into the workstream

**Files:**
- Modify: `workstream/authorized-red/README.md`

- [ ] Add `resumen-ejecutivo.md` to the document map.
- [ ] Describe it as the institutional overview for fintechs, cooperatives, and core-banking vendors.
- [ ] Keep `business-plan.md` described separately as the investor and partner document.

### Task 5: Verify editorial and factual quality

**Files:**
- Verify: `workstream/authorized-red/resumen-ejecutivo.md`
- Verify: `workstream/authorized-red/README.md`

- [ ] Search for unsupported deadlines, market figures, authorization claims, and claims that designed software already exists.
- [ ] Confirm every regional metric has a visible source and every regulatory statement has a resolution/article or Catálogo section.
- [ ] Confirm P1–P7 remain questions where the repository still treats them as unresolved.
- [ ] Check that Bold and Conekta do not appear and that regional evidence remains subordinate to the Ecuador narrative.
- [ ] Read the Spanish aloud for translated-English constructions and apply `docs/spanish-style.md`.
- [ ] Render or inspect both Mermaid diagrams for valid syntax and beginner readability.
- [ ] Confirm all relative links resolve and `business-plan.md` has no diff.
