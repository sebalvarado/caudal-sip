# Contributing to RPI EC (Proyecto Caudal)

Thank you for helping build open payments infrastructure for Ecuador. Contributions of all kinds are welcome: code, documentation, translations, regulatory analysis, testing, and design.

## Ground rules

- **License**: all contributions are accepted under [Apache-2.0](LICENSE). By submitting a pull request you certify the [Developer Certificate of Origin](https://developercertificate.org/) — sign your commits with `git commit -s`.
- **Language policy**:
  - Code, comments, commit messages, ADRs, and RFCs: **English** (so the global open-source community can participate).
  - The manifesto, outreach material, tutorials, and the website: **Spanish first** (our users are Ecuadorian institutions and developers), written as native Spanish per [docs/spanish-style.md](docs/spanish-style.md) — never translated prose.
  - The [glossary](docs/glossary.md) is bilingual and is the vocabulary of record. Use its terms consistently — e.g. *llave* (not "alias key"), *integrante* (not "member bank").
- **No real credentials, ever.** This repository must never contain production certificates, keys, account data, or institution-identifying test data. See [SECURITY.md](SECURITY.md).

## Where to start

1. Read the [docs reading map](docs/README.md) — 30 minutes gets you the full picture.
2. Check issues labeled `good-first-issue` and `help-wanted`.
3. For anything that changes scheme behavior (message profiles, directory rules, settlement semantics), open an [RFC](docs/rfcs/README.md) first.
4. For significant technical decisions inside a component, record an [ADR](docs/architecture/adr/README.md).

## Development

The Go workspace is being scaffolded (see the [blueprint](docs/architecture/blueprint.md) for the target layout). Until then, the website is the only buildable artifact:

```bash
cd site && npm install && npm run dev
```

When Go code lands, the standard loop will be: `go build ./...`, `go test ./...`, `golangci-lint run` — CI will enforce all three.

## Pull requests

- Keep PRs focused and small; link the issue, ADR, or RFC they implement.
- PRs need one maintainer approval to merge ([GOVERNANCE.md](GOVERNANCE.md)).
- Tests accompany code. For message-format work (`iso20022/`), golden-file tests against catalog examples are required.

## Conduct

We enforce the [Code of Conduct](CODE_OF_CONDUCT.md). Payments infrastructure needs a community that institutions can trust — professionalism is part of the product.
