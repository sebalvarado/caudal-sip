# Technical Requirements

The non-functional bar every component must clear. Functional scope is in [`../product/requirements.md`](../product/requirements.md); this document is about latency, availability, security, and the operational reality of the institutions that will run this.

Numbers marked **[assumption]** are our engineering budget until the BCE catálogo técnico y operacional specifies otherwise. We publish them anyway, because a budget you can measure against is more useful than silence — and if the catálogo contradicts us, the catálogo wins.

## The constraint that shapes everything

**A cooperativa with four people in IT must be able to run this.** Not a platform team, not a Kubernetes operator, not a 24/7 SRE rotation. This single constraint is why the stack is Go single binaries, why Postgres is the only required dependency, why the participant console exists, and why we will not adopt a technology that requires specialist operations knowledge to keep alive. Mojaloop is excellent software whose practical adoption is limited by exactly this. Every dependency proposal gets asked: *can the institution's existing team operate this at 2am?*

## Latency

Two ceilings are set by the Catálogo Operacional §5 and are not ours to choose ([decoded here](../research/catalogo-operacional.md)): after llave resolution the customer has **60 seconds** to confirm, and elapsed time from initiation to confirmation may not exceed **20 seconds** or the payment is rejected with `DT01`. Every stage additionally carries its own parameterized timeout, whose values the catálogo does not publish.

Our engineering budget works backwards from the 20-second ceiling, targeting **5 seconds p99** so that the regulatory limit is never the binding constraint:

| Hop | Budget (p99) | Notes |
|---|---|---|
| Gateway ingress: validate, sign, emit | ≤ 50 ms | Pure computation; no external calls on the happy path |
| Llave resolution, full round trip | ≤ 1500 ms | Traverses IR → RP → SIP → RP → IR and back; the beneficiary institution must answer within it |
| Switch or SIP: schema, signature, routing | ≤ 100 ms per hop | Stateless, no database write on the hot path |
| Beneficiary core banking: validate, credit, respond | ≤ 2000 ms | The slowest hop, and not ours. The gateway imposes the timeout rather than inheriting it |
| Settlement posting | ≤ 100 ms | Single-writer double-entry; the volumes make this comfortable |
| **End to end, confirmation to irrevocability** | **≤ 5 s**, hard limit 20 s | We publish and measure the 5; the 20 is the regulatory cliff |

Design consequences: no synchronous external calls inside a hop budget; every timeout is explicit, configurable per counterparty, and shorter than the budget of the hop above it; funds stay **reserved** on the ordering side for the whole flow, which is why the ceiling exists; and timeouts resolve into reconciliation rather than into a rejection ([`data-model.md`](data-model.md) §3). Note that in two of the four failure modes the payment is already irrevocable while the ordering side is still waiting, so a timeout must never release the reservation on its own.

## Throughput and scale

Ecuador's realistic volume is modest and worth stating plainly, because "national payment system" invites over-engineering. Bre-B, in a country roughly three times larger, peaks around 5 million transactions per day — an average near 60 TPS. Our design targets **500 TPS sustained and 2,000 TPS peak per switch node [assumption]**, which leaves substantial headroom and is achievable on ordinary hardware.

Scaling characteristics per component: the switch is stateless and scales horizontally behind a load balancer; the directory is read-heavy with strict uniqueness on write, so reads cache and writes serialize per key; settlement is the only strongly consistent core and stays single-writer, which at these volumes is a boring Postgres problem; the gateway scales per tenant and is embarrassingly parallel across institutions.

**Publishing the load-test numbers is itself a deliverable.** The argument that a national instant-payments rail requires a decade and a nine-figure budget does not survive a reproducible benchmark on modest hardware.

## Availability

Payments do not have a maintenance window. The whole promise is Sunday at midnight.

- Target **99.95% or better** for participant-facing components, measured monthly, with no scheduled downtime — upgrades are rolling, and schema migrations are backward compatible for at least one release.
- **Degrade, don't fail.** If the directory is unavailable, payments to an explicit credencial must still work. If settlement is behind, customer-facing finality is unaffected because the two are separate events by design.
- Every component exposes liveness and readiness endpoints, and a readiness check that fails for a dependency the component can operate without is a bug.
- Recovery objectives: **RPO zero** for anything financial — no acknowledged payment or posting may be lost, which means synchronous durability before acknowledgement — and an **RTO under 15 minutes [assumption]** for participant components.

## Security

Payments software is attacked. The [security policy](../../SECURITY.md) covers reporting; these are the build requirements.

**Transport and messages.** mTLS everywhere, with certificate pinning per counterparty where the scheme allows. Detached signatures on messages (JWS-style, as Pix does) so authenticity survives transport termination. Key rotation designed in from the first release, with expiry warnings surfaced in the participant console long before anything breaks — an expired certificate at 3am is a far more likely outage than a clever attack.

**Keys.** Private keys live in a KMS or HSM behind a pluggable interface, never in the database, never in configuration files, never in the repository. The interface ships with a file-based development implementation that refuses to start when the environment is production.

**Boundaries.** Every input from the network is untrusted, including from other participants: strict schema validation, explicit limits on message size and XML entity expansion, and a parser that fails closed. Multi-tenant isolation is enforced at the database with row-level security rather than only by application filters.

**Supply chain.** A deliberately small dependency tree — every addition is a decision, not a convenience. Signed releases with an SBOM, reproducible builds, pinned versions, and automated vulnerability scanning that blocks the release rather than filing a ticket. An institution's risk officer will read this list, and it is easier to defend a short one.

**Auditability.** Append-only audit written in the same transaction as the change, hash-chained **[proposal]**, and sufficient to reconstruct any payment years later without access to production. This maps directly to the security and continuity obligations of Art. 10 of BCE-GG-008-2025.

## Correctness and testing

Standard test discipline is not enough for a ledger. Required, per component:

- **Golden-file tests** for every message type, so a byte-level change to the wire format is impossible to make accidentally.
- **Property-based tests** for ledger invariants: no operation sequence produces an unbalanced entry, a negative balance, or a lost posting.
- **Fuzzing** of every parser exposed to the network — XML, QR payloads, JSON — as a permanent CI job, not a one-time exercise.
- **Failure injection** in the sandbox: dropped responses, duplicated messages, slow counterparties, partial network partitions. The unhappy paths are where real integrations break, and where the certification kit earns its value.
- **Conformance runs** against the standard, producing the same report an institution takes into BCE qualification. We run it against ourselves on every release.

## Observability

Institutions will be asked by a regulator what happened to a specific payment. That question must be answerable in seconds.

Structured logs with a correlation id that follows a payment across every component and appears in the console; metrics for the latency budget above, per hop, so a breach is visible before a customer complains; distributed tracing across the payment path; and alerting defined for the operator, not for us — certificate expiry, counterparty timeouts trending up, unreconciled timeouts aging, settlement position approaching a limit.

Hard rule: **no personal data in telemetry.** Correlation ids and hashes, never account or document numbers.

## Deployment and operations

- Single static Go binary per component, plus a container image. No runtime dependency beyond Postgres for the participant profile.
- Configuration by environment variables and one file, validated at startup with actionable errors. A misconfigured payment system must refuse to start rather than run in an ambiguous state.
- Deployable on a single virtual machine for a small institution, and horizontally for a network operator, from the same artifact.
- Every release ships with an upgrade note and a rollback path, and migrations are backward compatible for one release so rollback is real rather than theoretical.
- Runbooks live in the repository next to the code, in Spanish for the operator-facing ones.

## Compliance mapping

Where these requirements come from, so a risk officer can follow the thread: message formats and validations from BCE-GG-007-2026 and its catálogo; beneficiary credencial validation from Art. 10; irrevocability emission from Art. 12; security, continuity, and auditability from Art. 10 of BCE-GG-008-2025; clearing arrangements from Art. 9. Full citations are in [`../research/landscape.md`](../research/landscape.md). Requirements we cannot yet cite are marked **[assumption]** and revisited when the catálogo arrives.
