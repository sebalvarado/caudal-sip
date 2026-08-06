# RFCs — Requests for Comments

The process for **scheme-level** decisions: anything that changes how money moves, how llaves behave, what messages mean, or how the project is governed. Heavier than an ADR on purpose — these decisions affect every operator of the software and, potentially, real payment behavior in Ecuador.

Use an RFC for: message profile changes, llave lifecycle rules (claiming, portability, caps), settlement semantics, dispute/return flows, fee-related mechanics, North API breaking changes, governance changes.

Use an [ADR](../architecture/adr/README.md) instead for: implementation decisions inside a component.

## Process

1. Copy `0000-template.md` → `NNNN-short-title.md`, open a PR labeled `rfc`.
2. **Comment period: minimum 2 weeks.** Announce on the project channels. Substantive revisions restart the clock at 1 week.
3. Decision per [GOVERNANCE.md](../../GOVERNANCE.md): lazy consensus, maintainer vote if needed. Outcome recorded in the RFC header: `Accepted`, `Rejected`, or `Withdrawn` — rejected RFCs stay in the repo; the reasoning is the value.
4. Accepted RFCs get an implementation issue; the RFC is updated with a link when it ships.

## Open questions queue (candidate RFCs)

From [scheme-design.md](../architecture/scheme-design.md) §5 — waiting on the catálogo técnico and forum participation:

| Candidate | Topic |
|---|---|
| RFC: llave claiming & portability | Moving a phone-number llave between institutions without hijacking risk (Pix's claim flow as prior art) |
| RFC: request-to-pay | pain.013-style collection messages, not yet in the BCE catalog |
| RFC: returns & disputes | pacs.004-style returns; fraud return mechanism (Pix MED as prior art) |
| RFC: bulk disbursements | Batch-of-instant payments for payroll/G2P |
| RFC: directory anti-enumeration | Rate limits, audit, and privacy rules for llave resolution |

## Index

| # | Title | Status |
|---|---|---|
| — | (none yet) | — |
