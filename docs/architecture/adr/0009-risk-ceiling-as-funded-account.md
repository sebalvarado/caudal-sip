# 0009. The risk ceiling is a funded control account, and both settlement models share one ledger

- **Status**: Accepted
- **Date**: 2026-08-11
- **Deciders**: Founding maintainer
- **Supersedes**: [0005](0005-settlement-reference-model.md)

## Context

ADR-0005 was written before we had the Catálogo Operacional. It assumed Ecuador would *likely* use deferred net settlement and decided to implement RTGS as the reference model with DNS as a separate simulation mode.

The catálogo settles the question and adds a mechanism we had not modeled ([`../research/catalogo-operacional.md`](../../research/catalogo-operacional.md) §9–§10). Settlement **is** deferred net on a multilateral malla, liquidated several times a day in BCE accounts. The credit risk this creates is contained by a **liquidity-risk engine**: each integrante has a *nivel de riesgo aceptable* derived weekly from its reserve requirement, the SIP tracks committed outgoing value against it with a traffic light, and **at 100% the SIP automatically blocks the institution's outgoing payments**, with manual re-enablement.

Implemented naively, that ceiling is imperative logic in the payment path: compute, compare, block. That is bypassable, hard to test exhaustively, prone to check-then-act races, and it duplicates state the ledger already holds.

## Decision

**Model the nivel de riesgo aceptable as a funded control account per integrante, on its own ledger, with a hard no-overdraft invariant.**

The weekly calculation *credits* the institution's headroom account. Every outgoing obligation *debits* it as part of the same atomic reservation that creates the payment obligation. A payment that would take headroom below zero is refused by the storage engine — which **is** the red semáforo, enforced by the database rather than by us. The 75/90/100% thresholds become balance queries; alerts become a projection.

The risk service therefore stops being a gatekeeper and becomes a **funding service**: ingest reserve requirements, poll the BCE current-account balance on the catálogo's 30-minute cadence, recompute ceilings weekly, adjust headroom accounts, emit warnings.

**Both settlement models then run on one ledger**, differing only in how headroom is funded and when positions settle:

| | Ecuador's netting model | Gross prefunded model (Pix-style) |
|---|---|---|
| Headroom funded by | Weekly ceiling derived from encaje | Actual prefunded balance at the central bank |
| Settlement | Multilateral malla close, several times daily | Per transaction, immediately |
| Invariant | Identical — no overdraft | Identical — no overdraft |

Same accounts, same postings, same invariant. The comparative risk analysis we intend to publish becomes a configuration difference rather than a second implementation.

## Alternatives considered

- **Imperative check in the router before posting** — the obvious approach, and what a literal reading of the catálogo suggests. Rejected: it can be bypassed by any code path that forgets it, and the gap between checking and posting is a race that shows up exactly under the load that matters.
- **A separate limits service consulted synchronously** — cleaner than inline logic, but adds a hop inside the latency budget and still has the check-then-act race.
- **Keeping RTGS and DNS as two separate implementations** (ADR-0005's approach) — rejected because the funded-control-account model collapses them into one, and two implementations of settlement means two places for the invariants to diverge.

## Consequences

**The blocking rule cannot be bypassed**, because it is not a rule — it is an invariant.

**Ceiling decreases need explicit handling.** If a weekly recompute lowers a ceiling below the institution's already-committed value, the headroom debit cannot succeed without violating the invariant. The funding operation debits only the available balance and records the remainder as a pending reduction applied as existing commitments clear. Existing commitments always stand; only new ones are blocked. This case must have a test.

**Re-enablement stays manual**, per the catálogo — the funding service never auto-credits headroom back after a block.

Easier: correctness, testability, and quantifying the two settlement models against each other. Harder: operators must understand that headroom is not money, which the console must make obvious. Committed: the reserve is atomic across the obligation ledger and the headroom ledger, or it does not happen at all.
