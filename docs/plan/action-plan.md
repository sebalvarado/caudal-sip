# Action Plan: Build in Public, Aim at the Deadline

> **This plan covers the participant-facing track only.** The build order for the central system — the SIP-equivalent core, which is now ledger-first — is in [`core-build-order.md`](core-build-order.md). The two plans have not yet been reconciled into a single roadmap.

The strategic clock: **November 30, 2026** — every red de pagos and every integrante must speak the BCE's ISO 20022 standard for intra-network real-time payments. Hundreds of institutions, especially cooperativas, need tooling that does not exist. Every month between now and then, this project gets more relevant, not less.

## Phase 0 — Foundation (weeks 1–4)

1. ~~**Obtain the catálogo técnico y operacional**~~ — **half done (2026-08-05)**. The *Catálogo Operacional* v1.0 is published on bce.fin.ec and is decoded in [`../research/catalogo-operacional.md`](../research/catalogo-operacional.md): payment states, reason codes, llave types, timing windows, message versions, netting, and the liquidity-risk engine. **Still outstanding: the catálogo técnico** (field-level XSDs and validation rules). Routes: Registro Oficial (Segundo Suplemento Nº 245), Asobanca, a LOTAIP public-records request, or Gerencia de Medios de Pago y Servicios Nacionales directly.
2. **Get into the foros de interoperabilidad** (Comité Consultivo Interinstitucional de Pagos, created by BCE-GG-008-2025). Attend as fintech/expert. Being in the room = early access to standards drafts + credibility + distribution.
3. **License + manifesto.** Name decided: **RPI EC** (with non-affiliation disclaimer). Apache-2.0. Manifesto drafted in Spanish at `MANIFIESTO.md` — publish as the first post.
4. **Publish this research** (`docs/research/landscape.md`) as the opening content piece. It is already better documentation of Ecuador's RTP framework than anything public today — that alone earns attention from the industry and the BCE.

### Cold-start outreach strategy (no existing BCE/industry contacts)

We start with zero warm contacts; the artifacts open the doors. Ordered by expected yield:

1. **LOTAIP public-records request to the BCE** for the catálogos and forum participation criteria. Legally answerable, creates a paper trail, requires no relationship.
2. **Publish first, then knock.** The landscape research + manifesto + a working sandbox demo video are the credentials. Cold emails that link to a running network-in-a-box get answered; cold emails asking for meetings don't.
3. **Asobanca and cooperativa federations (RFEC, UCACSUR, cajas centrales)** — they republish BCE norms and their members carry the compliance burden; a free implementation kit is an easy conversation. Ask for a slot in their technical committees/webinars.
4. **Core-banking vendors for cooperativas** (Fitbank, Cobiscorp, and the long tail of local vendors) — one vendor integration = dozens of institutions. They become distribution, not competition, if the gateway is Apache-2.0.
5. **IDB / World Bank payments teams** — both advised BCE and Banrep; both fund digital public infrastructure. An OSS reference implementation is exactly what their agenda rewards; a warm intro to the BCE via a multilateral is worth ten cold emails.
6. **LinkedIn/X in Spanish, targeting BCE Gerencia de Medios de Pago staff and Ecuadorian fintech community** (Buen Trip Hub, AEFI - Asociación Ecuatoriana de Fintech). Publish threads summarizing each resolution in plain developer language; the people who wrote the norms will notice.
7. **Ecuadorian dev diaspora** — recruit contributors from Ecuadorian engineers at fintechs abroad (they have both the skills and the motivation to build for home).

## Phase 1 — The wedge: message library + sandbox (months 1–3)

Ship the two things with immediate standalone value:

- **`iso20022/`**: Go library for the BCE profile (pacs.008, pacs.002, acmt llave messages). Reader/writer/validator + CLI (`chasqui validate payment.xml`) + tiny HTTP service, moov-io style. Until the catálogo is in hand, build against the generic ISO 20022 XSDs + Bre-B/Pix profiles and structure for country profiles (a strength, not a delay: same core could later serve other LatAm profiles).
- **`sandbox/`**: docker compose network-in-a-box — 2 mock banks, 1 red, directory, settlement sim. **The demo is the marketing**: a video of a llave-addressed, QR-initiated payment clearing end-to-end in a simulated Ecuadorian network in under 2 seconds is the artifact that makes bureaucrats' "this is very hard" claim collapse.

Success metric: an outside Go developer sends a simulated real-time payment within 15 minutes of cloning.

## Phase 2 — The product: participant gateway + certification kit (months 3–6)

- **`gateway/`**: the Medusa move — the self-hostable piece a cooperativa actually deploys. Simple internal REST API + webhooks on one side; compliant, signed, mTLS ISO 20022 on the other. Llave management and QR included.
- **`tck/`**: conformance runner producing pass/fail reports against the standard. Institutions must pass BCE "calificación técnica y operativa" — the open pre-certification tool becomes infrastructure everyone (including networks and the BCE) benefits from.
- **First design partners**: recruit 2–3 cooperativas or a caja central / cooperativa federation (RFEC) + ideally one core-banking vendor (the software houses serving hundreds of cooperativas are a multiplier: one integration → many institutions).

## Phase 3 — The reference network + institutional push (months 6–12)

- Complete `switch/`, `llaves/`, `settlement/` into a full reference scheme; publish load-test numbers (Go + Postgres on modest hardware vs. national TPS needs — Bre-B peaks at ~5M tx/day ≈ 60 TPS average; Ecuador's will be far less. Publishing this math is itself powerful advocacy).
- **Institutional strategy**: present at foros de interoperabilidad; brief Asobanca, SEPS, cooperativa federations; approach IDB/World Bank payments teams (both already advised BCE/Banrep — they love reusable public goods); propose to the BCE: "adopt the sandbox as the public test environment, as BCB did with Pix specs on GitHub ([bacen/pix-api](https://github.com/bacen/pix-api))."
- **Community**: RFC process for scheme rules (fees, dispute flows, fraud data sharing — invite economists and lawyers, not just devs); Spanish-first tutorials; talks at Ecuadorian universities (ESPOL, USFQ, EPN) and LatAm fintech events; court the Ecuadorian dev diaspora.

## Content engine (continuous)

Weekly, in public, ES+EN: "Building Ecuador's open payments stack" — each component ships with a post explaining the payments concept behind it (what is gross settlement? why no overdraft? how does a llave directory prevent fraud?). The educational content is what draws contributors and makes regulators comfortable; nobody in the ecosystem is publishing this in Spanish.

## Risks and honest caveats

- **The catálogo técnico may be slow to obtain or under NDA within forums.** Reduced: the operational catalog is public and gives us states, codes, flows, message versions and mandatory fields, so `iso20022/` and `tck/` can start now. Mitigation for the rest: build profile-driven (generic core + country profile), track Bre-B/Pix profiles meanwhile.
- **BCE may see an OSS implementation as encroachment.** Mitigation: relentless positioning as *complement* (participant-side tooling + education), never "replacement for SIP." The Pix-on-GitHub precedent is the talking point.
- **BANRED and incumbent vendors sell competing integration services.** That's validation. OSS wins the long tail (cooperativas) that vendors underserve.
- **Timeline slippage by BCE** (18-month SIP deadline was ambitious). Doesn't hurt us: participant-side deadlines still stand, and slippage increases demand for ready-made tooling.

## Immediate next actions

- [x] Decide name + create GitHub org
- [x] ~~Hunt down the catálogo operacional~~ — found, decoded, docs reconciled
- [ ] Request the **catálogo técnico** (field-level profiles) via LOTAIP
- [ ] Raise the `pacs.008.001.08` vs `pacs.008.004.08` version inconsistency with the BCE — a small, precise, well-sourced question is the cheapest possible first contact
- [ ] Contact BCE re: foros de interoperabilidad participation
- [ ] Scaffold the Go monorepo (`iso20022/` first) 
- [ ] Publish manifesto + landscape research as posts 1 and 2
