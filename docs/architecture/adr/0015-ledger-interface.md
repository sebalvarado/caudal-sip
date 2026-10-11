# 0015. The ledger interface: one synchronous rule core behind an async port

- **Status**: Accepted
- **Date**: 2026-10-10
- **Deciders**: Founding maintainer
- **Issue**: [#20](https://github.com/sebalvarado/caudal-sip/issues/20), part of [#17](https://github.com/sebalvarado/caudal-sip/issues/17)
- **Refines**: [0011](0011-rust-monorepo.md) and [0012](0012-crate-boundaries.md), on what "synchronous" means for a domain crate

## Context

[`ledger.md`](../ledger.md) still showed a Go interface with a reservation timeout. Three accepted records pull on its Rust replacement. [ADR-0011](0011-rust-monorepo.md) makes domain crates synchronous and free of Tokio. [ADR-0007](0007-postgres-first.md) makes `sqlx` the query layer, and `sqlx` is async. [ADR-0012](0012-crate-boundaries.md) asks every domain crate for a storage trait with an in-memory implementation. A synchronous trait cannot be implemented on `sqlx` without blocking a runtime thread. An async trait in a domain crate looks like it breaks ADR-0011.

[ADR-0013](0013-ledger-is-the-aggregate.md) has meanwhile fixed the semantics. `Reserve`, `Post`, and `Void` are ledger commands. A reservation is keyed by the payment id, and a repeat returns the committed outcome. Time passing never ends a reservation.

## Decision

**The rules live once, in a synchronous core inside `ledger`.** Given the commands and the current state of the accounts and reservations a command touches, the core returns the entry to append or a refusal. It does no I/O, never awaits, and reads no clock. Every invariant in [`ledger.md`](../ledger.md) is enforced here, and the property tests drive the core directly.

**The port is a runtime-agnostic async trait, `Ledger`, in the same crate.** Its methods return `impl Future + Send`. The crate depends on no async runtime: an `async fn` needs no Tokio. Three implementations exist:

- `MemLedger`, in `ledger`, runs the core over in-memory maps. The property tests and the `caudal` terminal use it.
- `ledger-postgres` opens one transaction, locks the accounts the command touches in a fixed order, loads their state, calls the core, appends the result, and commits.
- `ledger-tigerbeetle` translates commands into linked pending transfers and lets the engine enforce the invariants. The differential suite proves that it agrees with the core.

"Synchronous" in ADR-0011 and ADR-0012 now means that **domain rules never await**. A domain crate may declare an async port. It still depends on no runtime, database, or network crate, and `cargo deny` keeps it that way. ADR-0012's "storage trait" is this port.

**Errors separate a refusal from not knowing.** Every method returns `Result<T, LedgerError>`:

- `Refused(Refusal)` is final: the ledger said no, and nothing was committed. Each `Refusal` variant is defined below.
- `Unknown` means the call ended without saying whether it committed (a lost connection or a timeout). The caller repeats the same command, which is safe because every command is idempotent. An `Unknown` is never treated as a refusal, so a timeout cannot become a void.

| `Refusal` | When | Catálogo code |
|---|---|---|
| `InsufficientHeadroom` | The headroom debit would go below zero | `AM04` (Tabla 12: the malla, not the customer) |
| `Conflict` | The payment id is already held with a different amount or beneficiary BIC, or a transfer id with a different entry | Decided with the message profile ([ADR-0013](0013-ledger-is-the-aggregate.md)) |
| `NotReserved` | `Post` or `Void` on a payment id with no reservation | None; a caller defect |
| `Irrevocable` | `Void` on a posted reservation (Catálogo Operacional §11.3) | None; a caller defect |
| `Voided` | `Post` on a voided reservation | None; a caller defect |

Only `AM04` is fixed by the catálogo. `payments` maps every other outcome to a code.

**A reservation has no timeout.** `Reserve` carries none, TigerBeetle pending transfers use timeout `0`, and Postgres has no expiry job. A reservation still open at the reconciliation close is an alert raised by the reconciliation projection, not something the ledger acts on.

**The ledger stamps its own commits.** Each commit gets a commit time that strictly increases within the ledger. Reads take an `AsOf`, which is either the latest state or a commit time, so a balance "as of 18:00" means the commits made by 18:00. Implementations are given the time source when they are built, which is how the terminal's simulated clock reaches them ([ADR-0017](0017-time-calendar-identifiers.md)).

## Alternatives considered

- **No port at all: the core returns a commit, and each adapter persists it.** The purest option, but every caller of the ledger would have to know which adapter to drive, and the persistence glue would spread into apps, which [ADR-0012](0012-crate-boundaries.md) keeps as wiring only.
- **A synchronous trait with a blocking Postgres driver.** Rejected. It drops `sqlx` ([ADR-0007](0007-postgres-first.md)) and blocks runtime threads on the most latency-sensitive call in the system.
- **Keep a timeout on the reservation as a safety net.** Rejected by [ADR-0013](0013-ledger-is-the-aggregate.md). In exceptions 3 and 4 the payment is already irrevocable, and an expiry would release headroom for a committed obligation.
- **A separate `ReservationId` minted by the ledger.** Rejected. The payment id already identifies the payment network-wide, and a second id would let a retry create a second reservation.

## Consequences

Easier: one place to read every ledger rule; property tests without a database; three backends that cannot disagree without failing the build.

Harder: async traits with `+ Send` futures are verbose until Rust's return-type notation stabilizes. Postgres must load exactly the accounts a command touches, so every command has to declare them.

Committed: the trait in [`ledger.md`](../ledger.md); `Unknown` is never mapped to a rejection; no reservation expiry in any backend.
