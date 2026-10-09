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
| **`red`** | Network profile: intra-network routing plus a distributed directory node. A *red de pagos* is a **licensed institution**, not a network topology — see below | Redes de pagos — BANRED, COONECTA, ETIKO's, DEUNA | Deploy |
| **`gateway`** | What an institution puts between its core banking and a red: simple API in, signed ISO 20022 over mTLS out | Banks, cooperativas, mutualistas, SEDPEs | Deploy / Hosted |
| **North API** | The gateway's REST + webhook surface, versioned | Core-banking systems and their vendors | Call |
| **SDKs** | Generated clients in Go, TypeScript and Java | Integration developers | Generate |
| **Consoles** | Operations UI, one application with role-scoped views per profile | Jefe de Operaciones, network operators, BCE administrators | Operate |
| **`spec`** | OpenAPI for the North API, ISO 20022 profile definitions, QR payload spec | Everyone, including our own SDK generation | Read / Generate |
| **`tck`** | Conformance runner producing a pass/fail report against the standard | Jefe de Sistemas, before BCE qualification | Run |
| **`sandbox`** | The whole simulated network in one command | Developers, and anyone who needs to see it work | Run |
| **`caudalctl`** | CLI: validate a message, inspect a payment, run migrations, rotate keys | Engineers, during installation and incidents — **not** daily operations | Run |
| **`pkg/iso20022`** | Go message library for the BCE profile | The rare institution or red building its own gateway | Import |
| **`pkg/qr`** | EMV QRCPS encoder and decoder carrying llaves | Same, plus mobile app teams | Import |
| **Documentation** | The catálogo decode, architecture, RFCs, runbooks, tutorials | Institutions, regulators, contributors | Read |
| **The simulation** | Published models: liquidity ceilings, netting exposure, fee scenarios | The BCE, multilaterals, the press | Read |

## What each thing is built as, and where its data lives

The table above says who touches what. This one says what a maintainer is actually building and what has to be backed up.

| Deliverable | Built as | Its data |
|---|---|---|
| `sip` | Go binary, long-running service | **Postgres**: ledger, registry, central key system, event log. TigerBeetle later as an alternative ledger backend |
| `red` | Go binary, same codebase, network profile | **Postgres**: distributed directory node, routing and empadronamiento state, event log |
| `gateway` | Go binary, single- or multi-tenant | **Postgres**: payment records, llave mappings, credential snapshots, outbox |
| North API | HTTP + JSON, defined by OpenAPI; webhooks outbound | None — the gateway's storage |
| SDKs | Generated from the OpenAPI spec, published to Go modules, npm and Maven Central | None |
| Consoles | Web application; **stack not yet decided — see below** | None of its own; reads the backend it is attached to |
| `spec` | Not code: OpenAPI YAML, ISO 20022 profile definitions and XSDs, QR payload spec. Versioned releases | Not applicable |
| `tck` | Go binary plus a corpus of declarative scenarios and golden messages | Emits report files; no persistent store |
| `sandbox` | `docker compose` stack with seed data | Ephemeral Postgres per container, thrown away |
| `caudalctl` | Go CLI, single binary | None; talks to a service or a file |
| `pkg/iso20022`, `pkg/qr` | Go libraries, semantically versioned | None |
| Documentation | Markdown in `docs/`, rendered by `site/` | Git |
| The simulation | Offline analysis: Go or notebooks over reconstructed public data | Input files and published results |

Every stateful component is **Postgres**, per [ADR-0007](../architecture/adr/0007-postgres-first.md). One database technology across the whole system is a deliberate operability constraint: a cooperativa's IT department can back up and audit Postgres with skills it already has.

### `tck` — the conformance kit

Not a test framework for us; a **conformance driver aimed at somebody else's implementation**. It impersonates the counterparty — a red, or the SIP — and drives the institution's gateway through a scripted corpus: the happy path, every rejection code in Tabla 12, each of the four exceptions in §11.4, llave registration and cancellation, timeout behaviour. It then produces a pass/fail report suitable for handing to the BCE. Mojaloop's Testing Toolkit is the closest prior art. An institution runs it before qualification and after every change. The ambition is that it becomes the de facto pre-certification tool for the whole country, including for institutions that never deploy our gateway.

### `sandbox` — the network in a box

A `docker compose` stack that starts the entire simulated national network locally: a fake SIP, two redes, several mock banks, seeded llaves and test identities. One command, no real infrastructure, no BCE involvement. It serves the fifteen-minute clone-to-payment experience, the demo video that makes "building this is hard" stop being credible, and the substrate the `tck` runs against.

### `caudalctl` — the CLI

**Not the day-to-day operations tool.** Daily work happens in the console; the *Jefe de Operaciones* never opens a terminal. `caudalctl` is for engineers: validate a message against the profile, inspect a payment by end-to-end ID during an incident, run migrations, generate configuration, rotate keys. Installation and incident response, not routine operation.

### `red` — and what a "red de pagos" actually is

The Spanish word invites a misreading. A **red de pagos is a licensed institution, not a network deployment and not a VPN** — an authorized *administradora* that routes payments among its member integrantes, in the sense that "the Visa network" names a company's routing role rather than any cabling. BANRED, COONECTA, ETIKO's and DEUNA are the four named in Anexo 2 of the catálogo.

Its regulated responsibilities are specific: incorporate integrantes into real-time operation, administer their distributed directory, and — the part that matters most for us — **technically qualify and validate its own members** (BCE-GG-008-2025, Art. 4). A red is the certifying authority for the institutions beneath it, which makes it a natural adopter of the conformance kit and one of the highest-leverage relationships in the project.

`red` is simply the binary such an institution deploys. Its architecture, domain model and requirements — and an honest account of what deploying it does *not* give you — are in [`../architecture/red.md`](../architecture/red.md).

**On connectivity**, the published norm mandates no transport protocol at all. Art. 5.1 requires connection "a los enlaces de telecomunicación establecidos por el BCE" and Disposición General TERCERA assigns a separate, still-unpublished document containing the connection standards. Our defaults and the resulting design constraint — transport must stay pluggable — are in [`../architecture/technical-requirements.md`](../architecture/technical-requirements.md).

### Consoles — the undecided part

One web application with role-scoped views, but **the stack is an open decision that matters more than it looks**. Embedding the UI in the Go binary — server-rendered, no separate deployment — means an institution installs one file and is done. A separate single-page application is more pleasant to build and gives us a second thing for a cooperativa without an infrastructure team to deploy, monitor and upgrade. Given that [`stakeholders.md`](stakeholders.md) identifies operability by existing staff as the adoption constraint, the single-binary option is strongly indicated, but this needs an ADR before anyone writes console code.

## Data pipelines: what exists and what deliberately does not

**There is no ETL in the payment path and no data platform.** Payments are synchronous request and response over an append-only event log. No Kafka, no warehouse, no stream processing. At Ecuador's volumes the transactional database also answers the reporting and dashboard queries the catálogo requires, and adding a second data tier would buy nothing while doubling what an operator has to run.

What does exist is a small set of **scheduled jobs**, each mandated by the catálogo:

| Job | Cadence | Source |
|---|---|---|
| Poll the integrante's BCE current-account balance | Every 30 minutes, parameterized | Sistema SLA (§10, rule 7) |
| Recompute the nivel de riesgo aceptable | Weekly, Thursday to Wednesday | Sistema SBA encaje cycle (§10) |
| Settle the multilateral malla | Several times daily, plus manual | Internal (§9, rules 2–3) |
| Close the `por conciliar` queue | Daily by 18:00 | Internal (§13.2) |
| Synchronize the central key system with distributed directories | Continuous, reconciled periodically | Internal (§3, §12) |
| Operational reports | On demand and scheduled | Internal (§2.1.17) |

Two of these — the SLA balance poll and the SBA encaje feed — are **integrations with existing BCE systems we do not control**, which makes them the most likely source of operational surprises and the first thing to stub in the sandbox.

The simulation deliverable is genuinely batch data work, but it is offline analysis over public data, not production infrastructure.

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

**Consoles need a home**, and it is not the marketing site — `site/` stays what it is. Where exactly depends on the stack decision above: server-rendered templates live beside the Go code they serve, while a single-page application needs its own directory and build. That ADR should come before the restructure touches this.

**Only `pkg/` is imported, and it is small.** Everything else in the engine is deployed, which is exactly why the core lives under `internal/` — we owe operability there, not API stability. Packages get promoted to `pkg/` when a real consumer appears, never speculatively.

**Binaries are the unit institutions think in.** `cmd/` holds one directory per deployable — `sip`, `red`, `gateway`, `caudalctl` — and those names should match what people say out loud when they describe what they are running.
