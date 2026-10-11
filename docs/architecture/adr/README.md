# Architecture Decision Records

Numbered, append-only records of technical decisions. An ADR explains **what** was decided, **why**, and **what alternatives were rejected** — so the decision doesn't get relitigated by accident and newcomers (human or agent) inherit the reasoning, not just the outcome.

## Process

1. Copy `0000-template.md` → `NNNN-short-title.md` (next number).
2. Open a PR. Discussion happens on the PR.
3. Merged = **Accepted**. To reverse a decision, write a *new* ADR that supersedes the old one (mark the old one `Superseded by NNNN`). Never edit an accepted ADR's substance.

ADRs cover component-level decisions. Scheme-level behavior (message profiles, llave rules, settlement semantics) goes through the heavier [RFC process](../../rfcs/README.md).

## Index

| # | Title | Status |
|---|---|---|
| [0001](0001-go-monorepo.md) | Go, single monorepo | Superseded by 0011 |
| [0002](0002-apache-license.md) | Apache-2.0 license | Accepted |
| [0003](0003-iso20022-profile-driven.md) | ISO 20022 XML on the wire, profile-driven message library | Accepted |
| [0004](0004-two-api-surfaces.md) | Two API surfaces: North (ours) and South (mandated) | Accepted |
| [0005](0005-settlement-reference-model.md) | Reference settlement is RTGS-style; DNS as simulation mode | Superseded by 0009 |
| [0006](0006-naming-and-positioning.md) | Naming: RPI EC (standard) + Caudal (brand); no blockchain framing | Accepted |
| [0007](0007-postgres-first.md) | Postgres-first persistence | Accepted |
| [0008](0008-ledger-architecture.md) | Ledger architecture: one bounded service, event log of record, two backends | Accepted |
| [0009](0009-risk-ceiling-as-funded-account.md) | Risk ceiling as a funded control account; one ledger for both settlement models | Accepted |
| [0010](0010-commissions-as-postings.md) | Commissions and taxes as first-class postings, quoted before charged | Accepted |
| [0011](0011-rust-monorepo.md) | Rust, single monorepo | Accepted |
| [0012](0012-crate-boundaries.md) | Crate boundaries and dependency rules | Accepted |
| [0013](0013-ledger-is-the-aggregate.md) | The ledger is the consistency boundary; a payment is a process | Accepted |
| [0014](0014-kernel-types.md) | Kernel types: catálogo vocabulary that cannot hold an invalid value | Accepted |
| [0015](0015-ledger-interface.md) | The ledger interface: one synchronous rule core behind an async port | Accepted |
| [0016](0016-data-ownership-and-audit.md) | One schema per domain, append-only by privilege, one audit chain per schema | Accepted |
| [0017](0017-time-calendar-identifiers.md) | Time is an argument, the calendar is data, identifiers come from the catálogo or the edge | Accepted |
| [0018](0018-composition-and-message-plane.md) | The same nodes run in one process or many; only transport, clock, and storage change | Accepted |
| [0019](0019-ci-on-azure-pipelines.md) | Rust CI runs on Azure Pipelines; the code stays on GitHub | Accepted |
