// Package tariff owns fee schedules: effective-dated, versioned, and
// changeable by an administrator without a deployment.
//
// It quotes a commission and its tax before the customer confirms, because the
// Catálogo Operacional requires the payer to see the amount with taxes first.
// The quote is snapshotted onto the payment, so a charge is always explained
// by the schedule in force when it was made, however long afterwards someone
// asks. Nothing downstream ever recomputes a fee.
//
// This package does not post to the ledger and holds no balances. It produces
// the fee legs that the ledger posts atomically with the payment.
//
// Design: docs/architecture/tariff.md
// Decision: ADR-0010 (commissions as first-class postings)
package tariff
