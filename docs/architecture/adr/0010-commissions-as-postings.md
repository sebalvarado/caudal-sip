# 0010. Commissions and taxes are first-class postings, quoted before they are charged

- **Status**: Accepted
- **Date**: 2026-08-11
- **Deciders**: Founding maintainer

## Context

The Catálogo Operacional treats fees as a real, parameterized flow rather than an afterthought ([`../research/catalogo-operacional.md`](../../research/catalogo-operacional.md) §2.1 item 18, §5 rules 2 and 12, §8, §9 rule 5): commissions are parameterized in the SIP, calculated by it, **displayed to the payer including taxes before confirmation**, and `ChrgBr` defaults to `DEBT` — the payer bears the cost.

Two requirements fall out of that. Fees must be **computable before the payment is committed**, because the customer has to see the amount and confirm it. And what the customer saw must be exactly what is charged, months later, when someone asks why.

## Decision

**Every commission and tax is a posting leg in the same atomic entry as the payment obligation.** Not metadata, not a field on the payment, not a figure recomputed at settlement time. Linked transfers in TigerBeetle; one balanced journal entry in Postgres. A payment and its fees either all post or none do.

**A tariff module owns fee schedules**, effective-dated, versioned, and changeable without redeployment. It exposes a quote operation used before confirmation.

**Quote, then commit.** The quoted fee is **snapshotted onto the payment** exactly as the credencial is snapshotted ([`../data-model.md`](../data-model.md) §3), so a payment always explains its own fee using the tariff in force when it was made, regardless of later changes.

Fee obligations settle in the same malla as the payments that generated them.

## Alternatives considered

- **Compute fees at settlement time** — simpler ledger during the day, but it cannot satisfy the pre-confirmation disclosure requirement, and it makes reconciliation between what the customer was shown and what was charged a permanent source of disputes.
- **Fees as metadata on the payment, posted in aggregate later** — loses atomicity, so a crash between payment and fee posting produces revenue that exists in one system and not the other. This is the classic way ledgers become unauditable.
- **A single net posting (payment plus fee combined)** — smaller ledger, but the fee is then unextractable for reporting, tax, and dispute purposes, and it breaks the ability to state the interbank obligation separately from scheme revenue.

## Consequences

**Entries become multi-leg**: interbank obligation, commission, and tax, atomically. The chart of accounts must include fee receivable and tax payable accounts ([`../ledger.md`](../ledger.md)).

**The headroom reservation covers amount plus fees**, since the payer's institution is committing to both.

**Historical fee explainability is guaranteed** by the snapshot, which is also what makes the participant console able to answer a customer's "why was I charged this" without archaeology.

Easier: reporting, tax handling, disputes, and any future policy analysis of fee levels — including the free-P2P argument, which now has a data path instead of a slogan. Harder: a tariff module that did not exist in the component list, and more legs to test. Committed: no fee is ever charged that was not quoted, and no quote expires silently — an expired quote forces re-quotation, matching the catálogo's 60-second confirmation window.
