# Epics to 2026-11-30

> **Status:** adopted · **Owner:** Sebastián · **Adopted:** 2026-10-09 · **Tracking:** GitHub milestone `2026-11-30`
>
> The single roadmap to the regulatory deadline. It reconciles the participant-first [`action-plan.md`](action-plan.md), the ledger-first [`core-build-order.md`](core-build-order.md), and the licensing track in [`workstream/authorized-red/`](../../workstream/authorized-red/README.md). Where they disagree on scope or sequence before 2026-11-30, this document wins.

Three epics, each serving a different audience, plus one foundation workstream that all three ship through. Every end state is dated **2026-11-30**. Tasks live as GitHub issues under the `2026-11-30` milestone; this file defines only end states, acceptance criteria, and cuts.

Every epic must ship **a public demo and a blog post** before it counts as done. Public proof is a rule of every epic, not an epic of its own.

## Epic 1 — Network in a box

**Audience:** developers, the BCE as a technical reader, the press.

**End state.** Someone clones the repository, runs one command, and within fifteen minutes sends a payment from a mock cooperativa on red 1 to a mock bank on red 2 through `sip`. They watch it reach `ACSP`, then `ACSC` once a malla settles. When the cooperativa's headroom is exhausted, its next payment is blocked by the ledger. The same stack runs live on Azure, driven from a page on the site.

**Acceptance criteria**

- **Ledger crate** (build order steps 1–2): double entry, two-phase reservations, the no-overdraft invariant, append-only event log, Postgres backend. A property-based suite exercises every flow in [`posting-patterns.md`](../architecture/posting-patterns.md), including the four catálogo exceptions.
- **Risk ceiling as a funded account** (step 4, [ADR-0009](../architecture/adr/0009-risk-ceiling-as-funded-account.md)), with reserve data seeded rather than fetched from the BCE.
- **Clearing** (step 6): multilateral malla netting and at least one settlement cycle that can be triggered manually.
- **Router**: a minimal `pacs.008` / `pacs.002` exchange on our own profile, marked **[assumption]** until the catálogo técnico is published. Idempotent on the payment id (`InstrId`, which holds the ordering BIC); returns a subset of the Tabla 12 reason codes.
- **Llaves**: a seeded central key system using the catálogo's llave types; resolution returns the masked name.
- **Topology**: two `red` profiles and at least three mock institutions, started by one command.
- **The `caudal` terminal**: one binary that runs `sip`, two redes, and the mock institutions in-process on an in-memory ledger backend (the same trait as Postgres), with no Docker. A full-screen terminal UI in the spirit of a Bloomberg terminal, for developers and finance professionals on a single machine:
  - a command line to invoke any operation (pay, resolve a llave, settle a malla) and see the request, the ISO 20022 message, and the response;
  - a live payment blotter with each payment's state;
  - per-institution headroom with the catálogo's traffic light, and malla net positions;
  - execution stats per operation: p50/p95/p99 latency, throughput, time from reservation to post;
  - a load generator with named scenarios (a payroll day, a cut-off rush).
  Every operation is also a scriptable subcommand (`caudal pay …`), matching [`local-stack.md`](../development/local-stack.md).
- **Public playground**: the same network running in the visitor's browser, on the site, driven by scenario scripts anyone can edit and share ([ADR-0022](../architecture/adr/0022-browser-playground.md)).
- **Public artifact**: a published simulation of one day of national traffic — net positions, settlement cycles, and what the ceiling does to a segment-3 cooperativa on a payroll day.

**Cut:** the `fees` module (commissions post as zero, but the posting structure of [ADR-0010](../architecture/adr/0010-commissions-as-postings.md) is kept), the TigerBeetle backend, the reconciliation queue, the console.

## Epic 2 — An institution connects

**Audience:** cooperativas, fintechs (SEDPES), core-banking vendors.

**End state.** A developer at a cooperativa or a core-banking vendor follows a Spanish tutorial, runs `gateway` against the sandbox, and through the North API registers a llave, resolves one, sends a payment, and receives one by webhook. They run `tck` and get a pass/fail report in Spanish they could hand to their red.

**Acceptance criteria**

- `spec/` holds a v0 OpenAPI definition of the North API and the red↔gateway message contract.
- `gateway` binary: single tenant, Postgres, outbox, idempotency keys, signed webhooks; mTLS to the red as our default, not as a requirement of the norm.
- `tck`: the happy path, at least five Tabla 12 reason codes, the four exceptions of §11.4, and timeout behaviour.
- A Spanish tutorial, *Conecta una cooperativa en 30 minutos*.
- Every message rule cites the catálogo or is marked **[assumption]**.

**Cut:** generated SDKs, multi-tenant hosting, the console.

**Dependency:** the red↔gateway contract lands in `spec/` in week 1, so this epic builds against a mock red while Epic 1 builds the real one.

## Epic 3 — The Caudal red is ready to file

**Audience:** the BCE as regulator, prospective members.

**End state.** The Caudal operating company could submit its BCE-GG-024-2024 authorization dossier the next day, and a BCE technical visit could be run against the live Azure stack.

**Acceptance criteria**

- Every item in [`dossier-psap.md`](../../workstream/authorized-red/regulatory/dossier-psap.md) is drafted or has a named external owner and date.
- [`technical/readiness.md`](../../workstream/authorized-red/technical/readiness.md) maps each requirement to evidence from Epics 1 and 2: CI runs, tests, the demo.
- Spanish runbooks for the red: operations, incidents, and continuity (a red is a single point of failure for its integrantes — see the observations register in [`../rfcs/README.md`](../rfcs/README.md)).
- At least three written expressions of interest from fintechs or cooperativas in the pipeline. The number is an **[assumption]**; no personal data enters the repository.
- Every item in [`open-questions.md`](../../workstream/authorized-red/regulatory/open-questions.md) is answered or escalated.

**Limit:** incorporation, capital, and signatures belong to the maintainer. Agents draft; they never sign or submit.

## Foundation workstream

Serves all three epics and owns CI, CD, and the site.

- **Workspace**: a Cargo workspace with the toolchain pinned in `rust-toolchain.toml` and `Cargo.lock` committed, in week 1 before any epic code ([ADR-0011](../architecture/adr/0011-rust-monorepo.md)).
- **CI** (GitHub Actions, path-filtered, required on `main`): `cargo fmt --check`, `cargo clippy --workspace -- -D warnings`, `cargo test --workspace` against a Postgres service container, `cargo-deny` for licences and advisories. The existing site build stays. A DCO sign-off check on every PR.
- **CD**: the site, simulator included, keeps deploying to Cloudflare Pages. On a `v*` tag, a GitHub Release with Linux and macOS binaries.
- **Public demo**: the `/simulador/` page runs the network as WebAssembly in the browser; no server, synthetic data only, non-affiliation disclaimer visible ([ADR-0022](../architecture/adr/0022-browser-playground.md), which supersedes the hosted demo of ADR-0020 and ADR-0021).
- **Blog**: at least one Spanish post every two weeks, tied to epic milestones.

## Decisions before code

The **Domain foundations** epic ([#17](https://github.com/sebalvarado/caudal-sip/issues/17)) gates every implementation issue. Seven decisions, made one at a time with the maintainer and recorded as ADRs or doc updates: crate map and dependency rules, shared kernel types, the ledger trait in Rust, data ownership and the audit log, time and identifiers, in-process composition and the message plane, and the open modeling questions. Until they merge, agents only draft decisions.

## How agents work

- **Tracking**: one GitHub issue per epic, the `2026-11-30` milestone, and the epic's acceptance criteria as its checklist. Smaller issues already opened are that checklist. Do not open a new issue for a slice of an epic.
- **Delivery**: one epic, one long-lived branch, one git worktree, until the epic's acceptance criteria are met. Merge that branch back in reviewable pull requests. A slice does not get its own worktree. CI green, then the maintainer reviews and merges. Agents never push to `main`.
- **Parallelism**: epics own directories and run at the same time only when those directories do not overlap. Domain foundations owns `docs/architecture/`. Network in a box owns the domain crates and `apps/caudal`. An institution connects owns `spec/`, `apps/gateway`, and `apps/tck`. The Caudal red owns `workstream/authorized-red/`. The foundation workstream owns the workspace root, `.github/`, and deploy. The map is in the `worktree-management` skill.

## Calendar

Seven weeks from 2026-10-12.

| Week | Foundation | Epic 1 | Epic 2 | Epic 3 |
|---|---|---|---|---|
| 1 | Workspace, CI, the two ADRs | Ledger core, in-memory backend; terminal ADR | red↔gateway contract in `spec/` | Dossier gap list |
| 2–3 | Release pipeline | Posting patterns, risk ceiling, Postgres backend; terminal shell with the ledger | North API v0, gateway against a mock red | Runbooks, pipeline outreach |
| 4–5 | Simulator page | Clearing, router, llaves, sandbox; terminal stats and scenarios | `tck` scenarios | Readiness mapped to evidence |
| 6 | Release v0.1 | Simulation post | Tutorial | Dossier ready |
| 7 | Buffer and hardening | Buffer and hardening | Buffer and hardening | Buffer and hardening |

## Risks

- **Rust learning curve on Epic 1's critical path.** Everything else depends on the ledger.
- **The catálogo técnico is unpublished.** Messages follow our profile, marked **[assumption]**, and get reworked when it appears.
- **A full day is slow in a browser.** Every message is signed and verified, so the national scenario takes tens of seconds there; the page shows progress, and the terminal runs it in seconds.
