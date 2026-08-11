# The Tariff Module

A component that did not exist in the original blueprint. It exists because the Catálogo Operacional treats commissions as a parameterized flow calculated by the SIP and **shown to the payer with taxes before confirmation** (§2.1 item 18, §5 rules 2 and 12, §9 rule 5), and because [ADR-0010](adr/0010-commissions-as-postings.md) makes every fee a posting rather than a computed report.

Small module, disproportionately load-bearing: it decides what every payment in the country costs, and it must be able to answer, years later, why a specific customer was charged a specific amount.

## Responsibilities

1. Hold **effective-dated fee schedules**, changeable by an administrator without a deployment.
2. **Quote** a fee and its tax for a proposed payment, before the customer confirms.
3. Produce the fee legs that the ledger posts atomically with the payment.
4. Explain any historical charge from the schedule that was in force at the time.

It does not post to the ledger, does not touch the payment path beyond quoting, and holds no balances.

## The schedule

A schedule is a versioned, effective-dated set of rules. It is never edited in place; a change creates a new version with a future effective date, which makes "what was in force on 12 March" a lookup rather than an archaeology project.

Dimensions a rule may key on, all present in the catálogo's own data: payment type or purpose (`Purp/Prtry`), initiation form (`Key`, `Cred`, `QR`), channel (`0001` Web, `0002` APP), amount band, and the ordering integrante's segment. A rule yields a fixed amount, a percentage with floor and cap, or zero.

Two design rules matter more than the shape:

- **Zero is a first-class outcome, not a missing rule.** A free P2P tier must be expressible as policy rather than as an absent configuration, both because it is the outcome worth arguing for and because "no rule found" must remain a hard error.
- **Tax is derived from the commission**, inside this module, at the same moment. Computing it anywhere downstream will eventually produce an entry that does not balance.

Rounding is decided **once, here**, and applied to the commission before the tax is derived from the rounded figure. Half-up to the cent, and the ledger stores integer cents.

## Quote, then commit

```go
type Quote struct {
    ID         QuoteID
    Commission Amount
    Tax        Amount
    ScheduleID ScheduleVersion  // what explains this figure, forever
    ExpiresAt  time.Time
}
```

The quote is issued during payment initiation, displayed to the customer with taxes included, and **snapshotted onto the payment** exactly as the credencial is ([`data-model.md`](data-model.md) §3). The ledger reservation uses the snapshot ([`posting-patterns.md`](posting-patterns.md) §1.1). Nothing recomputes the fee downstream — the quote is the only figure that exists.

**Quote expiry is the catálogo's 60-second confirmation window** (§5). An expired quote forces re-quotation from the resolution step, which is the same restart the catálogo already mandates. This is not extra strictness; it is the existing rule expressed in the one place that can enforce it.

A quote is not a reservation. Issuing one commits nothing and consumes no headroom.

## Administration and audit

Schedule changes are made by an administrator through the operations console, take effect at a future date, and are recorded with the operator's identity, the previous version, and a justification. A change is announced to integrantes before it takes effect — an unannounced fee change is an incident, not a configuration update.

The console shows a schedule diff and, before activation, its effect against a replay of a recent day's real volume. Publishing what a fee change would have cost the country is exactly the kind of artifact that makes the free-P2P argument evidence instead of opinion.

## Why it is separate

Fee policy changes on political and commercial timescales; payment routing changes on engineering ones. Keeping them apart means a fee change is a configuration event with an audit trail rather than a code change, and it keeps the router free of pricing logic. It also means the module can be run standalone to answer policy questions — what would zero-fee P2P below USD 20 cost the scheme, at last month's actual volume — without touching a production payment path.
