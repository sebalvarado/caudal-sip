# 0004. Two API surfaces: North (ours, simple) and South (mandated, faithful)

- **Status**: Accepted
- **Date**: 2026-08-05
- **Deciders**: Founding maintainer

## Context

The scheme's wire protocol (signed ISO 20022 over mTLS) is mandated and non-negotiable, but it is hostile to the average core-banking integration team. Adoption by ~400 institutions depends on integration cost. Mojaloop solved this with a "scheme adapter"; Pix left it to each PSP (spawning a market of proprietary connectors).

## Decision

The `gateway` exposes two asymmetric surfaces:

- **North API** (institution-facing, ours to design): minimal JSON REST + webhooks — payments, llaves, QR, reconciliation — with idempotency keys, generated SDKs, and sandbox parity. Target: a core-banking vendor integrates in days without touching XML.
- **South API** (scheme-facing): a faithful implementation of the BCE messaging over mTLS with detached signatures, exactly per catálogo. No creative deviations, ever — conformance is the product.

The North API contract lives in `spec/` as versioned OpenAPI and is treated as a public product with semver discipline.

## Alternatives considered

- **Expose ISO 20022 directly to institutions** — pushes the hardest part onto the least-resourced adopters; kills the adoption thesis.
- **gRPC-only North API** — better typing, but the core-banking world (Java shops, older stacks) integrates REST+webhooks far more readily. gRPC can be added alongside later.

## Consequences

Easier: adoption, tutorials, hosted multi-tenant operation. Harder: we own a translation layer where subtle bugs can live — mitigated by the TCK exercising both surfaces end-to-end. Committed: North API backward compatibility within a major version.
