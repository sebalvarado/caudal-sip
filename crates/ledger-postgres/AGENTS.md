# caudal-ledger-postgres — the ledger on Postgres

**Owns** the `ledger` schema ([ADR-0016](../../docs/architecture/adr/0016-data-ownership-and-audit.md)): its migrations (`migrations/`, embedded, tracked in `ledger._sqlx_migrations`), the `ledger_runtime` role, and `PgLedger`.

**Never depends on** an app, or on an adapter other than `audit`. It never holds a rule of its own: decisions come from `caudal_ledger::core`.

**Rules**

- Append only. The runtime role has `SELECT` and `INSERT` on `entry` and `leg`, and nothing else. A test asserts the grants. Balances are sums over the `ledger.posting` view and are never stored.
- Every command runs in one transaction that holds the ledger's write lock. That lock is a superset of ADR-0015's per-account locks: settlement stays single-writer, and commit times strictly increase.
- Queries are literal SQL with bind parameters. sqlx 0.9 rejects dynamic strings.
- Tests need `DATABASE_URL` (`just db`) and create a fresh database per case. The differential suite must agree with `MemLedger` exactly.
- **Not yet**: the per-schema `audit_event` chain, which lands with the `audit` crate.
