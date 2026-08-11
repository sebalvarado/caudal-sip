// Package postgres implements the ledger over PostgreSQL. It is the default
// backend, shipped to every operator, per ADR-0007 and ADR-0008.
//
// The rule that governs this implementation: never UPDATE a balance column.
// Postings are inserted append-only and balances are derived. That is what
// keeps hot accounts — and at Ecuador's scale every account is hot — free of
// lock contention.
//
// A second backend over TigerBeetle follows, and both run the same
// property-based suite in CI. A divergence between them fails the build.
package postgres
