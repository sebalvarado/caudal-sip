# RPI EC — Open Real-Time Payments for Ecuador

The open-source implementation of Ecuador's real-time payments standard (RPI/SIP): ISO 20022 messaging, llaves de pago, interoperable QR, and the participant tooling to adopt it all.

> **RPI EC is an independent open-source project. It is not affiliated with the Banco Central del Ecuador.** "RPI" and "SIP" refer to the public infrastructures and standards defined in current Ecuadorian regulation, which this project implements.

**The moment**: Ecuador's central bank has mandated interoperable real-time payments. By **November 30, 2026**, every payment network and financial institution — 24 banks, 365+ cooperativas, mutualistas, fintechs — must implement ISO 20022 messaging (pacs.008/pacs.002/acmt), payment keys, and standardized QR. There is no open-source tooling for any of it. This project builds it in public.

Read the manifesto: [MANIFIESTO.md](MANIFIESTO.md) (Spanish). Website: [caudal-sip.pages.dev](https://caudal-sip.pages.dev).

Inspired by [Pix](https://github.com/bacen/pix-api) (Brazil), Bre-B (Colombia), UPI (India); by [Mojaloop](https://github.com/mojaloop/mojaloop) and [moov-io](https://github.com/moov-io) as open-source prior art; and by [Medusa](https://medusajs.com/) / [Twenty](https://twenty.com/) as the open-core community model.

## Repository map (single source of truth)

| Path | Contents |
|---|---|
| [MANIFIESTO.md](MANIFIESTO.md) | The public manifesto (Spanish) |
| [docs/README.md](docs/README.md) | **Start here** — documentation map and reading order |
| [docs/research/landscape.md](docs/research/landscape.md) | Full research: Ecuador's 2024–2026 regulatory framework (SIP, RPI, deadlines), deep dives on Pix / Bre-B / UPI, open-source landscape, design principles |
| [docs/product/](docs/product/north-star.md) | North star and trust ladder, stakeholders, product requirements, adoption strategy |
| [docs/architecture/blueprint.md](docs/architecture/blueprint.md) | Positioning, architecture, Go monorepo shape, operator/deployment model |
| [docs/architecture/scheme-design.md](docs/architecture/scheme-design.md) | Integration APIs (North/South), direct & indirect participants, llave rules and account mapping, settlement model |
| [docs/architecture/data-model.md](docs/architecture/data-model.md) | Entities, payment and llave lifecycles, ledger and audit invariants |
| [docs/architecture/technical-requirements.md](docs/architecture/technical-requirements.md) | Latency budget, availability, security, testing and observability bar |
| [docs/development/local-stack.md](docs/development/local-stack.md) | The one-command local network we're committing to build |
| [docs/architecture/adr/](docs/architecture/adr/README.md) | Architecture Decision Records — decisions made and why |
| [docs/rfcs/](docs/rfcs/README.md) | RFC process for scheme-level proposals |
| [docs/glossary.md](docs/glossary.md) | Bilingual (ES/EN) domain vocabulary of record |
| [docs/plan/action-plan.md](docs/plan/action-plan.md) | Phased plan + cold-start outreach strategy |
| [AGENTS.md](AGENTS.md) | Context and rules for AI agents working in this repo |
| [research/README.md](research/README.md) | Primary sources and where to find them (papers are linked, not vendored) |
| [site/](site/README.md) | Landing page + blog (Astro). `cd site && npm install && npm run dev` |
| `spec/`, `iso20022/`, `gateway/`, `sandbox/`, `tck/`, ... | Coming next — see the blueprint |

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md), the [Code of Conduct](CODE_OF_CONDUCT.md), and [GOVERNANCE.md](GOVERNANCE.md). Security reports: [SECURITY.md](SECURITY.md). Licensed [Apache-2.0](LICENSE).

## Status

Research and design phase. Nothing to run yet — read the blueprint, open an issue, join the discussion.
