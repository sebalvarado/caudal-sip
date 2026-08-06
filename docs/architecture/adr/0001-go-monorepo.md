# 0001. Build in Go, in a single monorepo

- **Status**: Accepted
- **Date**: 2026-08-05
- **Deciders**: Founding maintainer

## Context

The primary adopters are Ecuadorian financial institutions — especially cooperativas with small IT teams — and their core-banking vendors. Deployment simplicity determines adoption more than architectural elegance. Mojaloop's practical adoption friction (a large Node.js/Kubernetes hub) is the cautionary example; moov-io's Go single-binary DX is the positive one. The project also needs docs, specs, website, and multiple services to evolve together as a single source of truth.

## Decision

All backend components are written in Go and live in this single repository (`iso20022/`, `llaves/`, `switch/`, `settlement/`, `gateway/`, `qr/`, `tck/`, `sandbox/`), alongside `docs/`, `spec/`, and `site/`. Each component ships as a static binary and a Docker image.

## Alternatives considered

- **Node.js/TypeScript (Mojaloop's stack)** — larger runtime footprint, heavier ops burden for small institutions; weaker fit for the single-binary distribution model.
- **Rust** — excellent runtime properties but a far smaller hiring/contributor pool in the target community.
- **Multi-repo (moov-io style)** — better per-component versioning, but fragments docs/specs and multiplies maintenance while the team is small. Can split later if needed.

## Consequences

Easier: onboarding ("clone one repo"), cross-component refactors, keeping specs/docs/code in sync. Harder: independent release cadences per component (mitigated with Go module tagging conventions later). Committed: Go toolchain in CI, `go test ./...` green on `main`.
