# The Ledger: accounts, invariants, and interface

The design of the scheme's obligation ledger. Decisions behind it are in [ADR-0008](adr/0008-ledger-architecture.md) (architecture), [ADR-0009](adr/0009-risk-ceiling-as-funded-account.md) (risk ceiling as a funded account) and [ADR-0010](adr/0010-commissions-as-postings.md) (fees as postings). The exact postings for every flow are in [`posting-patterns.md`](posting-patterns.md). Regulatory grounding is [`../research/catalogo-operacional.md`](../research/catalogo-operacional.md).

## Scope: which ledger this is

This service owns **the scheme's obligation ledger** — irrevocable inter-red obligations accumulating into mallas until they settle. It does **not** own settlement in central bank money (BCE current accounts) and it does **not** own customer balances (each institution's core banking). Those two are external systems this one produces instructions for and reflects, respectively.

Everything here is USD. Ecuador is dollarized, so there is no multi-currency dimension, and we will not add one speculatively.

## Two ledgers, one service

TigerBeetle's `ledger` field separates assets or types of value; transfers never cross ledgers. We use exactly two, and the same separation is modeled explicitly in the Postgres backend.

| # | Ledger | What it tracks | Unit |
|---|---|---|---|
| 1 | `obligations` | Real obligations between integrantes and the scheme, pending settlement | USD |
| 2 | `headroom` | Each integrante's remaining capacity to commit outgoing payments | USD-denominated, **not money** |

Keeping headroom on its own ledger is what makes the risk ceiling an invariant instead of a rule. It also prevents the most dangerous possible confusion — a control figure being mistaken for a balance — from ever being expressible.

## Chart of accounts

Versioned; changing it after launch is a migration on a live ledger, so it is specified up front.

### Ledger 1 — obligations

| Account | Per | Normal balance | Purpose |
|---|---|---|---|
| `ir:{bic}:posicion` | integrante | Either | Net position against the scheme. Debit balance = owes; credit balance = is owed |
| `scheme:comisiones_por_cobrar` | one | Credit | Commission revenue accrued, pending settlement |
| `scheme:impuestos_por_pagar` | one | Credit | Taxes collected on commissions, pending remittance |
| `scheme:liquidado` | one | Either | Contra account that positions close into when a malla settles |
| `scheme:suspenso` | one | Either | Payments in `por conciliar`; every balance here is an open operational item |

### Ledger 2 — headroom

| Account | Per | Constraint | Purpose |
|---|---|---|---|
| `ir:{bic}:headroom` | integrante | `debits_must_not_exceed_credits` | Remaining capacity. **The no-overdraft invariant here is the block** |
| `scheme:headroom_otorgado` | one | — | Contra account funded from weekly ceiling calculations |
| `scheme:headroom_comprometido` | one | — | Where consumed headroom accumulates while obligations are outstanding |

Under the netting model, `ir:{bic}:headroom` is credited weekly from the *nivel de riesgo aceptable*. Under a gross prefunded model it is credited with actual prefunded balance. Nothing else differs.

## Invariants

Enforced by the storage engine wherever possible, verified by property-based tests everywhere else. A violation is a stop-the-line defect, not a bug ticket.

1. **Every entry sums to zero** across its legs, on each ledger independently.
2. **No account with `debits_must_not_exceed_credits` may go negative.** This is the risk ceiling and, under the gross model, the prefunding rule.
3. **Postings are immutable.** No update, no delete. A correction is a new, opposing entry referencing the original.
4. **Balances are derived** from postings. A materialized balance may exist as a cache with a job that proves it matches.
5. **Reservation is atomic across both ledgers.** The obligation reservation and the headroom debit succeed together or neither happens.
6. **Every entry references the event and the signed wire message that caused it**, so any figure on any screen is traceable to evidence years later.
7. **Idempotency is structural**: `(originator BIC, end-to-end id)` deduplicates network-wide, and a repeated reservation returns the original reservation rather than creating a second.

## The interface

Narrow on purpose — this is the entire surface other components may use.

```go
type Ledger interface {
    // Two-phase: reserve consumes headroom and creates pending obligations
    // atomically across both ledgers. Returns ErrInsufficientHeadroom when
    // the ceiling would be breached — that error is the red semáforo, and it
    // maps to the catálogo's AM04, which is the SIP's code for malla-level
    // insufficiency, not the customer's balance.
    Reserve(ctx context.Context, req ReserveRequest) (ReservationID, error)
    Post(ctx context.Context, id ReservationID) error              // ACSP
    Void(ctx context.Context, id ReservationID, code ReasonCode) error // RJCT

    // Immediate balanced entry: funding, settlement, corrections.
    Transfer(ctx context.Context, entry Entry) (EntryID, error)

    // Reads. `at` enables point-in-time queries for reconciliation and audit.
    Balance(ctx context.Context, acct AccountID, at time.Time) (Balance, error)
    Positions(ctx context.Context, ledger LedgerID, at time.Time) ([]Position, error)
    Entries(ctx context.Context, q EntryQuery) ([]Entry, error)
}
```

`ReserveRequest` carries the payment amount, the quoted fee and tax legs ([ADR-0010](adr/0010-commissions-as-postings.md)), both BICs, the end-to-end id, and a timeout after which an unresolved reservation expires. `ReasonCode` is the catálogo's own code — `AC04`, `AM04`, `DT01` — never an internal enum.

## Backend mapping

| Concept | TigerBeetle | Postgres |
|---|---|---|
| Reservation | `pending` transfer with `timeout` | Row in `reservation` plus pending postings, expiry job |
| Post / Void | `post_pending_transfer` / `void_pending_transfer` | State transition plus posting materialization |
| Multi-leg atomicity | `linked` chain | One transaction |
| No overdraft | `debits_must_not_exceed_credits` flag | `CHECK` on a derived balance with serialized writes per account |
| Reason code | `code` field | Column |
| Correlation | `user_data_128` / `user_data_64` | Columns |
| Point-in-time balance | Native historical balances | Aggregate over postings up to a timestamp |

Two things to know about the TigerBeetle backend. Clients batch automatically — up to 8,189 events per request — so the throughput story requires no work from us, but a single in-flight request per client means the connection is shared rather than pooled per goroutine. And production means multi-replica consensus; single-replica mode exists for development only.

The Postgres backend must avoid the trap that makes hot-account ledgers slow: **never `UPDATE` a balance column.** Insert postings append-only and derive balances. With a few dozen accounts and append-only writes there is no lock contention to speak of at Ecuador's volumes.

## Differential testing

Both backends implement this interface and run the same property-based suite in CI: no operation sequence produces an unbalanced entry, a negative constrained balance, a lost posting, or divergent balances between the two implementations. **A divergence fails the build.** This is the single strongest correctness argument the project can make, and it is why TigerBeetle is built early rather than deferred.

## Operational consequences

The reserve happens synchronously during payment authorization, so **ledger availability equals inter-red payment availability**, and a posting sits inside the latency budget in [`technical-requirements.md`](technical-requirements.md). The Scheme Core and Network profiles therefore deploy the ledger multi-replica; the sandbox and participant profiles do not run this ledger at all.

The console must make one thing unmistakable to operators: **headroom is not money.** An institution at zero headroom is blocked from sending, but owes and is owed exactly what ledger 1 says.
