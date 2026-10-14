# caudal-ledger — the scheme's two books

**Owns** the only transactional aggregate ([ADR-0013](../../docs/architecture/adr/0013-ledger-is-the-aggregate.md)): the chart of accounts, entries, reservations, and the invariants of [`ledger.md`](../../docs/architecture/ledger.md). Three parts ([ADR-0015](../../docs/architecture/adr/0015-ledger-interface.md)):

- `core` decides each entry or refusal. Pure: no I/O, no clock.
- `Ledger` is the async port. Its futures are `Send` and tied to no runtime.
- `MemLedger` is the reference backend, used by every domain test and by the `caudal` terminal.

**Never depends on** any Caudal crate but `caudal-types`, or on `tokio`, `sqlx`, `axum`, `hyper`, `rustls`, or an adapter crate.

**Rules**

- Every backend runs `core`. A backend only loads state under its lock and appends what `core` decides. Don't re-implement a rule in a backend.
- The feature `testing` exports the property suite. Every backend runs it, and a new backend is differential-tested against `MemLedger` (`caudal-ledger-postgres/tests`).
- A refusal is final; `LedgerError::Unknown` is never treated as one.
- No reservation timeout, ever. Only a post or a void ends a reservation.
- A repeated command returns `Effect::Replay` and appends nothing. Undo a transfer with `reverse`, which mirrors every leg. Do not void a posted payment.
