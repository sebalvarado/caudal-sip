# Core Build Order

The sequence for building the central system — the SIP-equivalent the BCE would run. This is what changed when we decided the no-overdraft rule is a database invariant ([ADR-0009](../architecture/adr/0009-risk-ceiling-as-funded-account.md)) and commissions are first-class postings ([ADR-0010](../architecture/adr/0010-commissions-as-postings.md)).

> **Scope note.** [`action-plan.md`](action-plan.md) describes the participant-facing strategy — message library, gateway, TCK — written before the decision to build the central system. Both tracks are real and neither replaces the other; [`epics.md`](epics.md) reconciles them into one roadmap to 2026-11-30. Still pending: revisiting [`../product/north-star.md`](../product/north-star.md) and [`../product/stakeholders.md`](../product/stakeholders.md), which still assume participant-first positioning.

## What changed

Previously the ledger was late work — Phase 3, after the message library and the gateway, on the assumption it was a bookkeeping concern that could follow the protocol. It cannot. The ledger now enforces the liquidity ceiling synchronously inside payment authorization, which puts it on the critical path of the first payment that ever flows. **The ledger and the fees module are foundations, not later phases.**

The fees module (called the tariff module in ADR-0010) is new; it did not appear in any earlier component list.

## Sequence

Each step ends in something demonstrable, because a step that cannot be shown cannot be verified by anyone outside the project.

**1. Ledger core, Postgres backend.** Accounts, balanced entries, two-phase reservations, the no-overdraft invariant, the append-only event log. No payments yet, no messages, no network. *Proves:* the invariants hold under a property-based suite that tries hard to break them.

**2. Chart of accounts and posting patterns, executable.** [`../architecture/posting-patterns.md`](../architecture/posting-patterns.md) turned into tests: every flow in that document, including all four catálogo exceptions, exercised against the ledger. *Proves:* the design survives contact with the failure modes, particularly exceptions 3 and 4 where money is irrevocably committed and the sender does not know it.

**3. Fees module and quote-then-commit.** Effective-dated schedules, quoting, the snapshot onto the payment, multi-leg entries with commission and tax. *Proves:* what the customer is shown is exactly what posts, and any historical charge is explainable.

**4. Risk engine as a funding service.** Weekly ceiling calculation from reserve requirements, headroom funding, the traffic light as a balance projection, and the ceiling-decrease edge case. *Proves:* the automatic block works and cannot be bypassed, because it is not code that can be forgotten.

**5. TigerBeetle backend, differential testing in CI.** *Proves:* two independent implementations agree. This is the strongest correctness evidence the project will ever have, and it belongs before anything is built on top rather than after.

**6. Clearing engine.** Bilateral and multilateral malla accumulation, netting, settlement instruction generation, position close, headroom release. *Proves:* a day of simulated national traffic nets correctly and settles.

**7. Payment router and the message plane.** Only now — the ISO 20022 surface over a core that is already correct. Idempotency via the payment id (`InstrId`, which holds the ordering BIC), reason codes straight from the catálogo, the state machine wired to reserve, post, and void.

**8. Registry, directory, and resolution.** Llaves, the Sistema Central de Llaves and its two-phase reservation, distributed directory synchronization.

**9. Reconciliation service and operations console.** The `por conciliar` work queue with its 18:00 daily close, directory divergence resolution, headroom monitoring, fee administration.

## Why this order

Steps 1 through 6 have **no network dependency and no dependence on the unpublished catálogo técnico**. They can be built and proven now, while the field-level schemas are still outstanding. The parts that do depend on the técnico — steps 7 and 8 — come after, by which time we may have it.

The order also inverts the usual instinct, which is to build the protocol first because it is the visible part. Building the ledger first means the protocol layer is written against a core whose invariants already hold, rather than a protocol accumulating correctness patches once money starts moving through it.

## The artifact this produces

At step 6 the project can publish something no one else in Ecuador has: a simulation of a day of national payment traffic showing net positions, settlement cycles, and **what the liquidity ceiling does to a segment-3 cooperativa on a payroll day**. Built on public data, verifiable by anyone, and far more persuasive in an interoperability forum than any document. It is also, incidentally, the load-test evidence that a country's payment core runs on ordinary hardware.
