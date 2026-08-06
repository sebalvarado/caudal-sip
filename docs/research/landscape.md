# Research Landscape: Real-Time Payments for Ecuador

> Compiled August 2026. Sources linked inline. This is the evidence base for the project blueprint.

## 1. The headline finding: Ecuador's regulatory train is already moving

The common narrative ("Ecuador has been stuck for 10 years") is now outdated. Between late 2024 and early 2026, the Banco Central del Ecuador (BCE) issued a full regulatory framework for interoperable real-time payments, with hard deadlines:

| Instrument | Date | What it does |
|---|---|---|
| Resolución JPRM-2024-018-M | Sep 2024 | Regulates payment systems/media and fintech activities |
| Resolución JPRM-2024-029-M | Dec 2024 | Adds interoperability obligations; orders BCE to build the SIP within 18 months |
| [BCE-GG-008-2025](https://www.bce.fin.ec/storage/BCE/resoluciones/BCE-GG-008-2025.pdf) | May 2025 | Implementation norm: stages, milestones, deadlines, security requirements, QR mandate, interoperability forums |
| [BCE-GG-007-2026](https://www.bce.fin.ec/storage/BCE/resoluciones/BCE-GG-007-2026.pdf) ([Registro Oficial copy](https://asobanca.org.ec/wp-content/uploads/2026/03/RO-Res.-BCE-GG-007-2026-Estandar-mensajeria-transferencias-electronicas-pago-tiempo-real.pdf)) | Mar 2026 | Messaging standard: ISO 20022 pacs.008 / pacs.002 / ACMT for payment keys ("llaves") |

### Ecuador's chosen architecture (from the resolutions)

The model is closest to Colombia's Bre-B (multi-network + central integrator), not to Pix's single central switch:

- **Redes de pagos** (payment networks): private infrastructures (e.g. BANRED) that route real-time payments among their member institutions ("integrantes"). Each network manages a **distributed directory** of payment keys.
- **SIP — Sistema Integrador de Pagos** (BCE-run): resolves keys and clears **inter-network** payments, feeding settlement.
- **RPI — Red de Pagos Instantáneos** (BCE-run): the central bank's own instant payment network (analog of Colombia's Drixi).
- **Llaves de pago**: public aliases (like Pix keys / Bre-B llaves) mapping to "credenciales de pago" (cédula/RUC/passport + account number + account type + institution BIC).
- **Messaging**: ISO 20022 — `pacs.008` (payment order), `pacs.002` (status/irrevocability), `acmt.*` (key resolution). Details live in a **catálogo técnico** and **catálogo operacional** issued by the BCE.
- **Scope**: real-time transfers up to **USD 15,000**. Participants identified by **SWIFT BIC** (BCE assigns codes to entities without one).
- **QR codes**: must carry the beneficiary's llave; a BCE QR standard is mandated (EMV QRCPS is the regional norm, used by Bre-B).
- **Security baseline** (Art. 10, BCE-GG-008-2025): encryption in transit/at rest, strong mutual authentication, vulnerability management, monitoring, business continuity, DR, audit trails.

### Key deadlines

- **Nov 30, 2026**: all networks and members must use the ISO 20022 standard for **intra-network** real-time payments.
- **When SIP enters production** (originally mandated ~mid-2026): inter-network messaging becomes mandatory per BCE conditions. Rollout is phased by institution size (assets ≥ USD 4B first, then ≥ USD 1.5B, etc.).
- The BCE convenes **foros de interoperabilidad** (Comité Consultivo Interinstitucional de Pagos) where the technical/operational model is discussed with industry. This is the door for participation.

### Why this matters for the project

Roughly **24 private banks, 365 cooperativas de ahorro y crédito, 4 mutualistas, and 1 public bank** participate in the BCE's current SPI ([BCE SPI page](https://www.bce.fin.ec/sistema-de-pagos/sistema-central-de-pagos/sistema-de-pagos-interbancarios-spi/)). Nearly all of them — especially the hundreds of cooperativas — now face an ISO 20022 + llaves + QR implementation mandate with no open-source tooling, no public sandbox, and scarce in-house payments engineering. That is the gap this project fills.

### Current state (pre-reform)

- The **SPI** (Sistema de Pagos Interbancarios) is deferred-settlement: three daily windows, cleared and settled at the BCE. Real-time exists only through BANRED's private closed-loop "Pago Directo" service, settled once a day ([BCE/IDB technical note](https://www.bce.fin.ec/storage/JPRM/consulta-publica/Nota-Tecnica-Interoperabilidad.pdf)).
- Ecuador recorded ~**11 account-to-account transfers per capita in 2022** — among the lowest in the Bre-B working paper's international comparison (Brazil ~200, Colombia 36). Massive headroom.
- Ecuador is **dollarized**: no FX or monetary-policy complexity in the settlement layer, which simplifies design relative to Brazil/Colombia.

## 2. Reference systems

### Pix (Brazil) — the gold standard for central-bank-run instant payments

- **Architecture**: BCB runs two central components on the RSFN network: **SPI** (real-time gross settlement over special-purpose "Contas PI" at the central bank, no overdraft allowed, irrevocable once settled) and **DICT** (alias directory: phone, email, CPF/CNPJ, random key). Direct and indirect participation. ISO 20022 messaging; DICT is a REST/OpenAPI 3.0 API; security via mutual TLS + digital signatures (JWS for SPI messages, XMLDSig for DICT). Sources: [BCB Resolution 195/2022 (EN)](https://www.bcb.gov.br/content/financialstability/paymentssystem_docs/Res%20BCB%20195.pdf), [SPI annual report](https://www.bcb.gov.br/content/estabilidadefinanceira/relatorios_SPI/relatorio_anual_spi_2025.pdf), [World Bank Pix case study](https://fastpayments.worldbank.org/sites/default/files/2022-05/Brazil_Pix_Case_Study_May_2022_0.pdf).
- **Openness as strategy**: BCB publishes the Pix API specs on GitHub — [bacen/pix-api](https://github.com/bacen/pix-api) (~2.9k stars, Apache-2.0, OpenAPI 3.0), plus [pix-dict-api](https://github.com/bacen/pix-dict-api) and a Java [DICT quickstart](https://github.com/bacen/pix-dict-quickstart). A central bank shipping specs on GitHub with releases and issues is precedent we can point Ecuadorian regulators to.
- **Results**: 170M+ users, >80% of the population, free P2P mandated for individuals.

### Bre-B (Colombia) — the closest analog to Ecuador's chosen model

- Full working paper downloaded at `research/bre-b-working-paper.pdf` (Banco de la República, March 2026).
- **Architecture**: multiple competing private instant payment systems (SPBVIs: Transfiya, Visionamos, Entrecuentas/Redeban, Credibanco, Servibanca) interoperate through two central-bank components: **DICE** (centralized alias directory, guarantees alias uniqueness, resolves inter-SPBVI lookups; federated directories handle intra-SPBVI) and **MOL** (real-time gross settlement in central bank money via CUD deposit accounts). ISO 20022 + APIs mandated.
- **Rules that matter**: max 20 seconds end-to-end; cap of 1,000 UVB per transaction; P2P free to end users; unified brand and UX ("Bre-B Zone") across all participant apps; EMV QRCPS QR standard with a forced migration deadline.
- **Process playbook** (highly reusable for Ecuador advocacy): 2022 Payment Systems Forum with 90+ actors → legal mandate (Art. 104, Law 2294/2023) → central bank regulation (External Resolution 6/2023 + Circular DSP-465 with technical annexes) → five implementation phases (governance → standards → tech build → alias registration → launch, Oct 2025) → 218 participants, 99M aliases, 370M transactions in under 4 months.
- **Strategic insight**: "build on what has been built" — don't replace private networks; force them to interoperate through thin public components. Ecuador copied this shape.

### UPI (India) — the architecture lesson: a stateless switch

- **NPCI runs a stateless L7 switch**: it terminates TLS, validates schema and signatures, resolves Virtual Payment Addresses (VPAs like `alice@bank`), routes debit/credit legs between banks, applies rate limits and fraud heuristics, and logs only metadata. **All state (balances, accounts, KYC) stays at the banks.** Four-party model: payer PSP app, payer bank, payee PSP, payee bank. Deferred net settlement via RBI. Sources: [NPCI report](https://www.npci.org.in/uploads/UPI_The_Global_Benchmark_for_Digital_Payments_Digital_Version_1_b38d2c9598.pdf), [UPI architecture deep-dive](https://dev.to/mr_harshkumar_jha/upi-architecture-the-story-of-how-npci-became-indias-l7-super-switch-5gm), [system design walkthrough](https://dev.to/zeeshanali0704/system-design-upi-unified-payment-interface-npci-a-practical-end-to-end-system-design-kkc).
- **Governance lesson**: NPCI is a not-for-profit owned by the banking industry under central bank guidance — a third model between "central bank runs everything" (Pix) and "many private networks + thin public core" (Bre-B/Ecuador).
- UPI runs as an **open API layer on top of an existing rail** (IMPS) — evidence that a protocol/API layer can be built before/independently of new settlement infrastructure.

## 3. Open-source landscape (what exists, what doesn't)

| Project | What it is | Relevance |
|---|---|---|
| [Mojaloop](https://github.com/mojaloop/mojaloop) (Mojaloop Foundation, Gates-funded) | Full open-source instant-payments hub: central ledger, account lookup service (ALS + oracles), scheme adapter SDK, Testing Toolkit (TTK). Node.js/TypeScript, Kubernetes-first. Uses ILPv4 for cryptographic transfer terms and an [ISO 20022 JSON profile](https://docs.mojaloop.io/product/features/Iso20022/MarketPracticeDocument.html). | Proof that OSS national payment switches are viable (deployed in several African markets). Its component decomposition (hub / ALS / scheme adapter / testing toolkit) is the best-in-class map to imitate. Not Go, not ISO 20022 XML-native, heavy to operate — a lighter Go alternative has room. |
| [moov-io](https://github.com/moov-io) | Go OSS fintech libraries: [ach](https://github.com/moov-io/ach), wire, [rtp20022](https://github.com/moov-io/rtp20022), fednow20022, fedwire20022, watchman (sanctions), etc. Pattern: one repo = one responsibility; reader + writer + validator + HTTP server + Docker image. | The *repo architecture and DX model* to copy for Go. `rtp20022` shows how to model ISO 20022 pacs messages in Go (XML namespace caveats documented). |
| [bacen/pix-api](https://github.com/bacen/pix-api) | Regulator-published OpenAPI specs | The precedent for spec-first, GitHub-first regulation. |
| Interledger / ILPv4 | Cryptographic transfer conditions between ledgers | Used by Mojaloop for non-repudiation of two-phase transfers; optional inspiration for the clearing protocol. |
| Medusa / Twenty | OSS commerce / CRM platforms | The *community and product* model: developer-first, modular, self-hostable, commercial cloud later. |

**The gap**: there is no open-source, Go, ISO 20022-native implementation of a Latin-American style instant payment scheme (keys directory + switch + RTGS + participant gateway + certification kit), and specifically nothing targeting Ecuador's SIP/RPI standard. Mojaloop is the closest but is a heavyweight hub in Node.js with its own API instead of the pacs/acmt profile Ecuador mandated.

## 4. Institutional actors to know (Ecuador)

- **BCE** — Gerencia de Medios de Pago y Servicios Nacionales; Gerencia de Supervisión e Innovación de Medios de Pago (runs the interoperability forums).
- **Junta de Política y Regulación Financiera y Monetaria (JPRFM)** — issues the enabling resolutions.
- **BANRED** — incumbent switch operator ("Pago Directo"), likely the first qualified "red de pagos".
- **Asobanca** (private banks association — already republishing the BCE resolutions), **RFEC / cooperativa federations** (the long tail of 365 institutions with the least tech capacity — our natural user base), **Superintendencia de Bancos** and **SEPS** (supervisors).
- **IDB**: provided technical assistance for the interoperability diagnosis ([technical note](https://www.bce.fin.ec/storage/JPRM/consulta-publica/Nota-Tecnica-Interoperabilidad.pdf)); funds regional payments modernization.

## 5. Design principles distilled from all three systems

1. **Thin, stateless center; state at the edges** (UPI). The switch validates, routes, logs metadata. Banks keep accounts.
2. **Settlement in central bank money, gross, real-time, no overdraft, irrevocable** (Pix SPI / Bre-B MOL). Never proprietary net settlement.
3. **Aliases with guaranteed uniqueness + fast resolution** (DICT / DICE / llaves). Directory design is where privacy, fraud, and UX meet.
4. **ISO 20022 + OpenAPI + mTLS + message signing** as the interop contract. Publish the specs publicly with versioned releases.
5. **Hard UX rules make or break adoption**: sub-20-second completion, free P2P, one brand, standard QR (EMV QRCPS), mandatory participation for large institutions.
6. **Phased rollout with forums**: governance → standards → build → alias registration → launch. Industry co-design forums de-risk everything.
7. **Certification/testing infrastructure is a first-class citizen** (Mojaloop TTK; Bre-B interoperability testing phase; BCE requires "calificación técnica y operativa"). Whoever provides the best test harness becomes indispensable.
