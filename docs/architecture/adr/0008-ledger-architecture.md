# 0008. Ledger architecture: one bounded service, event log of record, two backends

- **Status**: Accepted
- **Date**: 2026-08-11
- **Deciders**: Founding maintainer
- **Extends**: [0007](0007-postgres-first.md)
- **Refined by**: [0013](0013-ledger-is-the-aggregate.md) (reservations have no timeout), [0016](0016-data-ownership-and-audit.md) (where the event log of record lives)

## Context

Three distinct ledgers exist in this scheme and conflating them produces bad designs:

1. **Settlement in central bank money** — the integrantes' current accounts at the BCE. Already exists; we never own it. Our clearing engine hands it net positions.
2. **The participant's customer ledger** — their core banking system. The gateway reflects it and never owns it ([`../data-model.md`](../data-model.md)).
3. **The scheme's obligation ledger** — irrevocable inter-red obligations accumulating into bilateral and multilateral mallas until settlement. **This is the one we build.**

Ledger 3 has an unusual shape. It holds very few accounts — a few dozen direct integrantes plus a handful of redes, at most a few hundred — and every payment in the country touches the same handful of them. The engineering problem is **extreme write contention on hot accounts plus absolute correctness**, not volume: Ecuador's realistic peak is on the order of 1,500 TPS.

The Catálogo Operacional also imposes semantics that map directly onto established ledger primitives: the ordering institution debits and *reserves* funds and releases them only on final confirmation (§11.3), obligations become irrevocable at `ACSP` and settle at `ACSC` (§5), and two of the four failure modes leave a payment irrevocably committed while the sender does not yet know it (§11.4).

## Decision

**The ledger is a dedicated bounded service** (`settlement/`) with its own storage, reachable only through a narrow `Ledger` interface ([`../ledger.md`](../ledger.md)). It is not embedded in the payment router, which stays stateless, and not in the participant gateway, which owns no balances.

**The append-only event log is the system of record; the ledger is a projection of it** with invariants enforced by the storage engine. Any ledger state must be reconstructible from events.

**The domain model uses double-entry primitives shaped after TigerBeetle**: accounts, transfers, two-phase pending transfers with timeouts, linked atomic chains, and a `code` field carrying the catálogo's reason codes. This holds regardless of backend, because it makes mutation inexpressible and forces balances to be derived rather than stored.

**Two backends, both first-class.** PostgreSQL is the default and shipped implementation, per ADR-0007. **TigerBeetle is implemented as a second backend early, not deferred** — not for throughput, which we do not need, but because two independent implementations driven by the same property-based suite are the strongest correctness evidence available to us. Differential testing between them runs in CI; a divergence fails the build.

**No hosted or SaaS ledger in the core, ever.**

## Alternatives considered

- **Hosted ledger services (Fragment, Modern Treasury, and peers)** — well-built, and their reconciliation models and API ergonomics are worth studying. Rejected outright for the core: a national payment system cannot hold its ledger of record inside a foreign commercial SaaS. Data residency, sovereignty, cost at national volume, vendor risk, and a direct contradiction of this project's premise.
- **TigerBeetle only** — tempting given the fit, but Postgres is required anyway for non-ledger state, and asking Ecuadorian operators to run an unfamiliar clustered database as their *only* option contradicts the operability constraint in [`../technical-requirements.md`](../technical-requirements.md).
- **Postgres only** — simplest, and sufficient at our volumes. Rejected because it forfeits differential testing and the in-database invariants that make "you cannot create an unbalanced transfer" a guarantee rather than a hope.
- **Ledger embedded in the payment router** — fewer moving parts, but it makes the router stateful and couples routing scale to ledger consistency.

## Consequences

**Ledger availability equals inter-red payment availability.** The reserve happens synchronously during authorization (see ADR-0009), so the ledger is on the critical path. This must be designed for, not discovered: it means multi-replica deployment for the Scheme Core and Network profiles, and it puts a posting inside the latency budget in [`../technical-requirements.md`](../technical-requirements.md).

**The chart of accounts becomes a designed, versioned artifact** rather than an implementation detail, because it determines what the system can express and changing it later is a migration on a live ledger.

Easier: auditability by construction, provable invariants, a credible answer to "how do we know it is correct". Harder: two backends to maintain and keep behaviourally identical. Committed: no mutation of postings ever; balances derived, never stored as truth; every entry traceable to the event and signed message that caused it.
