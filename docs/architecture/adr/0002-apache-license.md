# 0002. License under Apache-2.0

- **Status**: Accepted
- **Date**: 2026-08-05
- **Deciders**: Founding maintainer

## Context

Adopters are regulated financial institutions and their vendors; some will embed the software in commercial offerings (including our own hosted service). Institutional legal teams need a familiar license with an explicit patent grant. Precedents in this exact domain: Banco Central do Brasil's pix-api specs and the entire moov-io ecosystem are Apache-2.0.

## Decision

Apache-2.0 for all code and specs in this repository, with a NOTICE file carrying the non-affiliation disclaimer.

## Alternatives considered

- **AGPL-3.0** — would force hosted operators to open their modifications, protecting against closed forks, but scares institutional legal review and would deter the core-banking vendors we need as distribution partners.
- **MIT/BSD** — equally permissive but no patent grant; payments is patent-heavy territory.
- **BSL/fair-source** — contradicts the manifesto's public-good claim and would disqualify the project from "digital public infrastructure" funding (IDB, World Bank, DPGA).

## Consequences

Closed commercial forks are possible; we accept that and compete on community, pace, and trust. DCO sign-off required on contributions (no CLA, to keep friction low).
