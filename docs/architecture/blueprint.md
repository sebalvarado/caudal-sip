# Project Blueprint: RPI EC — An Open-Source Real-Time Payments Stack for Ecuador

## Positioning (the one-liner)

**"RPI EC: the open-source implementation of Ecuador's real-time payments standard (RPI/SIP)."**

Naming decision: we use the government's own nomenclature — **RPI EC** — so every conversation starts from the mandate itself. Since "RPI" is the BCE's infrastructure name, every public artifact carries the disclaimer: *independent open-source project, not affiliated with the BCE, implementing the public standard.* (See footer of `MANIFIESTO.md`.)

Not a competitor to the BCE's SIP/RPI — the missing developer layer around it: the message libraries, participant gateway, sandbox network, and certification tooling that let any bank, cooperativa, mutualista, SEDPE, or fintech implement the standard in weeks instead of years. Think **Medusa for commerce → this for payment rails**, or **moov-io, but for Ecuador's scheme**, with a Mojaloop-style full-network sandbox.

Two audiences, one codebase:

1. **Participants (the wedge)**: ~400 institutions must speak ISO 20022 pacs.008/pacs.002 + llaves + QR by the BCE deadlines. Most cooperativas cannot build this. We give them `docker compose up` compliance.
2. **The scheme operator (the endgame)**: a complete reference network — switch, directory, settlement simulator — that demonstrates to the BCE and the industry that the hard part is not the software. If the BCE ever wants an open sandbox or a reference stack (as BCB did publishing Pix specs on GitHub), it already exists.

## Operator model: one stack, four deployment profiles

Ecuador's regulation defines four operator roles. We build one codebase whose components compose into a profile for each role — that's what makes the project scalable instead of a pile of unrelated tools:

| Profile | Who runs it | Components | How they get it |
|---|---|---|---|
| **Scheme Core** | The BCE (SIP + RPI) — our endgame proposal | `switch` (integrator) + `llaves` (central resolution) + `settlement` | Open source. Offered as the reference/sandbox first; production adoption is the long-term institutional play, like BCB adopting GitHub for Pix specs |
| **Network** | Redes de pagos (BANRED, future private networks) | `switch` (intra-network routing) + `llaves` (federated directory node) | Open source; support contracts possible |
| **Participant (self-hosted)** | Banks and large cooperativas with IT teams | `gateway` (single-tenant) + `qr` + `tck` | Open source, free forever. The Medusa-style core product |
| **Participant (hosted)** | Small cooperativas, mutualistas, fintechs without infra teams | `gateway` (multi-tenant) operated by a host | **The PaaS/SaaS layer** — see below |

### The hosted connectivity play (PaaS/SaaS)

Yes — and it's the natural open-core business model (Medusa Cloud / Twenty Cloud analog): the software is Apache-2.0 and self-hostable forever; what's sold is operation — hosting, mTLS certificate and key management, HSM custody, monitoring, upgrades, compliance reporting, BCE qualification support, SLAs. Three tiers of who can be the host:

1. **Us (or a company formed around the project)**: "Conectividad RPI como servicio" — a cooperativa signs up, gets a tenant on a multi-tenant gateway, points its core banking at the North API, and is scheme-compliant in days.
2. **Distribution partners as hosts**: cajas centrales, cooperativa federations, and core-banking vendors run their own multi-tenant gateway instances for their members/clients. They keep the customer relationship; the project gets adoption. This matters because trust in the cooperativa sector is relational — they'll buy from their federation before they buy from a startup.
3. **Anyone else**: no lock-in; a hosted tenant can export config and move to self-hosted. That credibility is *why* institutions will accept tier 1 and 2.

**Regulatory note**: the norm already contemplates this role — integrantes may connect "con su infraestructura... de manera directa o la provista por un tercero, de forma indirecta" (BCE-GG-007-2026, def. 6), and third parties authorized as partícipes del Sistema Auxiliar de Pagos in "servicio de procesamiento de medios de pago electrónicos" are named as integrantes. A hosted-connectivity operator will likely need that BCE authorization — this is a feature, not a bug: it's a moat once obtained, and the requirements (security, continuity, auditability, Art. 10 of BCE-GG-008-2025) are exactly what the software already implements. Budget for this qualification in the company plan.

### Who talks to whom (full topology with deployment boundaries)

```
╔═ BCE (Scheme Core profile) ═══════════════════════════════════════╗
║   switch/integrator (SIP)   llaves central   settlement (RTGS/CCE) ║
╚══════════▲══════════════════════▲═══════════════════▲═════════════╝
           │ ISO 20022 mTLS       │ acmt               │
╔═ Red de pagos A (Network profile) ═╗  ╔═ Red de pagos B ═══════════╗
║  switch (intra-red)                ║  ║  (same stack, competing    ║
║  llaves federated node             ║  ║   operator)                ║
╚═══▲══════════════▲═════════════════╝  ╚═════▲══════════════════════╝
    │              │                          │
╔═ Banco X ═════╗ ╔═ Hosted gateway (PaaS) ═╗ ╔═ Caja central Y ═════╗
║ gateway       ║ ║ multi-tenant gateway    ║ ║ multi-tenant gateway ║
║ (self-hosted) ║ ║ coop A │ coop B │ coop C║ ║ for member coops     ║
╚═══▲═══════════╝ ╚═══▲═════════════════════╝ ╚═════▲════════════════╝
    │ North API       │ North API                   │
 core banking      core banking (per coop)       core banking
```

Scalability characteristics by component: the `switch` is stateless (UPI's lesson) — horizontal scaling behind a load balancer, no sticky state; `llaves` is read-heavy with strict uniqueness on write — cacheable reads, serialized writes per key; `settlement` is the only strongly-consistent core — single-writer double-entry ledger, which at Ecuador's realistic volumes (Bre-B, a country 3x larger, peaks ~5M tx/day ≈ 60 TPS average) is comfortably a boring Postgres problem. `gateway` scales per-tenant and is embarrassingly parallel across institutions.

## Architecture overview

Mirrors Ecuador's regulated topology (redes de pagos + SIP + RPI) so every component maps to a real-world role:

```
                          ┌─────────────────────────────┐
                          │   sandbox "BCE" (simulated)  │
                          │  ┌──────────┐ ┌───────────┐ │
                          │  │ integrator│ │ settlement│ │   integrator = SIP-like
                          │  │  (switch) │ │  (RTGS)   │ │   settlement = RPI/CCE-like
                          │  └────┬─────┘ └─────┬─────┘ │
                          └───────┼─────────────┼───────┘
                 pacs.008/002     │             │  gross, real-time,
                 acmt (llaves)    │             │  no overdraft, irrevocable
              ┌───────────────────┼─────────────┼──────────────────┐
              │                   │             │                  │
        ┌─────┴─────┐       ┌─────┴─────┐                   ┌─────┴─────┐
        │  red de   │       │  red de   │                   │ directory │
        │  pagos A  │       │  pagos B  │                   │ (llaves)  │
        └─────┬─────┘       └─────┬─────┘                   └───────────┘
              │                   │            per-network distributed dirs
     ┌────────┴───────┐     ┌─────┴────┐       + central resolution
     │    gateway     │     │ gateway  │
     │ (participant)  │     │          │
     └────────┬───────┘     └──────────┘
              │  REST/gRPC internal API
        ┌─────┴─────┐
        │ core bank │  (each institution's existing system)
        └───────────┘
```

## Repository shape (Go monorepo, moov-io DX conventions)

This repository is the **single source of truth**: code, specs, docs, research, manifesto, and community artifacts all live here. No satellite repos until scale forces it.

```
rpi-ec/
├── MANIFIESTO.md      The public manifesto (Spanish; EN translation in docs/)
├── docs/
│   ├── research/      Landscape, regulatory analysis, references
│   ├── architecture/  This blueprint, scheme-design.md, ADRs
│   ├── plan/          Action plan, outreach strategy
│   └── rfcs/          Numbered RFCs for scheme rules and protocol proposals
├── spec/              Machine-readable contracts: OpenAPI (North API),
│                      XSD/profile definitions (South API), QR payload spec.
│                      Versioned releases, Pix-style. The spec IS a product.
├── iso20022/          Go structs + XML/JSON codecs + validation for the BCE
│                      profile: pacs.008, pacs.002, acmt.* (llave lookup).
│                      Codegen from XSD; golden-file tests; fuzzing.
├── llaves/            Directory service: key registration, uniqueness,
│                      resolution, portability, block/unblock. DICT/DICE-inspired.
│                      Pluggable storage (Postgres first).
├── switch/            Stateless router/validator (UPI-inspired): schema +
│                      signature validation, BIC routing, timeouts ("time out"
│                      per BCE norm), idempotency, metadata-only logging.
├── settlement/        RTGS simulator: participant accounts, gross real-time
│                      debit/credit, no overdraft, irrevocability events
│                      (pacs.002 emission). Double-entry ledger core.
├── gateway/           THE PRODUCT for participants. Sits between a core
│                      banking system and a red de pagos: exposes a simple
│                      REST/gRPC + webhook API internally, speaks signed
│                      ISO 20022 over mTLS externally. Includes llave
│                      management, QR generation/parsing, reconciliation.
├── qr/                EMV QRCPS (merchant-presented) encoder/decoder carrying
│                      llaves, per the upcoming BCE QR standard.
├── sandbox/           "Ecuador payments network in a box": docker compose
│                      spinning up 2 redes + N mock banks + integrator +
│                      settlement + directory, seeded with test identities.
│                      One command → end-to-end payment demo.
├── tck/               Test & certification kit (Mojaloop TTK analog):
│                      scenario runner + conformance reports against the
│                      catálogo técnico/operacional. Target: become the de
│                      facto pre-certification tool for BCE qualification.
├── console/           Web UI (later): observe flows, inspect messages,
│                      manage sandbox participants. Great for demos/content.
└── examples/          "Connect a cooperativa in 30 minutes" tutorials.
```

Scheme-level design decisions (integration APIs, direct/indirect participation, llave rules, settlement model) are specified in [`scheme-design.md`](scheme-design.md).

### Technical decisions (proposed defaults)

- **Language**: Go throughout. Single static binaries per service, Docker images, Helm charts later. Cooperativa IT departments can run a binary; they cannot run a 40-pod Kubernetes hub (Mojaloop's practical adoption problem).
- **Messaging**: ISO 20022 XML on the wire (BCE mandate), JSON representations for internal APIs and debugging. Learn from `moov-io/rtp20022` on Go XML namespace handling.
- **Security**: mTLS everywhere + detached message signatures (JWS like Pix SPI), key rotation story from day one, audit log append-only. Maps directly to Art. 10 of BCE-GG-008-2025.
- **Ledger**: strict double-entry, immutable journal, event-sourced state for settlement. (Optionally benchmark against TigerBeetle as a pluggable backend later — good content, good performance story.)
- **Storage**: Postgres for everything at first. Boring wins trust in payments.
- **Protocol discipline**: every inter-service contract published as OpenAPI/protobuf with versioned releases, Pix-style. The spec *is* the product as much as the code.
- **License**: Apache-2.0 (same as bacen/pix-api and moov-io; patent grant matters for institutional adoption).

### What we deliberately do NOT build

- No custody, no user-facing wallet, no KYC/AML engine (integrate hooks, don't own the liability).
- No blockchain. Settlement is central bank money; the credibility of this project with regulators depends on saying that clearly.
- No fork of Mojaloop — different architecture (Ecuador is multi-network), different stack, different weight class. We cite them as prior art and possibly interop later.

## Naming (decided)

**RPI EC** — adopting the regulation's own nomenclature (Red de Pagos Instantáneos) so the project reads as an implementation of the national standard, not a parallel invention. Go module paths: `github.com/<org>/rpi-ec/...`. Non-affiliation disclaimer mandatory in README, manifesto, and site.
