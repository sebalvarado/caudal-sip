# Governance

This document describes how decisions are made. It is deliberately lightweight for the project's current stage and will evolve as the community grows (each change to it via pull request + RFC).

## Current phase: founder-led

The project is bootstrapped by its founding maintainer(s), who hold final decision authority while the contributor base forms. The path away from this model is written down now, on purpose:

1. **Phase 1 (now)** — founder-led. Decisions in public: every non-trivial decision is recorded as an [ADR](docs/architecture/adr/README.md) or debated as an [RFC](docs/rfcs/README.md), even when the founder decides.
2. **Phase 2** — maintainer team. Contributors with sustained, high-quality contributions are invited as maintainers with merge rights. Decisions by lazy consensus among maintainers; RFCs decided by maintainer vote when consensus fails.
3. **Phase 3** — if the project achieves institutional adoption, stewardship moves to a neutral entity (foundation or multi-stakeholder association) so that no single company — including any commercial entity founded by the maintainers — controls the scheme implementation. This mirrors the NPCI/Mojaloop lesson: national infrastructure needs neutral governance to be adoptable.

## Decision types

| Type | Mechanism | Examples |
|---|---|---|
| Routine change | Pull request + one maintainer approval | Bug fix, docs, refactor |
| Component-level technical decision | ADR merged via PR | Storage backend, library choice, API shape within a component |
| Scheme-level or cross-cutting decision | RFC with public comment period (min. 2 weeks) | Message profile changes, llave rules, settlement semantics, governance changes |
| Security | Private advisory process ([SECURITY.md](SECURITY.md)) | Vulnerabilities and fixes |

## Conflicts of interest

Maintainers must disclose affiliations with payment networks, financial institutions, vendors, or any commercial operator of this software. Disclosure lives in `MAINTAINERS.md` alongside each maintainer entry. A maintainer with a direct commercial interest in an RFC's outcome may participate in discussion but should abstain from casting the deciding vote.

## Relationship with the regulator

If the Banco Central del Ecuador or another public body wishes to participate, they are welcome under the same public process as everyone else — issues, RFCs, and PRs. The project does not accept private feature agreements that bypass the public process; that guarantee is what makes the project trustworthy to *all* participants.
