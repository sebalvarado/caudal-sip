# Proyecto Caudal — Open Real-Time Payments for Ecuador

The open-source implementation of Ecuador's real-time payments standard (**RPI EC** / SIP): the scheme ledger, ISO 20022 messaging, llaves de pago, interoperable QR, and the participant tooling to adopt it all.

> **Proyecto Caudal is an independent open-source project. It is not affiliated with the Banco Central del Ecuador.** "RPI" and "SIP" refer to the public infrastructures and standards defined in current Ecuadorian regulation, which this project implements.

**The moment**: Ecuador's central bank has mandated interoperable real-time payments. By **November 30, 2026**, every payment network and financial institution — 24 banks, 365+ cooperativas, mutualistas, fintechs — must implement ISO 20022 messaging (pacs.008/pacs.002/acmt), payment keys, and standardized QR. There is no open-source tooling for any of it. This project builds it in public.

Read the manifesto: [MANIFIESTO.md](MANIFIESTO.md) (Spanish). Website: [caudal-sip.pages.dev](https://caudal-sip.pages.dev).

Inspired by [Pix](https://github.com/bacen/pix-api) (Brazil), Bre-B (Colombia), UPI (India); by [Mojaloop](https://github.com/mojaloop/mojaloop) and [moov-io](https://github.com/moov-io) as open-source prior art; and by [Medusa](https://medusajs.com/) / [Twenty](https://twenty.com/) as the open-core community model.

## Repository map (single source of truth)

Everything lives in this monorepo: code, specs, docs, primary sources, the website, and the business track.

| Path | Contents |
|---|---|
| [MANIFIESTO.md](MANIFIESTO.md) | The public manifesto (Spanish) |
| [docs/README.md](docs/README.md) | **Start here** — documentation map and reading order |
| [docs/research/catalogo-operacional.md](docs/research/catalogo-operacional.md) | The BCE's Catálogo Operacional decoded (Spanish): states, codes, llaves, timing, netting, liquidity risk. Normative |
| [docs/research/landscape.md](docs/research/landscape.md) | Ecuador's 2024–2026 regulatory framework, deep dives on Pix / Bre-B / UPI, open-source landscape |
| [docs/product/](docs/product/north-star.md) | North star and trust ladder, stakeholders, requirements, deliverables, adoption strategy |
| [docs/architecture/blueprint.md](docs/architecture/blueprint.md) | Positioning, components, repository shape, operator/deployment model |
| [docs/architecture/scheme-design.md](docs/architecture/scheme-design.md) | Integration APIs (North/South), direct & indirect participants, llave rules, settlement model |
| [docs/architecture/red.md](docs/architecture/red.md) | What a red de pagos deploys, and what deploying it does not give you |
| [docs/architecture/data-model.md](docs/architecture/data-model.md) | Entities, payment and llave lifecycles, ledger and audit invariants |
| [docs/architecture/technical-requirements.md](docs/architecture/technical-requirements.md) | Latency budget, availability, security, testing and observability bar |
| [docs/architecture/adr/](docs/architecture/adr/README.md) | Architecture Decision Records — including [ADR-0011](docs/architecture/adr/0011-rust-monorepo.md): the backend is Rust |
| [docs/plan/epics.md](docs/plan/epics.md) | **The roadmap to 2026-11-30**: three epics, acceptance criteria, how agents work |
| [docs/plan/core-build-order.md](docs/plan/core-build-order.md) | Build order for the central system: the ledger core comes first |
| [docs/development/local-stack.md](docs/development/local-stack.md) | The one-command local network we're committing to build |
| [docs/rfcs/](docs/rfcs/README.md) | RFC process for scheme-level proposals |
| [docs/glossary.md](docs/glossary.md) | Bilingual (ES/EN) domain vocabulary of record |
| [research/bce/](research/bce/README.md) | Primary BCE sources: official PDFs, searchable transcriptions, URLs and SHA-256 |
| [workstream/authorized-red/](workstream/authorized-red/README.md) | Business and licensing track (Spanish): becoming a BCE-authorized red de pagos |
| [site/](site/README.md) | Landing page + blog (Astro). `cd site && npm install && npm run dev` |
| [AGENTS.md](AGENTS.md) | Context and rules for AI agents working in this repo |

## Building

The backend is a Rust Cargo workspace ([ADR-0011](docs/architecture/adr/0011-rust-monorepo.md)); it lands with the ledger crate. Once it does:

```sh
cargo build --workspace
cargo test --workspace
cargo clippy --workspace -- -D warnings
```

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md), the [Code of Conduct](CODE_OF_CONDUCT.md), and [GOVERNANCE.md](GOVERNANCE.md). Security reports: [SECURITY.md](SECURITY.md). Licensed [Apache-2.0](LICENSE).

## Status

Design is done for the central system's core; implementation starts with the ledger in Rust. The website and blog are live. Nothing to run yet — read the blueprint, open an issue, join the discussion.
