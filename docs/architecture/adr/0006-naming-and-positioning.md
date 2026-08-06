# 0006. Naming and positioning: RPI EC (standard) + Caudal (brand); complement, not replacement; no blockchain framing

- **Status**: Accepted
- **Date**: 2026-08-05
- **Deciders**: Founding maintainer

## Context

The project needs (a) a name that signals it implements the national standard, (b) a brand that can carry a public movement, and (c) a positioning that keeps the BCE as a potential adopter rather than an antagonist. "RPI" is the BCE's own infrastructure name, which carries both signaling value and impersonation risk.

## Decision

- **RPI EC** is the standard/compliance name (what the software implements).
- **Caudal / Proyecto Caudal** is the project brand (a river's flow; in Spanish, *caudal* also means funds/wealth). Brand identity is centralized in `site/src/config.ts`.
- Every public artifact carries the non-affiliation disclaimer (see `NOTICE`).
- Positioning is **complement to the BCE, never replacement**: participant-side tooling, sandbox, education first; the scheme-core profile is offered as reference, with BCB's publication of Pix specs on GitHub as the cited precedent.
- **No blockchain/crypto framing anywhere.** Settlement is central bank money; credibility with regulators and institutions depends on saying so unambiguously.

## Alternatives considered

- **Chimborazo** (mountain codename) — strong story, but the river/flow metaphor maps directly to payments and *caudal*'s double meaning is unbeatable in Spanish.
- **Only "RPI EC", no brand** — descriptive but can't carry a movement, and increases confusion-with-official-infrastructure risk rather than reducing it.
- **Adversarial positioning ("the BCE is too slow, use ours")** — emotionally satisfying, strategically fatal.

## Consequences

Committed: trademark check for "Caudal" (SENADI, financial services classes) before official launch; disclaimer maintenance in site footer, manifesto, NOTICE, and README; renaming remains cheap (one config file) if the check fails.
