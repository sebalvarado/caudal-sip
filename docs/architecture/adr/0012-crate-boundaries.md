# 0012. Crate boundaries and dependency rules

- **Status**: Accepted
- **Date**: 2026-10-10
- **Deciders**: Founding maintainer
- **Issue**: [#18](https://github.com/sebalvarado/caudal-sip/issues/18), first decision of the Domain foundations epic ([#17](https://github.com/sebalvarado/caudal-sip/issues/17))
- **Refined by**: [0014](0014-kernel-types.md) (the kernel set), [0015](0015-ledger-interface.md) (storage ports), [0016](0016-data-ownership-and-audit.md) (who writes audit), [0018](0018-composition-and-message-plane.md) (translation and `south-api`)

## Context

[ADR-0011](0011-rust-monorepo.md) makes the backend a Cargo workspace. In Cargo a crate can only use what its manifest lists, so the crate graph *is* the domain boundary map, enforced at compile time. It has to be decided before agents write code in parallel, or each one draws its own boundaries.

The domains already exist on paper. [`data-model.md`](../data-model.md) names five (identity and participation, llaves and the directory, payments, settlement, audit) and says each owns its tables. [`red.md`](../red.md) says `red` and `sip` are the same libraries wired into different binaries. [ADR-0008](0008-ledger-architecture.md), [ADR-0009](0009-risk-ceiling-as-funded-account.md) and [ADR-0010](0010-commissions-as-postings.md) define the ledger, the risk ceiling and fees. This record turns those into crates and rules.

## Decision

### Layout

Libraries live in `crates/<name>/`, binaries in `apps/<name>/`. The repository root keeps `docs/`, `spec/`, `site/`, `research/` and `workstream/`. Every package is named `caudal-<name>` (imported as `caudal_<name>`), so the two importable libraries publish without name clashes. Architecture checks live in `xtask/`.

### TypeScript in the same repository

`apps/` holds deployables in any language. A Next.js web app lives at `apps/web/`, beside the Rust binaries, and generated TypeScript clients live in `packages/<name>/`.

- **Two package managers, one command surface.** Cargo builds the Rust crates; a pnpm workspace (`pnpm-workspace.yaml`) holds `apps/web`, `site/` and `packages/*`. `just` runs both, and `just ci` gates both. Turborepo is not used; if the TypeScript side grows past a few packages it may be added for that side only, never wrapping Cargo.
- **The contract is `spec/`, not shared code.** The web app talks to the Rust services only through APIs defined in `spec/` (OpenAPI), and imports a client generated from it. Rust types are never exported to TypeScript directly; the published spec stays the single contract, and a breaking change fails CI on both sides.
- **`apps/web` is hosted by us**: the Caudal red's member portal, the public demo, our own operations. Nothing an institution must deploy depends on Node. The participant console that institutions run remains an open decision ([`deliverables.md`](../../product/deliverables.md)) and needs its own ADR.
- **Worktrees** install TypeScript dependencies with `pnpm install --frozen-lockfile` in the same `wt` hook that runs `cargo fetch`; pnpm's shared store keeps that cheap.

The pnpm workspace, the `site/` move off its npm lockfile, the hooks and the CI job land with the first TypeScript package, not before.

### Five layers, dependencies only downward

| Layer | Crates | May depend on | Never |
|---|---|---|---|
| **Kernel** | `types` | nothing internal | any I/O crate |
| **Domain** | `registry`, `directory`, `payments`, `ledger`, `risk`, `fees`, `clearing`, `reconciliation` | kernel; domain crates below it (next table) | `tokio`, `sqlx`, `axum`, `rustls`, `quick-xml`, protocol or adapter crates |
| **Protocol** | `iso20022`, `qr` | kernel | domain crates, async runtimes, databases |
| **Adapters** | `<domain>-postgres`, `msgplane`, `audit`, `north-api` | everything below | apps |
| **Apps** | `sip`, `red`, `gateway`, `caudal` (the terminal), `tck` | everything below | each other |

Domain crates are synchronous and do no I/O ([ADR-0011](0011-rust-monorepo.md)). Each defines its storage trait and ships an in-memory implementation, which the property tests and the `caudal` terminal use. Postgres implementations are separate adapter crates, so `sqlx` cannot reach a domain crate.

### What each domain crate owns

| Crate | Owns | May depend on |
|---|---|---|
| `registry` | Integrantes, redes, BICs, participation status | `types` |
| `directory` | Llave lifecycle, two-phase registration, resolution to a credencial de pago, synchronization between the Directorio Distribuido and the Sistema Central de Llaves, divergence | `types`, `registry` |
| `ledger` | Both ledgers, accounts, entries, reservations, the invariants of ADR-0008 | `types` |
| `fees` | Effective-dated schedules, quotes, fee and tax legs ([`fees.md`](../fees.md)) | `types` |
| `risk` | Ceiling calculation, headroom funding, the traffic light (ADR-0009) | `types`, `ledger` |
| `clearing` | Malla accumulation, netting, settlement instructions | `types`, `ledger` |
| `payments` | The payment state machine with the catálogo's states, idempotency on `(BIC, end-to-end id)`; drives reserve, post and void | `types`, `registry`, `directory`, `ledger`, `fees` |
| `reconciliation` | The `por conciliar` queue and the daily close | `types`, `payments`, `ledger`, `directory` |

One `payments` crate serves `sip`, `red` and `gateway`, so every profile runs the same catálogo rules; the profiles differ in wiring, not in what a state means. There is no `router` crate: the SIP's router and the red's routing ([`red.md`](../red.md)) are `payments` composed with `msgplane` inside each app, with the split settled in [#23](https://github.com/sebalvarado/caudal-sip/issues/23). The red's membership and certification workflow belongs to `registry`.

### Rules

1. **The kernel holds only what the catálogo defines and every domain uses** (money, BIC, end-to-end id, payment state, reason code, llave type). Anything used by one domain stays in that domain. The exact set is the next decision ([#19](https://github.com/sebalvarado/caudal-sip/issues/19)).
2. **Protocol crates know nothing of the domain.** Translation between ISO 20022 and domain commands happens in the adapter layer, so the unpublished catálogo técnico, when it arrives, changes `iso20022` and the translation and nothing in the domain. Where that translation lives is part of [#23](https://github.com/sebalvarado/caudal-sip/issues/23).
3. **Domain crates do not depend on `audit`.** They return the events they produce; apps hand them to `audit`. Its storage is decided in [#21](https://github.com/sebalvarado/caudal-sip/issues/21).
4. **Apps are wiring only**: configuration, lifecycle, composition. Logic found in an app moves down a layer.
5. **Each crate has an `AGENTS.md`** stating what it owns and what it must never depend on, created with the crate.

### Enforcement

- `cargo deny` bans `tokio`, `sqlx`, `axum`, `hyper` and `rustls` except under adapter and app crates, and `quick-xml` except under `iso20022`, using `[bans]` wrappers.
- `cargo xtask arch` reads `cargo metadata` and fails on any internal edge not allowed by the two tables above. `just ci` and CI run it.

### Names

The fees module is the crate `fees`; [ADR-0010](0010-commissions-as-postings.md) calls it the *tariff module*, and `tariff.md` is now [`fees.md`](../fees.md). The llave service is the crate `directory`, matching the catálogo's Directorio Distribuido; its types keep the word `Llave`, per the glossary.

## Alternatives considered

- **Top-level crate directories** (`ledger/`, `router/`), as the blueprint first drew. Rejected: thirty directories at the root bury `docs/`, `spec/` and `site/`.
- **A state machine per profile.** Rejected: three readings of the same catálogo states would drift, and the conformance kit would have to test all three.
- **No kernel; each domain exports its own types.** Rejected: `Usd`, `Bic` and `ReasonCode` would end up owned by whichever crate defined them first, and every domain would depend on it.
- **One crate per layer.** Rejected: it gives up the compile-time boundary between domains, which is the point.
- **Turborepo over everything, or a separate repository for the web app.** Rejected: Cargo already is the Rust build graph, and a second repository breaks the single-source-of-truth rule and lets the spec and its client drift.

## Consequences

Easier: a wrong dependency fails `cargo build` or `just ci`, not a code review; agents working on different domain crates cannot collide; the terminal composes the real crates in-process, because the domain has no I/O.

Harder: more manifests to maintain; a change to `types` rebuilds everything, which keeps the kernel deliberately small; translation code between ISO 20022 and the domain is written by hand.

Committed: the layout and tables above, `cargo xtask arch` in CI from the first crate, and an `AGENTS.md` per crate. The blueprint's repository shape follows this record.
