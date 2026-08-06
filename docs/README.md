# Documentation Map

How to navigate this project's documentation, whether you're a contributor, an institution evaluating the software, or an AI agent doing implementation work.

## Reading order (first hour)

| # | Document | What you'll learn | Time |
|---|---|---|---|
| 1 | [research/landscape.md](research/landscape.md) | Why this project exists: Ecuador's 2024–2026 real-time payments regulation, deep dives on Pix / Bre-B / UPI, the open-source gap | 20 min |
| 2 | [architecture/blueprint.md](architecture/blueprint.md) | What we're building: components, monorepo layout, the four-operator deployment model, PaaS/SaaS strategy | 15 min |
| 3 | [architecture/scheme-design.md](architecture/scheme-design.md) | How it works: North/South APIs, direct & indirect participants, llave rules, settlement model | 15 min |
| 4 | [plan/action-plan.md](plan/action-plan.md) | When and how: phases, outreach strategy, risks | 10 min |

Then, depending on your role:

- **Writing code?** → [architecture/adr/](architecture/adr/README.md) (decisions already made) and [glossary.md](glossary.md) (the vocabulary of record). Also read `/AGENTS.md` at the repo root.
- **Proposing a change to scheme behavior?** → [rfcs/](rfcs/README.md).
- **Evaluating the project for your institution?** → the [manifesto](../MANIFIESTO.md) (Spanish) and the blueprint's operator model section.
- **Press / policy audience?** → the [website](https://caudal-sip.pages.dev) and the manifesto.

## Structure

```
docs/
├── README.md            ← you are here
├── glossary.md          Bilingual (ES/EN) domain vocabulary of record
├── spanish-style.md     Style rules for public-facing Spanish (no translated prose)
├── research/            Evidence base: regulation, reference systems, prior art
├── architecture/
│   ├── blueprint.md     Components, repo shape, operator/deployment model
│   ├── scheme-design.md APIs, participants, llaves, settlement
│   └── adr/             Architecture Decision Records (numbered, immutable)
├── plan/
│   └── action-plan.md   Phases, outreach, risks
└── rfcs/                Requests for Comments: scheme-level proposals
```

## Documentation rules

1. **English here, Spanish outward.** Engineering docs (this folder) are English; the manifesto, website, and tutorials are Spanish. The [glossary](glossary.md) bridges both, and public Spanish must follow [spanish-style.md](spanish-style.md).
2. **Decisions leave a trail.** If it changed how the system works and it isn't in an ADR or RFC, it didn't happen. PR descriptions are not documentation.
3. **Cite the norm.** Claims about what Ecuadorian regulation requires must cite the specific resolution and article (see landscape.md for links). Assumptions where the catálogo técnico is still unknown must be marked **[assumption]**.
4. **Keep the manifesto mirrors in sync**: `/MANIFIESTO.md` (canonical) and `/site/src/pages/manifiesto.md` (website copy).
