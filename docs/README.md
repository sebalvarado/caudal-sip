# Documentation Map

How to navigate this project's documentation, whether you're a contributor, an institution evaluating the software, or an AI agent doing implementation work.

## Reading order (first hour)

| # | Document | What you'll learn | Time |
|---|---|---|---|
| 1 | [product/north-star.md](product/north-star.md) | Where this is going and the ladder it has to climb to get there | 5 min |
| 2 | [research/landscape.md](research/landscape.md) | Why this project exists: Ecuador's 2024–2026 real-time payments regulation, deep dives on Pix / Bre-B / UPI, the open-source gap | 20 min |
| 3 | [research/catalogo-operacional.md](research/catalogo-operacional.md) 🇪🇸 | **The specification itself**, decoded: payment states, reason codes, llave rules, timing, netting, the liquidity-risk engine, and who is already obligated. The reference every component is built against. In Spanish, with an English abstract | 20 min |
| 4 | [architecture/blueprint.md](architecture/blueprint.md) | What we're building: components, monorepo layout, the four-operator deployment model, PaaS/SaaS strategy | 15 min |
| 5 | [architecture/scheme-design.md](architecture/scheme-design.md) | How it works: North/South APIs, direct & indirect participants, llave rules, settlement model | 15 min |
| 6 | [plan/action-plan.md](plan/action-plan.md) | When and how: phases, outreach strategy, risks | 10 min |

Then, depending on your role:

- **Writing code?** → [architecture/data-model.md](architecture/data-model.md) (entities, lifecycles, invariants), [architecture/technical-requirements.md](architecture/technical-requirements.md) (the non-functional bar), [architecture/adr/](architecture/adr/README.md) (decisions already made), and [glossary.md](glossary.md) (the vocabulary of record). Also read `/AGENTS.md` at the repo root.
- **Building the ledger or settlement?** → [architecture/ledger.md](architecture/ledger.md) (accounts, invariants, interface) then [architecture/posting-patterns.md](architecture/posting-patterns.md) (the exact postings for every flow). Fees live in [architecture/tariff.md](architecture/tariff.md).
- **Scoping a feature?** → [product/stakeholders.md](product/stakeholders.md) then [product/requirements.md](product/requirements.md). If it serves nobody in the first, it doesn't belong in the second.
- **Wondering what we actually ship, or where something belongs in the tree?** → [product/deliverables.md](product/deliverables.md).
- **Setting up to develop?** → [development/local-stack.md](development/local-stack.md).
- **Proposing a change to scheme behavior?** → [rfcs/](rfcs/README.md).
- **Evaluating the project for your institution?** → the [manifesto](../MANIFIESTO.md) (Spanish) and the blueprint's operator model section.
- **Doing outreach?** → [product/adoption.md](product/adoption.md) for the strategy, [outreach/](outreach/README.md) for the actual letters and what has been sent.
- **Press / policy audience?** → the [website](https://caudal-sip.pages.dev) and the manifesto.

## Structure

```
docs/
├── README.md            ← you are here
├── glossary.md          Bilingual (ES/EN) domain vocabulary of record
├── language-policy.md   Which language a document gets, and why
├── spanish-style.md     Style rules for Spanish (no translated prose)
├── research/
│   ├── catalogo-operacional.md  🇪🇸 The scheme spec decoded — the normative reference
│   └── landscape.md     Regulation, reference systems (Pix/Bre-B/UPI), prior art
├── product/
│   ├── north-star.md    Direction, trust ladder, guardrails — read before scoping
│   ├── stakeholders.md  Who we build for, the people inside them, who we don't
│   ├── deliverables.md  What ships, who uses each piece, how they touch it
│   ├── requirements.md  Jobs, phased scope, surfaces (including internal UIs), non-goals
│   └── adoption.md      Why each group says yes, objections, proof artifacts
├── architecture/
│   ├── blueprint.md     Components, repo shape, operator/deployment model
│   ├── scheme-design.md APIs, participants, llaves, settlement
│   ├── data-model.md    Entities, lifecycles, invariants across components
│   ├── ledger.md        Chart of accounts, ledger invariants, the Ledger interface
│   ├── posting-patterns.md  Exact postings per flow — the implementation reference
│   ├── tariff.md        Fee schedules, quoting, and why fees are their own module
│   ├── technical-requirements.md  Latency, availability, security, testing bar
│   └── adr/             Architecture Decision Records (numbered, immutable)
├── development/
│   └── local-stack.md   Running the whole simulated network locally
├── outreach/        🇪🇸 Drafts of institutional correspondence, and the register
├── plan/
│   ├── action-plan.md   Participant-facing track: phases, outreach, risks
│   └── core-build-order.md  Central system: ledger-first build sequence
└── rfcs/                Requests for Comments: scheme-level proposals
```

## Documentation rules

1. **Language follows audience**, per [language-policy.md](language-policy.md): Spanish for what an Ecuadorian institution reads to do its job (marked 🇪🇸 above), English for code, ADRs, RFCs, and contributor docs. One canonical language per document — never a translated mirror. The [glossary](glossary.md) bridges both, and Spanish must read native per [spanish-style.md](spanish-style.md).
2. **Decisions leave a trail.** If it changed how the system works and it isn't in an ADR or RFC, it didn't happen. PR descriptions are not documentation.
3. **Cite the norm.** Claims about what Ecuadorian regulation requires must cite the specific resolution and article, or the section of the Catálogo Operacional (see [research/catalogo-operacional.md](research/catalogo-operacional.md)). Assumptions where the catálogo *técnico* is still unknown must be marked **[assumption]**.
4. **Keep the manifesto mirrors in sync**: `/MANIFIESTO.md` (canonical) and `/site/src/pages/manifiesto.md` (website copy).
