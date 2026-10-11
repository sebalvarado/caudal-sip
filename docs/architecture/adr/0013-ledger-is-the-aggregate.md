# 0013. The ledger is the consistency boundary

- **Status**: Accepted
- **Date**: 2026-10-10
- **Deciders**: Founding maintainer
- **Issue**: [#17](https://github.com/sebalvarado/caudal-sip/issues/17)

## Context

A payment must not leave the books half-updated. Reserving an outgoing payment consumes headroom and creates a pending obligation together, or not at all ([ADR-0008](0008-ledger-architecture.md), [ADR-0009](0009-risk-ceiling-as-funded-account.md)). A timeout must not release that reservation on its own. The catálogo then moves the payment to `ACSP` when the obligation is posted, and to `ACSC` when the malla settles.

Something has to be the single place those facts are committed. If the payment, the obligation ledger, and headroom can each commit on their own, the atomic reserve has to be rebuilt with retries and compensation, which is how a payment rail loses money.

## Decision

The ledger is the only transactional aggregate. One commit covers both ledgers: obligations and headroom.

A payment is a process. It tells the ledger to reserve, post, or void, and it records the messages around that. It does not commit money movement itself.

The ledger is the book of record. A payment remembers the reservation, the payment id, and the messages, and nothing about its own status. Where it stands is derived from the ledger and those messages. Where a stored copy and the ledger would disagree, the copy is not kept. "The payment lags" means the process has not yet read the commit. It does not mean two truths are allowed to stand.

A reservation ends only when the ledger posts it or voids it. Time passing is not a void.

A reservation is identified by the ordering integrante's BIC and the payment id: the code the catálogo has the ordering integrante generate for every payment, carried in `PmtId/InstrId` (Catálogo Operacional §7, §8). `Reserve` with a pair the ledger already holds returns that reservation and does not move headroom again. `Post` and `Void` on a reservation already in that state return the committed outcome. A repeated call is not a second payment. The same pair is the same payment only when the amount, the beneficiary's BIC, and the account the llave resolved to are unchanged. A difference in any of those is not a retry and not an edit: it is refused, and the first reservation stands. The ledger compares what it books, the amount and the beneficiary's BIC. The payment compares the account against its credencial snapshot before it calls `Reserve`, because no account number enters the ledger ([`posting-patterns.md`](../posting-patterns.md) §6). The quoted fee is not compared. The scheme computed it, and [ADR-0010](0010-commissions-as-postings.md) already snapshots it onto the payment. Which catálogo reason code that refusal uses is decided with the message profile, not invented here. A posted reservation cannot be voided to undo it; irrevocability at `ACSP` already forbids that (Catálogo Operacional §11.3).

## Alternatives considered

- **The payment is the aggregate, and the ledger lives inside it.** One transaction, and no way for clearing, risk, or the directory to change without passing through the payment model. Rejected. Those are separate domains ([ADR-0012](0012-crate-boundaries.md)).
- **Payment, obligations, and headroom are three aggregates coordinated by a saga.** The most room to scale and deploy them separately, and the atomic reserve becomes compensation logic. Rejected. The invariant is the product.

## Consequences

Easier: the no-overdraft rule and the paired postings stay inside one commit, and can be tested without the payment process. Clearing and risk call the same ledger. The payment process can be restarted from the ledger.

Harder: every screen that says where a payment is must read a projection built from the ledger and the messages. The projection can be dropped and rebuilt. A contradiction with the ledger is a defect in the projection, and the rebuild is the fix. There is no status column that outranks a posting.

Committed: `Reserve`, `Post`, and `Void` are ledger commands. The payment process is their caller, not their owner. Scalability work that wants to split the write path has to keep this one commit intact. Ecuador's volumes fit one Postgres ledger ([ADR-0007](0007-postgres-first.md)); splitting it is not a goal.
