// Package risk is a funding service, not a gatekeeper.
//
// The Catálogo Operacional's nivel de riesgo aceptable is modeled as a funded
// control account with a no-overdraft constraint (ADR-0009), so the automatic
// block at 100% is a refusal by the storage engine rather than logic in the
// payment path that a new code path could forget to call. What remains here is
// the funding: ingest reserve requirements, poll the BCE current-account
// balance on the catálogo's 30-minute cadence, recompute ceilings weekly
// (Thursday to Wednesday, following the SBA cycle), credit headroom accounts,
// and emit warnings well before the threshold.
//
// The case that needs care is a ceiling decrease below already-committed
// value: the debit cannot take headroom negative, so only the available
// balance is debited and the remainder applies as commitments clear. Existing
// commitments always stand; only new ones are blocked.
//
// Re-enablement after a block is manual, per the catálogo. This service never
// auto-credits headroom back.
package risk
