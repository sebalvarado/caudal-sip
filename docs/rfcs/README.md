# RFCs — Requests for Comments

The process for **scheme-level** decisions: anything that changes how money moves, how llaves behave, what messages mean, or how the project is governed. Heavier than an ADR on purpose — these decisions affect every operator of the software and, potentially, real payment behavior in Ecuador.

Use an RFC for: message profile changes, llave lifecycle rules (claiming, portability, caps), settlement semantics, dispute/return flows, fee-related mechanics, North API breaking changes, governance changes.

Use an [ADR](../architecture/adr/README.md) instead for: implementation decisions inside a component.

## Process

1. Copy `0000-template.md` → `NNNN-short-title.md`, open a PR labeled `rfc`.
2. **Comment period: minimum 2 weeks.** Announce on the project channels. Substantive revisions restart the clock at 1 week.
3. Decision per [GOVERNANCE.md](../../GOVERNANCE.md): lazy consensus, maintainer vote if needed. Outcome recorded in the RFC header: `Accepted`, `Rejected`, or `Withdrawn` — rejected RFCs stay in the repo; the reasoning is the value.
4. Accepted RFCs get an implementation issue; the RFC is updated with a link when it ships.

## Language

RFCs split by audience, per [language-policy.md](../language-policy.md). A **scheme-change proposal aimed at the BCE, the redes, or Ecuadorian institutions is written in Spanish** with a short English abstract — those readers are the ones blocked without it. RFCs about this project's own governance or internal mechanics stay in English.

## Observations register

Findings from reading the norm that are candidates for an RFC, a question to the BCE, or both. Keeping them in one place turns scattered notes into a systematic contribution — and a rejected observation is as useful as an accepted one, provided the reasoning is written down.

| Observation | Type | Source |
|---|---|---|
| **Settlement is not 24/7 although the system declares itself 24/7** | **RFC [0001](0001-liquidacion-continua.md)** | Catálogo §2.1, §5.8–9, §9.4 |
| `PDNG` is named "por conciliar" but defined as next-day deferral after cut-off — so the irrevocable-but-unknown state may have no code at all | Question to the BCE | Catálogo Tabla 11 |
| `pacs.008.001.08` (§8) vs `pacs.008.004.08` (§11.3) version inconsistency | Question to the BCE | Catálogo §8, §11.3 |
| Risk table leaves reserve and risk factor **blank for cooperative segments 4 and 5** | Question to the BCE | Catálogo Tabla 19 |
| Llave portability only exists as cancel-and-re-register: no claim window, no confirmation by the losing institution | RFC candidate | Catálogo §11.1 |
| No returns or disputes mechanism of any kind (no `pacs.004` equivalent, no fraud reversal) | RFC candidate | Absent from the catálogo |
| Full customer identity travels in every `pacs.008` — data minimization | RFC candidate | Catálogo §8 |
| No bulk payment construct; payroll is N individual messages | RFC candidate | Absent from the catálogo |
| No request-to-pay (`pain.013`-style collection) | RFC candidate | Absent from the catálogo |
| Remittance data too thin for automatic invoice reconciliation (B2B adoption blocker) | RFC candidate | Catálogo §8 |
| Timeout values are all "parametrizable" with no published values | Question to the BCE | Catálogo §5 |
| Directory anti-enumeration: rate limits, audit and privacy rules for llave resolution | RFC candidate | Absent from the catálogo |
| Commission policy: is free P2P viable within this design? | RFC candidate | Catálogo §2.1.18, §9.5 |

## Index

| # | Title | Status |
|---|---|---|
| [0001](0001-liquidacion-continua.md) | Liquidación continua para un sistema que ya se declara 24/7 🇪🇸 | Draft |
