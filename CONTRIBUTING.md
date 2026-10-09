# Contributing to RPI EC (Proyecto Caudal)

Thank you for helping build open payments infrastructure for Ecuador. Contributions of all kinds are welcome: code, documentation, translations, regulatory analysis, testing, and design.

## Ground rules

- **License**: all contributions are accepted under [Apache-2.0](LICENSE). By submitting a pull request you certify the [Developer Certificate of Origin](https://developercertificate.org/) — sign your commits with `git commit -s`.
- **Language policy** — full version in [docs/language-policy.md](docs/language-policy.md):
  - **Spanish** for anything an Ecuadorian institution reads to do its job: the catálogo reference, conformance material, technical proposals, operator runbooks, console interfaces, tutorials, the manifesto and the website. Written as native Spanish per [docs/spanish-style.md](docs/spanish-style.md) — never translated prose.
  - **English** for code, comments, identifiers, commit messages, ADRs, RFCs, and contributor documentation, so the global open-source community can participate.
  - **One canonical language per document.** Never maintain a translated mirror; flagship Spanish documents may carry a short English abstract instead.
  - The [glossary](docs/glossary.md) is bilingual and is the vocabulary of record. Use its terms consistently — e.g. *llave* (not "alias key"), *integrante* (not "member bank").
- **No real credentials, ever.** This repository must never contain production certificates, keys, account data, or institution-identifying test data. See [SECURITY.md](SECURITY.md).

## Where to start

1. Read the [docs reading map](docs/README.md) — 30 minutes gets you the full picture.
2. Check issues labeled `good-first-issue` and `help-wanted`.
3. For anything that changes scheme behavior (message profiles, directory rules, settlement semantics), open an [RFC](docs/rfcs/README.md) first.
4. For significant technical decisions inside a component, record an [ADR](docs/architecture/adr/README.md).

## Development

The Cargo workspace is not scaffolded yet (see the [blueprint](docs/architecture/blueprint.md) and [ADR-0011](docs/architecture/adr/0011-rust-monorepo.md)). Until then, the website is the only buildable artifact:

```bash
cd site && npm install && npm run dev
```

Tools: [rustup](https://rustup.rs), [just](https://just.systems), Docker, and Node 22 for the site; [Worktrunk](https://worktrunk.dev) (`wt`) and `sccache` if you run parallel worktrees. On macOS: `brew install rustup just worktrunk sccache`. `just doctor` reports what is missing.

The loop is `just check <crate>` and `just test <crate>` while working, then `just ci` — format, clippy with warnings denied, tests, `cargo deny`, and the site build — once before opening a PR. CI runs the same checks.

## Pull requests

- Keep PRs focused and small; link the issue, ADR, or RFC they implement.
- PRs need one maintainer approval to merge ([GOVERNANCE.md](GOVERNANCE.md)).
- Tests accompany code. For message-format work (`iso20022/`), golden-file tests against catalog examples are required.

## Conduct

We enforce the [Code of Conduct](CODE_OF_CONDUCT.md). Payments infrastructure needs a community that institutions can trust — professionalism is part of the product.
