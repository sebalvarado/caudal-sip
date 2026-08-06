# 0005. Reference settlement model: RTGS-style gross, no overdraft; DNS as a simulation mode

- **Status**: Accepted
- **Date**: 2026-08-05
- **Deciders**: Founding maintainer

## Context

Ecuador's resolutions point to real-time clearing with settlement through the BCE's Cámara de Compensación Especializada — likely deferred net settlement (DNS), as with BANRED today. International best practice (Pix's Contas PI, Bre-B's MOL, both prompted by IMF/FSAP warnings about credit risk in proprietary/net settlement) is gross real-time settlement in central bank money with prefunded, no-overdraft accounts.

## Decision

`settlement/` implements **RTGS-style gross settlement with prefunded accounts and no overdraft as the reference model**: strict double-entry, immutable journal, per-transaction finality events. It also implements a **DNS mode** (configurable cycles, net positions, exposure tracking) so the sandbox can simulate Ecuador's likely production arrangement and *quantify* the credit-exposure difference between the two models.

## Alternatives considered

- **DNS-only (mirror the likely BCE arrangement)** — faithful to today's Ecuador, but bakes in the model international reviewers flag as risky, and forfeits our strongest evidence-based advocacy position in the interoperability forums.
- **RTGS-only** — cleaner, but then the sandbox can't simulate the actual national arrangement, weakening the TCK's realism.

## Consequences

Easier: demonstrating risk trade-offs with running code; adapting whichever way the BCE's final rules land. Harder: two settlement modes to test and maintain. Committed: ledger invariants (no negative balances in RTGS mode, journal immutability, idempotent settlement instructions) enforced by property-based tests.
