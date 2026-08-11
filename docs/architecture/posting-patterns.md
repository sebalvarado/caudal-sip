# Posting Patterns

Exactly which postings each flow produces. This is the implementation reference: a developer should be able to code the settlement service from this document plus [`ledger.md`](ledger.md), and a reviewer should be able to check an implementation against it line by line.

Accounts and invariants are in [`ledger.md`](ledger.md). Flows and states come from the Catálogo Operacional ([decoded](../research/catalogo-operacional.md) §5, §9, §10, §11.3–§11.4).

Notation: `A` is the payment amount, `F` the commission, `T` the tax on it. `ORD` is the ordering integrante's BIC, `BEN` the beneficiary's. Every entry carries the end-to-end id, the causing event id, and the hash of the wire message.

## 1. Inter-red payment, happy path

### 1.1 Reserve — at SIP validation (catálogo §11.3 step 4)

Before routing to the beneficiary red. Atomic across both ledgers; if the headroom leg fails, nothing happens and the payment is rejected.

**Ledger 2 — headroom** (pending, with timeout)

| Debit | Credit | Amount |
|---|---|---|
| `ir:ORD:headroom` | `scheme:headroom_comprometido` | `A + F + T` |

**Ledger 1 — obligations** (pending, with timeout, linked)

| Debit | Credit | Amount |
|---|---|---|
| `ir:ORD:posicion` | `ir:BEN:posicion` | `A` |
| `ir:ORD:posicion` | `scheme:comisiones_por_cobrar` | `F` |
| `ir:ORD:posicion` | `scheme:impuestos_por_pagar` | `T` |

The headroom reservation covers amount plus fees, because that is the institution's total commitment. Insufficient headroom returns `ErrInsufficientHeadroom` — the red semáforo — and the payment is rejected before it ever reaches the beneficiary red.

### 1.2 Post — on confirmation (§11.3 step 10, state `ACSP`)

Post both pending sets. Nothing new is computed; the amounts were fixed at reservation. The payment is now **irrevocable** and part of today's malla.

### 1.3 Malla close and settlement (§9, state `ACSC`)

Net `ir:*:posicion` across all participants, emit the settlement instruction to the BCE, and on confirmation close the positions:

| Debit | Credit | Amount |
|---|---|---|
| `scheme:liquidado` | `ir:X:posicion` | net debit position of X |
| `ir:Y:posicion` | `scheme:liquidado` | net credit position of Y |

Liquidation runs on the **multilateral** malla; the bilateral one is computed for reporting only (§9 rule 1). Settlement instructions are idempotent — a replayed instruction must not double-post.

### 1.4 Headroom release

On settlement, the commitment is discharged and headroom returns:

| Debit | Credit | Amount |
|---|---|---|
| `scheme:headroom_comprometido` | `ir:ORD:headroom` | `A + F + T` |

**This is the pacing mechanism of the whole system.** Under netting, an institution's headroom is only restored when a malla settles, so its capacity between settlement cycles is bounded by its ceiling. Under the gross model, settlement and release happen per transaction, so headroom recycles continuously. Same postings, different cadence — this is the difference ADR-0009 makes measurable.

## 2. Rejections and exceptions

Mapped to the catálogo's four named exceptions (§11.4).

| Case | State | Irrevocable | Ledger action |
|---|---|---|---|
| Local validation failure, before reserve | `RJCT` | No | None — nothing was reserved |
| Beneficiary rejects (`AC04`, `AC01`, `AC06`, `AC05`, `AC07`) | `RJCT` | No | **Void** both pending sets |
| Insufficient headroom — the reserve itself fails | `RJCT` / `AM04` | No | Nothing was reserved |
| Exception 1 — ordering red cannot reach the SIP | `RJCT` | No | Void, if a reservation was created |
| Exception 2 — SIP cannot deliver to the beneficiary red | `RJCT` | No | Void |
| Exception 3 — beneficiary red cannot return confirmation | `ACSP` | **Yes** | **Post**, then move to suspense (below) |
| Exception 4 — SIP cannot return confirmation to the ordering red | `ACSC` | **Yes** | Post normally; the malla is generated |

Voiding releases headroom and cancels the obligations in one atomic operation. The reservation timeout is a safety net, not the primary mechanism — an expired reservation that was never posted or voided is an incident to investigate, not a routine outcome.

### 2.1 Suspense — `por conciliar` (exception 3)

The payment is irrevocable but the outcome is unknown, so the obligation stands while the operational item is tracked:

| Debit | Credit | Amount |
|---|---|---|
| `scheme:suspenso` | `scheme:suspenso` | *(memo entry, zero net)* |

The suspense account carries the open item by reference rather than by value, since the money is already correctly represented in ledger 1. Every balance line here is a payment that **must be resolved the same day by 18:00** (§13.2). The reconciliation console works this queue.

Resolution posts nothing further if the payment succeeded — it was already posted at `ACSP` — and if it turns out to have failed, it produces a **reversal** (section 4), never a deletion.

## 3. Headroom funding

### 3.1 Weekly recalculation (§10)

The nivel de riesgo aceptable is recomputed weekly and applies the following week.

**Increase:**

| Debit | Credit | Amount |
|---|---|---|
| `scheme:headroom_otorgado` | `ir:X:headroom` | increase |

**Decrease** — the case that needs care. The debit cannot take headroom below zero:

| Debit | Credit | Amount |
|---|---|---|
| `ir:X:headroom` | `scheme:headroom_otorgado` | `min(decrease, available balance)` |

Any remainder is recorded as a **pending reduction** and applied as existing commitments clear. Existing commitments always stand; only new ones are blocked. This must have a dedicated test — it is the one place where the invariant and the business rule pull against each other.

### 3.2 Manual block and re-enablement

An automatic block is simply headroom reaching zero; no special posting exists or is needed. Re-enablement is manual per the catálogo and is an explicit funding entry by the administrator, recorded with the operator's identity in the audit log. **The funding service never auto-credits headroom after a block.**

## 4. Corrections and reversals

Nothing is ever updated or deleted. A correction is a new entry that mirrors the original with debits and credits exchanged, references the original entry id, and carries a reason code plus the authorizing operator. Reversals of settled positions post against `scheme:liquidado`, never against a closed malla.

## 5. Worked example

`A = 100.00`, `F = 0.25`, `T = 0.0375` (15% IVA on the commission), ordering `PICHECEQ500`, beneficiary `CJEPECEQ`.

**Reserve** — headroom: debit `ir:PICHECEQ500:headroom` 100.2875, credit `scheme:headroom_comprometido` 100.2875. Obligations, linked: debit `ir:PICHECEQ500:posicion` 100.00 / credit `ir:CJEPECEQ:posicion` 100.00; debit `ir:PICHECEQ500:posicion` 0.25 / credit `scheme:comisiones_por_cobrar` 0.25; debit `ir:PICHECEQ500:posicion` 0.0375 / credit `scheme:impuestos_por_pagar` 0.0375.

**Post at `ACSP`** — the same amounts become posted. Pichincha's position now shows 100.2875 owed; JEP is owed 100.00.

**Malla close** — netted against every other obligation in the cycle, settled at the BCE, positions closed against `scheme:liquidado`, and 100.2875 of headroom returns to Pichincha.

Rounding is decided once, in the tariff module, and the tax leg is computed from the quoted commission — never recomputed downstream, or the entry will not balance.

## 6. What must never appear

No posting without a causing event. No entry that references a payment not in the event log. No balance written directly. No fee charged that was not quoted ([ADR-0010](adr/0010-commissions-as-postings.md)). No account number, document number, or customer name in any posting — the ledger holds BICs, amounts, and references, and personal data stays in the payment record where access is controlled.
