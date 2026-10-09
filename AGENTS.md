# AGENTS.md

**RPI EC / Proyecto Caudal** — open-source (Apache-2.0) implementation of Ecuador's real-time payments standard. Independent; **not affiliated with the BCE** — the disclaimer stays on every public artifact.

## Commands

`just` is the command surface (`just` lists recipes; `just doctor` checks tools). No Turborepo: Cargo is the build graph.

- `just check <crate>` / `just test <crate>` — the fast loop · `just ci` — the done-gate, once before a PR
- `just db` — this worktree's own Postgres database; `DATABASE_URL` is set per worktree
- `just site` (:4321) · `just site-build` must pass before finishing site or `docs/` work

## Before non-trivial work, read the relevant doc — don't guess

- `docs/README.md` — documentation map and reading order
- `docs/product/north-star.md` + `stakeholders.md` + `requirements.md` — direction, who we serve, what to build
- `docs/architecture/blueprint.md` + `scheme-design.md` — components, APIs, operator model
- `docs/architecture/data-model.md` + `technical-requirements.md` — entities and invariants, the non-functional bar
- `docs/development/local-stack.md` — the local dev experience we're committed to
- `docs/architecture/adr/` — decisions already made; supersede, don't relitigate
- `docs/research/catalogo-operacional.md` — 🇪🇸 the scheme spec decoded: states, codes, timing, netting, liquidity risk. Normative
- `research/bce/README.md` — primary BCE sources: agents search the `.md` transcription first, then verify the PDF before making or citing regulatory claims
- `docs/glossary.md` — vocabulary of record (_llave_, _integrante_, _red de pagos_ — use even in English)
- `docs/language-policy.md` — which language a doc gets; `docs/spanish-style.md` — mandatory for any Spanish
- `docs/rfcs/README.md` — scheme-level changes need an RFC; component decisions need an ADR
- `workstream/authorized-red/` — the business/licensing track to become a BCE-authorized red de pagos: plan, authorization dossier checklists, legal open questions, commercial pipeline. Start at its README; never commit signed contracts or personal data there

## Non-negotiables

1. Regulatory claims cite resolution + article (sources in `research/bce/` and `docs/research/landscape.md`); unknowns marked **[assumption]**. The **Catálogo Operacional is public** — states, codes, llave types, timing and netting come from `docs/research/catalogo-operacional.md`, never from invention. The catálogo _técnico_ is still missing, so don't invent field-level rules.
2. Ecuador = multi-network (redes + SIP + RPI), like Bre-B, NOT single-switch like Pix. Deadline driving the roadmap: **2026-11-30**.
3. `MANIFIESTO.md` and `site/src/pages/manifiesto.md` are mirrors — change one, change both.
4. Language follows **audience**, per `docs/language-policy.md`: Spanish for anything an Ecuadorian institution reads to do its job (catálogo, conformance, proposals, runbooks, consoles, tutorials, public copy); English for code, identifiers, commits, ADRs, RFCs, contributor docs. One canonical language per document — **never mirror**. Spanish must read native per `docs/spanish-style.md`.
5. No credentials/keys/real account data ever, even as fixtures. No blockchain framing (ADR-0006). No fees in the OSS core (GOVERNANCE.md).
6. Commits: DCO sign-off (`-s`), imperative subject, reference issue/ADR/RFC.
7. Brand config lives in `site/src/config.ts` (one file).

## Loop

Work is planned in [`docs/plan/epics.md`](docs/plan/epics.md) and tracked as GitHub issues against the `2026-11-30` milestone. One issue, one `wt` worktree, one PR; CI green before review. Write the failing test first — golden files for messages, property tests for ledger invariants.

**Decisions before code.** Issues labelled `blocked` are not started. No implementation begins until the Domain foundations epic (#17) is merged: crate map, kernel types, ledger trait, data ownership, time and identifiers, composition. Each crate's `AGENTS.md` says what it owns and must never depend on; Cargo dependencies enforce it.

Process is Caudal-owned. Do not follow Superpowers or other generic brainstorm / plan / TDD / worktree stacks.

| Skill (`.agents/skills/`) | When to use |
|---|---|
| `worktree-management` | Starting an issue, handing off to a subagent, cleaning up merged branches |
| `self-verification` | After every change and before claiming done |
| `domain-modeling` | Pinning terminology or recording a decision during Domain foundations |
| `grill-with-docs` | Checking a claim against the docs before relying on it |
| `caudal-diagrams` | Any diagram |
| `authorized-red` | The licensing track |

Third-party skills are restored with `npx skills experimental_install`.

Verify before finishing: build passes, mirrors in sync, new docs linked from indexes, Spanish reads native.
