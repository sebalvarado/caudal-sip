# Product Requirements

What each component must do for the people in [`stakeholders.md`](stakeholders.md), stated as jobs and acceptance criteria rather than features. Architecture is in [`../architecture/blueprint.md`](../architecture/blueprint.md); non-functional requirements are in [`../architecture/technical-requirements.md`](../architecture/technical-requirements.md); message-level and scheme behavior is in [`../architecture/scheme-design.md`](../architecture/scheme-design.md).

Requirements marked **[assumption]** depend on the BCE catálogo técnico we do not have yet. Requirements marked **[proposal]** are ours, not the regulation's, and need an [RFC](../rfcs/README.md) before they become scheme behavior.

## The jobs

| # | Who | Job | Today's alternative |
|---|---|---|---|
| J1 | Core-banking developer | Make my institution send and receive real-time payments without learning ISO 20022 | A vendor contract, or months reading XSDs |
| J2 | Head of IT at a cooperativa | Pass BCE technical qualification by the deadline with the team I already have | Hope, or a quote they cannot pay |
| J3 | Payments operations staff | Run daily operations: reconcile, investigate a failed payment, answer "where is my money" | Spreadsheets and phone calls |
| J4 | Red de pagos operator | Route, validate, and clear intra-network payments, and onboard members without bespoke work per member | Custom integration per institution |
| J5 | Regulator / network architect | See how a design decision behaves before mandating it | Consultant slide decks |
| J6 | Contributor | Understand the domain and ship a correct change | Nothing — no public Spanish-language material exists |

## Scope by phase

Phases follow [`../plan/action-plan.md`](../plan/action-plan.md). Nothing here authorizes building ahead of a named user.

| Phase | Ships | Serves | Done when |
|---|---|---|---|
| 1 | `iso20022/` message library + CLI, `spec/`, `sandbox/` | J1, J5, J6 | An outside developer clones and completes a simulated payment in under 15 minutes |
| 2 | `gateway/` (self-hosted), participant console, `qr/`, `tck/` | J1, J2, J3 | A design-partner institution runs it in their test environment and passes the conformance suite |
| 3 | `switch/`, `llaves/`, `settlement/`, network console, multi-tenant gateway | J4, J5 | A full simulated network runs under published load tests; a federation onboards multiple institutions on one deployment |

## Functional requirements

### Payment initiation and receipt (`gateway`, J1)

- Initiate a payment by **llave** or by explicit credencial de pago (document, account, account type, BIC), in a single JSON call, with an idempotency key required on every mutation.
- Resolve a llave and return the masked beneficiary name for confirmation before the payer commits — the anti-fraud step Pix and Bre-B both proved is essential.
- Deliver every state change to the institution by webhook with at-least-once delivery, signed payloads, retry with backoff, and a replay endpoint for missed events.
- Receive inbound payments: validate that the credencial matches an actual client account and reject on mismatch per Art. 10 of BCE-GG-007-2026, then respond within the response budget.
- Never require the institution to construct, parse, or sign XML. The gateway owns translation, signing, and mTLS.
- Support the regulated transfer ceiling (USD 15,000) and per-institution limits configurable without redeployment.
- Ship generated SDKs for Go, TypeScript, and Java **[proposal]** — Java because the core-banking world runs on it.

### Llaves (`llaves`, `gateway`)

- Register, list, re-point, block, and remove keys of each supported type, with system-wide uniqueness of one llave to exactly one credencial.
- Enforce a cap on keys per account **[proposal]**, and rate-limit plus audit every resolution to prevent directory enumeration — a real, expensive Pix lesson.
- Support portability between institutions with a confirmation flow through the losing institution **[proposal]**, so a phone number cannot be hijacked.
- Expose the full lifecycle to operations staff, not just to the API: who registered a key, when, and what it points at now.

### QR (`qr`)

- Encode and decode EMV QRCPS payloads carrying a llave, static (merchant identity) first and dynamic (amount plus reference) immediately after **[assumption:** exact payload profile pending the BCE QR standard**]**.
- Validate third-party QRs strictly and fail closed on anything malformed — QR is the most likely fraud surface for end users.

### Routing, clearing, settlement (`switch`, `settlement`, J4, J5)

- Route by BIC, validate schema and signature, enforce per-hop timeouts, and remain stateless so it scales horizontally.
- Emit `pacs.002` for acceptance, rejection, and the irrevocability declaration, with the correct emitter per topology: the red for intra-network, the SIP for inter-network (Art. 12, BCE-GG-007-2026).
- Maintain a double-entry ledger with no overdraft, where every payment produces balanced postings and no balance can go negative.
- Implement gross real-time settlement as the reference model **and** simulate deferred net cycles, so the sandbox can quantify the risk difference between them. That comparison is a deliverable aimed at the interoperability forums, not an academic exercise.

### Conformance (`tck`, J2)

- Run a scenario suite against any implementation reachable over the South API and produce a signed, shareable pass/fail report an institution can take into BCE qualification.
- Cover the unhappy paths that break real integrations: timeouts, duplicate messages, malformed XML, signature failures, credencial mismatches, and partial network failure.
- Be usable by someone who is not a Go developer: one command, readable output, exit codes for CI.

## Surfaces: do we need internal UIs?

Yes — and treating this as an afterthought is the most common way infrastructure projects fail to get adopted. An API-only product implicitly requires every institution to build its own operations tooling, which is exactly what a cooperativa with four people in IT cannot do. There are four distinct surfaces, and they are not one application.

| Surface | Who uses it | Ships with | Why it exists |
|---|---|---|---|
| **Developer console** | Engineers, demos, contributors | `sandbox/` | Watch a payment traverse the network, inspect every message, replay and mutate scenarios. Doubles as the single most effective marketing artifact we have |
| **Participant operations console** | Operations staff at an institution (J3) | `gateway/` | A **`por conciliar` queue with a same-day 18:00 deadline** (Catálogo Operacional §13.2) and a **liquidity-threshold indicator** showing distance to the risk ceiling that would block outgoing payments (§10) — both are regulatory obligations, not conveniences. Plus failed-payment reason codes in plain Spanish, llave support lookups, certificate and key expiry warnings, connectivity status. **This is the difference between software an institution can adopt and software it cannot** |
| **Network operations console** | Red de pagos or scheme operator (J4) | `switch/` + `settlement/` | Member onboarding, positions and cycle status, suspend or resume a participant during an incident, directory abuse monitoring, message-level forensics |
| **Hosted control plane** | Whoever operates the multi-tenant gateway | Hosted profile | Tenant lifecycle, per-tenant keys and certificates, SLA and usage reporting. The only surface tied to the commercial layer, and it stays thin |

Design constraints that apply to all of them: Spanish first with the vocabulary in [`../glossary.md`](../glossary.md); read-only by default with destructive actions behind explicit confirmation and an audit entry; every screen answerable from the audit log rather than from live production data where possible; and no console is ever required to operate the system — everything it does is also an API call, so automation stays possible.

## Product-level non-goals

Stated so they stop coming up. These are refusals, not "later".

- No end-user wallet, app, or custody. No KYC/AML engine — we expose hooks and leave the liability where it legally belongs.
- No accounting, core banking, or ledger-of-record for an institution's customer accounts. The gateway reflects; it does not own the balance.
- No fraud scoring engine in v1. We ship the data and the hooks; scoring is a specialization with its own vendors, and pretending otherwise oversells.
- No dispute resolution or chargeback workflow until the BCE catalog defines return semantics. Modeling it early and being wrong is worse than waiting.
- No multi-country abstraction. Profile-driven design means another country is *possible*; building for it now means serving no one well.
- No paid features in the core, and no compliance capability behind a contract.

## Open questions this document does not answer

Tracked here so they become RFCs rather than assumptions that quietly harden. These are the same items flagged in [`../architecture/scheme-design.md`](../architecture/scheme-design.md) §5, viewed as product decisions: return and dispute flows, request-to-pay, bulk disbursement, the fee policy we advocate for, key portability mechanics, and the uniform UX rules (maximum seconds, confirmation-of-payee display, common branding) that Ecuador's texts have not addressed.

## Changelog

- 2026-08-05 — Initial version.
