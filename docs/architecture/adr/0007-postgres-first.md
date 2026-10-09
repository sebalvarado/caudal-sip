# 0007. Postgres-first persistence

- **Status**: Accepted
- **Date**: 2026-08-05
- **Deciders**: Founding maintainer

> The Postgres decision below is unchanged. "Narrow Go interfaces" is restated by [ADR-0011](0011-rust-monorepo.md) as Rust traits. Explicit SQL stays; the query layer is `sqlx`, not an ORM.

## Context

Realistic national volumes are modest by software standards: Bre-B — serving a country three times Ecuador's size — peaks around 5M transactions/day (~60 TPS average). Adopting institutions must be able to operate and audit the storage layer with skills they already have. Trust ("boring and auditable") outweighs exotic performance.

## Decision

PostgreSQL is the storage backend for every stateful component (`llaves/`, `settlement/`, `gateway/`). Schema migrations are versioned and reviewed like code. Storage access sits behind narrow Go interfaces per component — not to chase database portability, but to keep domain logic testable against fakes and to leave room for specialized backends if a component ever proves it needs one.

## Alternatives considered

- **TigerBeetle for the ledger** — purpose-built for double-entry accounting and extremely fast, but young operationally and unfamiliar to institutional DBAs. Revisit behind the settlement interface if volume ever demands it; benchmarking it makes good content regardless.
- **Event store (Kafka/NATS) as primary storage** — event sourcing has appeal for audit trails, but an append-only journal table in Postgres achieves the auditability without a second infrastructure dependency.
- **SQLite for small deployments** — attractive for tiny cooperativas, but multi-tenant hosting and replication needs make Postgres the single answer that covers all deployment profiles.

## Consequences

Easier: operations, hiring, auditing, backup/recovery stories for institutions. Harder: nothing material at target volumes. Committed: no ORM magic — explicit SQL with type-safe generated queries; every table owned by exactly one component.
