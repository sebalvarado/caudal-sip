# 0011. Build the backend in Rust, in the same monorepo

- **Status**: Accepted
- **Date**: 2026-10-09
- **Deciders**: Founding maintainer
- **Supersedes**: the language half of [0001](0001-go-monorepo.md). The single-repository half of 0001 stands and is restated here.

## Context

[ADR-0001](0001-go-monorepo.md) chose Go because a cooperativa must be able to run one static binary, and because Go has a larger contributor pool in Ecuador than Rust. The operability constraint is unchanged and is restated in [`../technical-requirements.md`](../technical-requirements.md): a small IT team runs the process; they do not write it. [`../../product/deliverables.md`](../../product/deliverables.md) makes the same split. Institutions deploy `sip`, `red`, and `gateway`, or call the North API. Generated SDKs, not the server language, are how Java and TypeScript core-banking systems integrate.

Almost no backend code exists yet. `cmd/sip` prints a version, and the domain packages are comments. The cost of switching is documentation and the toolchain, not a rewrite.

The binding technical property of this system is correctness under the catálogo's state machine and the ledger invariants in [ADR-0008](0008-ledger-architecture.md): postings are never mutated, balances are derived, the two ledgers do not cross, and a timeout must not release a reservation on its own. Rust encodes those constraints in the type system. The founding maintainer will write the system and is learning the language on this codebase. [ADR-0001](0001-go-monorepo.md) weighed a future contributor pool more heavily than that. This record accepts the pool cost.

Throughput does not motivate the change. The engineering budget in the technical requirements (500 TPS sustained, 2,000 peak) is a Postgres problem either way.

## Decision

All backend components are written in Rust and live in this single repository, next to `docs/`, `spec/`, and `site/`. The repository is a Cargo workspace. Each component directory in the [blueprint](../blueprint.md) that contains code is a crate. Binaries (`sip`, and later `red`, `gateway`, `tck`, `caudalctl`) are workspace members. Each shipped service is a release binary from `cargo build --release` plus a container image. There is no Rust runtime for an operator to install.

Domain types use enums and newtypes so illegal states and mixed units do not compile: payment state, `Usd` distinct from headroom, reason codes. Storage access is a trait per component, as [ADR-0007](0007-postgres-first.md) required of its Go interfaces. The ISO 20022 library in [ADR-0003](0003-iso20022-profile-driven.md) is a Rust crate: profile data, codecs, golden files, and parser fuzzing. `quick-xml` is the XML stack. moov-io's notes on namespace handling stay useful as prior art and are not the implementation.

Async I/O stays at the edges (HTTP, mTLS, Postgres). The ledger and other domain crates are synchronous and do not depend on Tokio.

The Go module, the `sip` stub, the Go CI workflow, and the Go Makefile have been removed. The Cargo workspace lands with the ledger, the first component.

## Alternatives considered

- **Stay on Go ([ADR-0001](0001-go-monorepo.md))** — still the better hiring story, and enough for the latency budget. Rejected because the implementation has not started, the type system matches the ledger invariants, and the person building it is choosing Rust deliberately.
- **Rust for the ledger, Go for the gateway and the message library** — two toolchains, two CI pipelines, and a boundary that has to be FFI or another network hop. Rejected while the team is one person.
- **Edit 0001 in place** — rejected. Accepted ADRs are append-only; a reversal is a new record.

## Consequences

Easier: illegal payment states and mixed monetary units fail at compile time; immutable postings are the default; `cargo test`, `cargo clippy`, and `cargo-fuzz` match the correctness bar already required (golden files, property tests, parser fuzzing); TigerBeetle's Rust client fits the second ledger backend in ADR-0008.

Harder: fewer contributors who already know the language; ISO 20022 codecs are written here rather than adapted from a Go library; async Rust is a steeper edge than goroutines, which is why domain crates stay synchronous. The two importable libraries ship as crates. Institutions that cannot link Rust continue to call the North API or use a generated SDK, which was already the common case.

Committed: Cargo workspace on `main`; `cargo test --workspace` and `cargo clippy --workspace -- -D warnings` green; `Cargo.lock` committed; a short dependency list, reviewed on each addition, consistent with the supply-chain requirement. Not committed: a rewrite of docs that still describe the stack as Go (the blueprint, the technical requirements, deliverables, the action plan, the site, and the authorized-red workstream). `AGENTS.md` and `CONTRIBUTING.md` already name the Cargo loop. Those are follow-up edits so the repo does not contradict this record. The build order is unchanged: the ledger core is first ([`../../plan/core-build-order.md`](../../plan/core-build-order.md)).
