# Deliverables: what we ship, who uses it, how they touch it

> **Read before adding a component, naming a directory, or arguing about repository structure.** Structure is a physical expression of this list; when they disagree, this document wins and the tree moves.
>
> **Owner:** Sebastián · **Last reviewed:** 2026-08-11 · **Update trigger:** a new deliverable, or a deliverable changing its audience or interaction mode.

Complements [`stakeholders.md`](stakeholders.md), which says *who* we build for, and [`requirements.md`](requirements.md), which says *what they need*. This one says what physically ships and how a person or a system comes into contact with it.

## The rule

**Every deliverable has exactly one primary audience and exactly one primary interaction mode.** If we cannot name both, it is not a deliverable — it is an implementation detail that belongs inside something else.

This is the discipline that keeps the project from becoming a pile of Go packages with a website attached.

## Interaction modes

How something is touched matters more than what it is, because it determines what we owe: an API we owe stability, a binary we owe operability, a library we owe semantic versioning.

| Mode | Meaning | What we owe |
|---|---|---|
| **Deploy** | They run our binary on their infrastructure | Operability by existing staff: config, logs, health, upgrade path, runbooks |
| **Hosted** | Someone else runs our binary for them | An SLA and the credible ability to leave |
| **Call** | Their system talks to ours over the network | A versioned API contract and backward compatibility |
| **Generate** | An SDK produced from our spec, in their language | A correct, published spec — the SDK is downstream of it |
| **Import** | Our Go code compiles into their binary | Semantic versioning and a stable public API |
| **Operate** | A person uses a UI to do their job | That the job is completable without reading documentation |
| **Run** | A one-shot command producing a report | A trustworthy answer and a legible failure |
| **Read** | A human reads it | Accuracy, and Spanish that does not read as translation |

Note what is scarce: **exactly two deliverables are imported.** Almost nobody integrates with us by writing Go.

## The catalogue

| Deliverable | What it is | Primary audience | Mode |
|---|---|---|---|
| **`sip`** | The central system: registry, central key system, resolution, payment router, risk funding, clearing and the ledger | The BCE (Scheme Core profile) | Deploy |
| **`red`** | Network profile: intra-network routing plus a distributed directory node | Redes de pagos — BANRED, COONECTA, ETIKO's, DEUNA | Deploy |
| **`gateway`** | What an institution puts between its core banking and a red: simple API in, signed ISO 20022 over mTLS out | Banks, cooperativas, mutualistas, SEDPEs | Deploy / Hosted |
| **North API** | The gateway's REST + webhook surface, versioned | Core-banking systems and their vendors | Call |
| **SDKs** | Generated clients in Go, TypeScript and Java | Integration developers | Generate |
| **Consoles** | Operations UI, one application with role-scoped views per profile | Jefe de Operaciones, network operators, BCE administrators | Operate |
| **`spec`** | OpenAPI for the North API, ISO 20022 profile definitions, QR payload spec | Everyone, including our own SDK generation | Read / Generate |
| **`tck`** | Conformance runner producing a pass/fail report against the standard | Jefe de Sistemas, before BCE qualification | Run |
| **`sandbox`** | The whole simulated network in one command | Developers, and anyone who needs to see it work | Run |
| **`caudalctl`** | CLI: validate a message, inspect a payment, administer a deployment | Operators and developers | Run |
| **`pkg/iso20022`** | Go message library for the BCE profile | The rare institution or red building its own gateway | Import |
| **`pkg/qr`** | EMV QRCPS encoder and decoder carrying llaves | Same, plus mobile app teams | Import |
| **Documentation** | The catálogo decode, architecture, RFCs, runbooks, tutorials | Institutions, regulators, contributors | Read |
| **The simulation** | Published models: liquidity ceilings, netting exposure, fee scenarios | The BCE, multilaterals, the press | Read |

## What each audience actually touches

The useful way to read the catalogue is per person, because most of the people who decide whether we succeed never see code.

**A small cooperativa.** Its *Jefe de Sistemas* deploys `gateway` — or, more likely, signs up with a host and deploys nothing. They run `tck` once to prove conformance before BCE qualification, and read Spanish documentation. Their core banking calls the **North API**, probably through a **generated Java SDK** written by their vendor rather than by them. Their *Jefe de Operaciones* lives in the **participant console** every single day: reconciling the `por conciliar` queue before the 18:00 close, answering "where is my money", watching certificate expiry and headroom. Their *Oficial de Riesgos* reads the security model and the audit trail. **Nobody at this institution ever imports a Go package or reads an XSD** — [`requirements.md`](requirements.md) makes that an explicit promise, and it is the promise the whole participant product rests on.

**A large bank.** Same, except it may build its own gateway, in which case it imports `pkg/iso20022` and `pkg/qr` and reads `spec` closely. It is also the one institution likely to use us purely as a second quote that resets a vendor's pricing, which is a legitimate outcome.

**A red de pagos.** Deploys `red`, operates the network console, and cares more about `spec` and the conformance kit than about any of our source code, because its job is to police its members' conformance.

**The BCE.** Deploys `sip`, operates the administration console, and receives our documentation, RFCs and simulations whether or not it ever runs a line of our code. Note that the reading relationship is real and valuable on its own: [RFC 0001](../rfcs/0001-liquidacion-continua.md) and the observations register are deliverables to this audience even if the software is never adopted.

**A core-banking vendor.** Never deploys anything. Calls the North API for dozens of client institutions, and is therefore the highest-leverage integrator in the project — one vendor integration reaches many cooperativas.

**A contributor.** Clones, runs `sandbox`, and reads `docs`. Their fifteen-minute experience is a product requirement, not a nicety.

## Three consoles, one application

The consoles are the most under-specified deliverable and the one most likely to decide adoption, because the *Jefe de Operaciones* is the person nobody builds for. There are three surfaces — participant, network, scheme administration — and they are **one application with role-scoped views**, mirroring the one-codebase-four-profiles principle in [`../architecture/blueprint.md`](../architecture/blueprint.md). Three separate UIs would triple the surface with no benefit; the underlying data differs by profile, but reconciling a payment looks the same everywhere.

## Not deliverables

Stated so they stop being proposed: no custody, wallet, or ledger of customer accounts; no KYC/AML engine, only hooks; no mobile app for end users; no blockchain; no fork of Mojaloop; and **no requirement that an institution write Go, construct XML, or manage a signing key by hand**.

## What this implies for the repository

Four consequences follow directly, and they settle the structure argument.

**`spec/` is a top-level deliverable, not a byproduct.** SDKs generate from it, the conformance kit tests against it, and it is what institutions read. It is the closest thing this project has to a product in the Pix sense, where the specification *is* the artifact.

**Consoles need a home.** One application, three role-scoped views, and it is not the marketing site. `site/` stays what it is.

**Only `pkg/` is imported, and it is small.** Everything else in the engine is deployed, which is exactly why the core lives under `internal/` — we owe operability there, not API stability. Packages get promoted to `pkg/` when a real consumer appears, never speculatively.

**Binaries are the unit institutions think in.** `cmd/` holds one directory per deployable — `sip`, `red`, `gateway`, `caudalctl` — and those names should match what people say out loud when they describe what they are running.
