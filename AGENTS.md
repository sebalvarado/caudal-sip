# AGENTS.md

**RPI EC / Proyecto Caudal** — open-source (Apache-2.0) implementation of Ecuador's real-time payments standard. Independent; **not affiliated with the BCE** — the disclaimer stays on every public artifact.

## Commands

- Site: `cd site && npm run dev` (:4321) · `npm run build` must pass before finishing site work.
- Go (when it lands): `go build ./... && go test ./... && golangci-lint run`.

## Before non-trivial work, read the relevant doc — don't guess

- `docs/README.md` — documentation map and reading order
- `docs/product/north-star.md` + `stakeholders.md` + `requirements.md` — direction, who we serve, what to build
- `docs/architecture/blueprint.md` + `scheme-design.md` — components, APIs, operator model
- `docs/architecture/data-model.md` + `technical-requirements.md` — entities and invariants, the non-functional bar
- `docs/development/local-stack.md` — the local dev experience we're committed to
- `docs/architecture/adr/` — decisions already made; supersede, don't relitigate
- `docs/research/catalogo-operacional.md` — 🇪🇸 the scheme spec decoded: states, codes, timing, netting, liquidity risk. Normative
- `docs/glossary.md` — vocabulary of record (_llave_, _integrante_, _red de pagos_ — use even in English)
- `docs/language-policy.md` — which language a doc gets; `docs/spanish-style.md` — mandatory for any Spanish
- `docs/rfcs/README.md` — scheme-level changes need an RFC; component decisions need an ADR

## Non-negotiables

1. Regulatory claims cite resolution + article (sources in `docs/research/landscape.md`); unknowns marked **[assumption]**. The **Catálogo Operacional is public** — states, codes, llave types, timing and netting come from `docs/research/catalogo-operacional.md`, never from invention. The catálogo _técnico_ is still missing, so don't invent field-level rules.
2. Ecuador = multi-network (redes + SIP + RPI), like Bre-B, NOT single-switch like Pix. Deadline driving the roadmap: **2026-11-30**.
3. `MANIFIESTO.md` and `site/src/pages/manifiesto.md` are mirrors — change one, change both.
4. Language follows **audience**, per `docs/language-policy.md`: Spanish for anything an Ecuadorian institution reads to do its job (catálogo, conformance, proposals, runbooks, consoles, tutorials, public copy); English for code, identifiers, commits, ADRs, RFCs, contributor docs. One canonical language per document — **never mirror**. Spanish must read native per `docs/spanish-style.md`.
5. No credentials/keys/real account data ever, even as fixtures. No blockchain framing (ADR-0006). No fees in the OSS core (GOVERNANCE.md).
6. Commits: DCO sign-off (`-s`), imperative subject, reference issue/ADR/RFC.
7. Brand config lives in `site/src/config.ts` (one file).

## Loop

Skills in `.agents/skills/` (restore with `npx skills experimental_install`): brainstorming → writing-plans → executing-plans → test-driven-development (golden files for messages) → verification-before-completion → requesting/receiving-code-review. Debug with systematic-debugging; model domain types with domain-modeling; check claims with grill-with-docs.

Verify before finishing: build passes, mirrors in sync, new docs linked from indexes, Spanish reads native.
