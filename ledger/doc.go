// Package ledger is the scheme's obligation ledger: the irrevocable
// inter-red obligations that accumulate into mallas until they settle.
//
// It does not own settlement in central bank money (the integrantes' current
// accounts at the BCE) and it does not own customer balances (each
// institution's core banking). Those are external systems this one produces
// instructions for and reflects, respectively.
//
// Two ledgers live here and transfers never cross between them: obligations,
// denominated in USD, and headroom, which is USD-denominated but is not money
// — it is each integrante's remaining capacity to commit outgoing payments.
// Keeping them apart is what makes the liquidity ceiling an invariant enforced
// by the storage engine rather than a rule the application has to remember.
//
// Design: docs/architecture/ledger.md
// Postings per flow: docs/architecture/posting-patterns.md
// Decisions: ADR-0008 (architecture), ADR-0009 (risk ceiling as a funded account)
package ledger
