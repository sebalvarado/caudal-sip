# 0016. One schema per domain, append-only by privilege, one audit chain per schema

- **Status**: Accepted
- **Date**: 2026-10-10
- **Deciders**: Founding maintainer
- **Issue**: [#21](https://github.com/sebalvarado/caudal-sip/issues/21), part of [#17](https://github.com/sebalvarado/caudal-sip/issues/17)
- **Refines**: [0012](0012-crate-boundaries.md) rule 3, on who writes the audit record

## Context

[ADR-0007](0007-postgres-first.md) commits every table to exactly one owner. [`data-model.md`](../data-model.md) §5 wants one append-only audit log, written in the same transaction as the change it describes and chained by hash **[proposal]**. [ADR-0012](0012-crate-boundaries.md) keeps domain crates away from `audit` and says apps hand events to it. If the app writes the audit record after the domain commits, there is a moment when a change exists with no audit entry, and the same-transaction rule is broken.

[ADR-0008](0008-ledger-architecture.md) calls the append-only event log the system of record, and [ADR-0013](0013-ledger-is-the-aggregate.md) calls the ledger the book of record. This record says where that log is, so the two statements describe the same tables.

## Decision

**Each domain crate that keeps data owns one Postgres schema, named after the crate**: `registry`, `directory`, `payments`, `ledger`, `fees`, `risk`, `clearing`, `reconciliation`, plus `audit`. An adapter crate that keeps data of its own, such as `north-api`'s idempotency records and webhook deliveries, gets a schema the same way. The schema's tables and migrations live in its adapter crate (`crates/<domain>-postgres/migrations/`), embedded in the binary. Each schema keeps its own migrations table, so a binary migrates only the schemas it composes, at startup, before it serves.

**Roles enforce the boundary.** Each schema has an owner role that only migrations use, and a runtime role. A runtime role has privileges in its own schema and nowhere else. A cross-schema join therefore fails with a permission error instead of passing review, and no transaction can span two schemas. That matches [ADR-0013](0013-ledger-is-the-aggregate.md): the only atomic commit that spans tables is the ledger's, and it stays inside `ledger`. Data another domain needs comes through the owner's crate or through events, never through its tables.

**Append-only is a privilege, not a convention.** On an append-only table the runtime role holds `SELECT` and `INSERT` only. Never `UPDATE`, `DELETE`, or `TRUNCATE`. A test in each adapter crate asserts those grants. State that changes is modeled as appended events, with the current state derived from them. A reservation, for example, is its reserved, posted, and voided events, not a row whose status is updated. Each schema's append-only tables are its event log of record. For the ledger, those are its entries and reservation events, so ADR-0008's log and ADR-0013's book are the same tables.

**Audit is one chain per schema.** Each schema has an append-only `audit_event` table. The adapter that commits a change appends its audit records in the same transaction. Each record carries the hash of the previous record in that schema, so the chain head serializes writes within a schema and nowhere else. The `audit` crate owns the record format, the hashing, the append function that runs inside the caller's transaction, and the verifier. That makes `<domain>-postgres → audit` an edge between two adapter crates, which `cargo xtask arch` allows by name. The other is `south-api → msgplane` ([ADR-0018](0018-composition-and-message-plane.md)). Any further one needs a record. Domain crates still do not depend on `audit`. They return events, and their adapter persists events and change together. Apps no longer write audit records.

**An anchor joins the chains.** On a schedule, and at every malla close and daily close, `audit` records every schema's chain head in an anchor that is itself chained. Readers see one log through a view in the `audit` schema. The `audit` role has `SELECT` on each `audit_event` table and nothing else outside its schema. Verifying means recomputing each chain and checking it against the anchors. What may never enter an audit record is unchanged ([`data-model.md`](../data-model.md) §5).

**Development mirrors production.** Each worktree has one database (`just db`) holding every schema with the same roles. Tests that need Postgres get a fresh database per test on the worktree's server. The gateway's tenant rule is unchanged: every tenant-scoped row carries `tenant_id`, with row-level security on top of the schema boundary.

## Alternatives considered

- **One global hash-chained audit table.** The simplest to verify, but every write in every domain would queue on one chain head, and every runtime role would need rights in a shared schema. Rejected for the per-schema chain plus anchor, which gives the same tamper evidence.
- **Triggers that reject `UPDATE` and `DELETE`.** Rejected as the mechanism. The owner can disable a trigger, the trigger costs time on every row, and Postgres already checks privileges on every statement. A privilege that was never granted cannot be misused.
- **One database per domain.** Stronger isolation, but it multiplies what a cooperativa's four-person IT team has to run ([`technical-requirements.md`](../technical-requirements.md)), for a boundary that roles already enforce.
- **Apps write audit after the domain commits**, as ADR-0012 first said. Rejected. A crash between the two commits leaves a change with no audit record.

## Consequences

Easier: a domain's data can be reasoned about, migrated, and handed to an auditor on its own. A wrong join fails in the first test that runs it.

Harder: one connection pool per schema role in each binary. Migrations have to stay backward compatible for one release, schema by schema. The anchor job is one more scheduled task to monitor.

Committed: schema-per-domain, runtime roles without `UPDATE`, `DELETE`, or `TRUNCATE` on append-only tables, and audit written in the change's own transaction. The hash chain in [`data-model.md`](../data-model.md) §5 is no longer a proposal.
