# 0003. ISO 20022 XML on the wire; profile-driven message library

- **Status**: Accepted
- **Date**: 2026-08-05
- **Deciders**: Founding maintainer

> The profile-driven XML decision below is unchanged. Wording that binds the library to Go types, or to Go's XML namespace behavior via moov-io, yields to [ADR-0011](0011-rust-monorepo.md): codecs, golden files, and fuzzing are Rust. moov-io remains prior art for the namespace problems, not the implementation.

## Context

BCE-GG-007-2026 mandates ISO 20022 messages (pacs.008, pacs.002, acmt family) whose exact fields and validations live in the BCE's catálogo técnico y operacional — which we do not yet possess. We must start building without it, and remain correct when we obtain it. Reference systems differ: Pix uses ISO 20022 XML; Mojaloop uses an ISO 20022-inspired JSON profile.

## Decision

`iso20022/` implements a **generic ISO 20022 core** (types, XML codecs, validation engine) plus **country/scheme profiles** as data: field presence, formats, and business rules defined per profile. The Ecuador profile starts from the generic XSDs plus what the resolutions state, with unknowns marked `[assumption]`, and is corrected when the catálogo is obtained. Wire format is XML (per mandate); JSON mappings exist for internal APIs, debugging, and documentation.

## Alternatives considered

- **Wait for the catálogo** — loses months; the library's core (codecs, validation engine) is catalog-independent anyway.
- **Hardcode Ecuador's rules into Go types** — faster initially, but brittle against catalog revisions and blocks reuse for other LatAm profiles (Bre-B, future schemes) that make the library attractive to a wider contributor base.
- **JSON-first like Mojaloop** — diverges from the Ecuadorian mandate; translation layers to XML would become the riskiest code in the system.

## Consequences

Easier: adapting to catalog changes (data, not code), supporting multiple schemes, testing (golden files per profile). Harder: a validation engine is more work than hardcoded structs. Committed: fuzzing on parsers, golden-file test suites, documented Go XML namespace handling (see moov-io/rtp20022's notes on Go's XML limitations).
