# The Ledger: accounts, invariants, and interface

The design of the scheme's obligation ledger. The ledger is the only transactional aggregate; a payment is a process that calls it ([ADR-0013](adr/0013-ledger-is-the-aggregate.md)). Decisions behind the books themselves are in [ADR-0008](adr/0008-ledger-architecture.md) (architecture), [ADR-0009](adr/0009-risk-ceiling-as-funded-account.md) (risk ceiling as a funded account) and [ADR-0010](adr/0010-commissions-as-postings.md) (fees as postings). The exact postings for every flow are in [`posting-patterns.md`](posting-patterns.md). Regulatory grounding is [`../research/catalogo-operacional.md`](../research/catalogo-operacional.md).

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
3. **Postings are immutable.** No update, no delete. Undoing a transfer is a reversal: a new entry that mirrors every leg and names the original. A posted payment is not reversed this way. It is irrevocable; only a void ends a reservation that is still pending.
4. **Balances are derived** from postings. A materialized balance may exist as a cache with a job that proves it matches.
5. **Reservation is atomic across both ledgers.** The obligation reservation and the headroom debit succeed together or neither happens.
6. **Every entry references the event and the signed wire message that caused it**, so any figure on any screen is traceable to evidence years later.
7. **Idempotency is structural**: the payment id, which holds the ordering BIC, deduplicates network-wide, and a repeated reservation returns the original reservation rather than creating a second ([ADR-0013](adr/0013-ledger-is-the-aggregate.md)).

## The interface

Narrow on purpose — this is the entire surface other components may use. The rules live in a synchronous core; this trait is the async port over it, with no runtime dependency ([ADR-0015](adr/0015-ledger-interface.md), [ADR-0023](adr/0023-replay-purpose-and-reversal.md)).

```rust
pub trait Ledger {
    /// Consumes headroom and creates the pending obligations, atomically
    /// across both ledgers. A repeat with the same payment id returns the
    /// original reservation.
    fn reserve(&self, r: Reserve) -> impl Future<Output = Result<Applied<Reservation>, LedgerError>> + Send;
    /// ACSP. Idempotent.
    fn post(&self, id: PaymentId) -> impl Future<Output = Result<Applied<Reservation>, LedgerError>> + Send;
    /// RJCT. Idempotent. Refused once posted.
    fn void(&self, id: PaymentId, reason: ReasonCode) -> impl Future<Output = Result<Applied<Reservation>, LedgerError>> + Send;

    /// Immediate balanced entry. `Transfer::purpose` names the scheme event.
    /// Idempotent on `Transfer::id`, purpose, and legs.
    fn transfer(&self, t: Transfer) -> impl Future<Output = Result<Applied<EntryId>, LedgerError>> + Send;
    /// Mirror of every leg of a transfer. Not a way to undo a payment.
    fn reverse(&self, of: TransferId, cause: Cause) -> impl Future<Output = Result<Applied<EntryId>, LedgerError>> + Send;

    /// Reads at the latest state or at a past commit time.
    fn balance(&self, account: AccountId, at: AsOf) -> impl Future<Output = Result<Balance, LedgerError>> + Send;
    fn positions(&self, book: Book, at: AsOf) -> impl Future<Output = Result<Vec<Position>, LedgerError>> + Send;
    fn entries(&self, q: EntryQuery) -> impl Future<Output = Result<Vec<Entry>, LedgerError>> + Send;
}

pub enum LedgerError {
    /// Final: nothing was committed.
    Refused(Refusal),
    /// The call ended without saying whether it committed. Repeat the command;
    /// never treat this as a refusal.
    Unknown(Box<dyn Error + Send + Sync>),
}
```

`Reserve` carries the payment id, the beneficiary's BIC, the amount, the quoted fee and tax ([ADR-0010](adr/0010-commissions-as-postings.md)), and the causing event and message hash (invariant 6). It carries no timeout: only `Post` or `Void` ends a reservation. Every idempotent command returns `Applied`: `Effect::Appended` when this call wrote an entry, `Effect::Replay` when the books already held it. A replay is not evidence that the caller already sent a message. `Refusal::InsufficientHeadroom` is the red semáforo and maps to the catálogo's `AM04`, the SIP's code for malla-level insufficiency, not the customer's balance. The other refusals are listed in [ADR-0015](adr/0015-ledger-interface.md). `ReasonCode` is the catálogo's own code — `AC04`, `AM04`, `DT01` — never an internal enum ([ADR-0014](adr/0014-kernel-types.md)). `Book` names one of the two ledgers. `Usd` is the kernel's amount, and the headroom unit is `ledger`'s own type.

A transfer's purpose is one of `headroom`, `settlement`, `suspense`, or `correction`. The same id with a different purpose or different legs is a conflict. `reverse` builds the mirror itself, including headroom, and refuses an id that is not a transfer. A second reverse of the same transfer is a replay. A reversal that would take a constrained account below zero is `InsufficientHeadroom`, and the original entry stays.

## Backend mapping

| Concept | TigerBeetle | Postgres |
|---|---|---|
| Reservation | `pending` transfer, timeout `0` | Appended reservation event plus pending postings; no expiry job |
| Post / Void | `post_pending_transfer` / `void_pending_transfer` | Appended post or void event plus its postings |
| Reversal | a new transfer that mirrors the original, not a void of a pending one | Appended `reversed` entry; the original stays |
| Multi-leg atomicity | `linked` chain | One transaction |
| No overdraft | `debits_must_not_exceed_credits` flag | `CHECK` on a derived balance with serialized writes per account |
| Reason code | `code` field | Column |
| Correlation | `user_data_128` / `user_data_64` | Columns |
| Point-in-time balance | Native historical balances | Aggregate over postings up to a commit time |

Two things to know about the TigerBeetle backend. Clients batch automatically — up to 8,189 events per request — so the throughput story requires no work from us, but a single in-flight request per client means the connection is shared rather than pooled per task. And production means multi-replica consensus; single-replica mode exists for development only.

The Postgres backend must avoid the trap that makes hot-account ledgers slow: **never `UPDATE` a balance column.** Insert postings append-only and derive balances. With a few dozen accounts and append-only writes there is no lock contention to speak of at Ecuador's volumes.

## Differential testing

Both backends implement this interface and run the same property-based suite in CI: no operation sequence produces an unbalanced entry, a negative constrained balance, a lost posting, or divergent balances between the two implementations. **A divergence fails the build.** This is the single strongest correctness argument the project can make, and it is why TigerBeetle is built early rather than deferred.

## Operational consequences

The reserve happens synchronously during payment authorization, so **ledger availability equals inter-red payment availability**, and a posting sits inside the latency budget in [`technical-requirements.md`](technical-requirements.md). The Scheme Core and Network profiles therefore deploy the ledger multi-replica; the sandbox and participant profiles do not run this ledger at all.

The console must make one thing unmistakable to operators: **headroom is not money.** An institution at zero headroom is blocked from sending, but owes and is owed exactly what ledger 1 says.
