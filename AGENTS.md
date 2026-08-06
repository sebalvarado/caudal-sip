# AGENTS.md

**RPI EC / Proyecto Caudal** — open-source (Apache-2.0) implementation of Ecuador's real-time payments standard. Independent; **not affiliated with the BCE** — the disclaimer stays on every public artifact.

**Status**: docs + website only. Go components designed but not scaffolded. Not yet a git repo.

## Commands

- Site: `cd site && npm run dev` (:4321) · `npm run build` must pass before finishing site work.
- Go (when it lands): `go build ./... && go test ./... && golangci-lint run`.

## Before non-trivial work, read the relevant doc — don't guess

- `docs/README.md` — documentation map and reading order
- `docs/architecture/blueprint.md` + `scheme-design.md` — components, APIs, operator model
- `docs/architecture/adr/` — decisions already made; supersede, don't relitigate
- `docs/glossary.md` — vocabulary of record (*llave*, *integrante*, *red de pagos* — use even in English)
- `docs/spanish-style.md` — mandatory for any public-facing Spanish
- `docs/rfcs/README.md` — scheme-level changes need an RFC; component decisions need an ADR

## Non-negotiables

1. Regulatory claims cite resolution + article (sources in `docs/research/landscape.md`); unknowns marked **[assumption]**. We lack the BCE catálogo técnico — don't invent message fields.
2. Ecuador = multi-network (redes + SIP + RPI), like Bre-B, NOT single-switch like Pix. Deadline driving the roadmap: **2026-11-30**.
3. `MANIFIESTO.md` and `site/src/pages/manifiesto.md` are mirrors — change one, change both.
4. English for code/docs/commits; Spanish for public copy, native-sounding per `docs/spanish-style.md`.
5. No credentials/keys/real account data ever, even as fixtures. No blockchain framing (ADR-0006). No fees in the OSS core (GOVERNANCE.md).
6. Commits: DCO sign-off (`-s`), imperative subject, reference issue/ADR/RFC.
7. Brand config lives in `site/src/config.ts` (one file).

## Loop

Skills in `.agents/skills/` (restore with `npx skills experimental_install`): brainstorming → writing-plans → executing-plans → test-driven-development (golden files for messages) → verification-before-completion → requesting/receiving-code-review. Debug with systematic-debugging; model domain types with domain-modeling; check claims with grill-with-docs.

Verify before finishing: build passes, mirrors in sync, new docs linked from indexes, Spanish reads native.
