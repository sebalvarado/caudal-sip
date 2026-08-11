# Scheme Design: APIs, Participants, Keys, and Settlement

Answers to the core design questions, anchored in what the BCE regulations already mandate (BCE-GG-008-2025, BCE-GG-007-2026) and in Pix/Bre-B/UPI precedent where Ecuador's norm is silent. Where the norm is silent, our position is marked **[proposal]**.

> **Partially superseded (2026-08-05).** The Catálogo Operacional turned out to be public, and it answers several questions this document treated as open — payment states, reason codes, llave types, timing, netting, and a liquidity-risk engine we had not modeled. Read [`../research/catalogo-operacional.md`](../research/catalogo-operacional.md) alongside this file; where they disagree, the catálogo wins. The sections below carry inline notes where that applies.

## 1. The integration API (what parties actually code against)

Two API surfaces, deliberately asymmetric — this is the single most important product decision:

### North API (institution-facing) — ours to design

What a bank/cooperativa/fintech integrates with the `gateway`. Simple, JSON, webhook-driven, no ISO 20022 knowledge required:

```
POST /v1/payments                  initiate an outgoing payment (by llave or by account+BIC)
GET  /v1/payments/{id}             status (accepted / rejected / settled / irrevocable)
POST /v1/llaves                    register a key for a customer account
DELETE /v1/llaves/{llave}          remove / unlink
POST /v1/llaves/{llave}/resolve    resolve a key -> credencial de pago (pre-payment lookup)
POST /v1/qr/decode | /v1/qr/encode EMV QRCPS with embedded llave
Webhooks: payment.received, payment.settled, payment.rejected, llave.claimed, ...
```

Design contract: **a cooperativa's core-banking vendor should integrate in days.** Idempotency keys on every mutation, sandbox parity with production, generated SDKs (Go, TypeScript, Java for the core-banking world). This layer is our Medusa-like DX moat — the scheme's wire protocol is mandated, but the developer experience is not, and that's where adoption is won.

### South API (scheme-facing) — mandated by the BCE, ours to implement faithfully

Signed ISO 20022 over mTLS, exactly per the catálogo técnico/operacional:

- `pacs.008` — payment order (integrante ordenante → red → [SIP → red beneficiaria] → integrante beneficiario)
- `pacs.002` — accept/reject response and the irrevocability notification (emitted by the red for intra-network, by the SIP for inter-network)
- `acmt.*` — llave resolution: query returns the **credencial de pago** (see §3)

The gateway translates North↔South. Institutions never touch XML unless they want to.

## 2. Participants and use cases

### Direct and indirect — both, from day one

The BCE norm itself contemplates both: integrantes act "con su infraestructura tecnológica de manera directa **o la provista por un tercero, de forma indirecta**" (BCE-GG-007-2026, def. 6). Concretely:

- **Direct participant**: runs its own gateway instance, holds its own BIC, its own mTLS certs and signing keys, connects to its red de pagos itself.
- **Indirect participant**: rides on infrastructure provided by a third party — a caja central, a core-banking vendor (the multiplier for 365 cooperativas), or an authorized Sistema Auxiliar de Pagos processor. **[proposal]** The gateway ships with a **multi-tenant mode**: one deployment, N institutions, per-tenant BICs, keys, and isolation. This single feature is what makes the long tail reachable — a federation or vendor deploys once and onboards dozens of cooperativas.
- Fintechs/SEDPEs: participate per the norm as integrantes if licensed, otherwise as clients of a participant. We don't invent access categories the regulation doesn't have — but our tooling makes the licensed path cheap.

### Use cases (phased, matching the regulation's scope: transfers up to USD 15,000)

| Phase | Use case | Notes |
|---|---|---|
| 1 | **P2P by llave** | The adoption engine everywhere (Pix, Bre-B, UPI). Note the catálogo assumes **priced** transactions: commissions are parameterized in the SIP, disclosed to the payer with taxes before confirmation, and `ChrgBr` defaults to `DEBT`. Free P2P is therefore an argument against a specified default, and needs evidence rather than a slogan. |
| 1 | **P2M via QR** | Static QR (merchant's llave) first; dynamic QR (per-transaction amount + reference) close behind. BCE QR standard is mandated. |
| 2 | **B2B / payroll** | Same rail, business-initiated, batch-of-instant disbursements ("pagos masivos" — Bre-B's next frontier). |
| 2 | **Request-to-pay / cobros** | Needs pain.013-style messaging; not yet in the BCE catalog — an RFC we can lead in the forums. |
| 3 | **G2P (bono disbursements, tax refunds)** | The government-adoption hook: Decreto 1553 already forces state institutions to use SPI; instant G2P is the modern successor. |

## 3. Llaves: multiplicity, uniqueness, and account mapping

The BCE defines a llave as an "identificador único y público que permite al cliente vincular sus credenciales de pago" and the **credencial de pago** as: beneficiary's cédula/RUC/passport + account number + account type + institution BIC (BCE-GG-007-2026, defs. 5 and 10).

Rules (mandated + proposed):

1. **One llave → exactly one account.** Uniqueness is system-wide (same rule as Pix and Bre-B: an alias resolves to exactly one payment instrument). This is what makes "send to my number" unambiguous.
2. **One user → many llaves.** A user can hold multiple keys of different types — phone, cédula, email, random/alphanumeric — each independently pointed at whichever account they choose, across different institutions. **[proposal]** Cap per account per institution (Pix uses 5 for individuals, 20 for businesses) to limit directory abuse.
3. **Mapping back to the preferred account is the directory's whole job**: payer enters a llave → ordering institution sends an `acmt` query → directory (red's distributed directory intra-network; SIP for inter-network) returns the credencial de pago → payer confirms the displayed beneficiary name (crucial anti-error/anti-fraud UX from Pix/Bre-B) → `pacs.008` carries the credencial → **the beneficiary institution validates that the credencial matches its client and rejects on mismatch** (Art. 10, BCE-GG-007-2026). The account choice therefore lives with the *receiving* customer, set at key-registration time, changeable any time (re-point the llave, no coordination with payers needed).
4. **Portability and claiming** — **now a documented gap, not a guess.** The catálogo specifies only cancel-then-re-register (§11.1), with no claim window and no confirmation through the losing institution; the bulk migration process (§4) is a BCE-run administrative operation performed outside operating hours. Our Pix-style claim flow **[proposal]** is therefore the strongest RFC candidate we have, and it addresses a real key-hijacking exposure.
5. **Privacy**: resolution responses expose minimum data (masked name for confirmation, not full identity). Directory queries are rate-limited and audited to prevent enumeration/scraping (a real Pix DICT lesson).

## 4. Settlement: how money actually moves

The layered answer, per the regulations:

- **Money layer**: Ecuador is dollarized; all participant institutions hold accounts at the BCE. Final settlement is a book transfer **in central bank money at the BCE** — same terminal rail as today's SPI. No new money rail is created; the innovation is speed and orchestration of clearing.
- **Intra-network** (payer and payee in the same red): the red clears in real time — validates, routes, obtains the beneficiary's accept (`pacs.002`), declares irrevocability — and the red's positions are settled at the BCE through the **Cámara de Compensación Especializada** per its specifications (Art. 9, BCE-GG-008-2025). Today that CCE cycle is periodic (BANRED settles once daily), meaning **deferred net settlement**: the end user experiences instant finality while institutions carry intraday bilateral exposure.
- **Inter-network**: the red ordenante forwards to the **SIP**, which resolves, forwards to the red beneficiaria, "procesa y compensa" and **remits settlement information** to the BCE settlement systems. Irrevocability for inter-network payments is declared by the SIP via `pacs.002` (Art. 12, BCE-GG-007-2026).
- **User-visible finality vs. interbank finality are different events.** The user sees irrevocable credit in seconds (the beneficiary institution credits immediately upon irrevocability); interbank positions settle at the BCE per the clearing cycle. Pix eliminated the gap entirely (pure RTGS on Contas PI, no overdraft); Bre-B's MOL likewise settles gross in real time. Ecuador's texts point to clearing-then-settlement instead.
- **Confirmed by the catálogo (§9, §10)**: settlement is deferred net on a **multilateral malla**, liquidated several times a day in the integrantes' BCE accounts, with the bilateral malla kept for reporting and out-of-hours payments settling the next business day. The BCE contains the resulting credit risk with a **liquidity-risk engine**: a weekly per-institution ceiling derived from its reserve requirement, a traffic-light dashboard, and automatic blocking of an institution's outgoing payments when it crosses the ceiling — re-enablement being manual.
- **Our position [advocacy], revised**: the comparison worth publishing is no longer "gross versus net" in the abstract. It is a quantified study of *this* design: what happens to a segment-3 cooperativa with a 100% risk level on payday, how responsive a weekly recalculation is to an intraday spike, and what an institution should do operationally as it approaches red. Our `settlement/` module implements gross real-time settlement as the reference model and simulates the Ecuadorian deferred-net-plus-risk-ceiling design, so the difference can be measured rather than asserted.

## 5. Open questions to resolve in the forums (our RFC pipeline)

Revised against the catálogo — the full list with detail is in [`../research/catalogo-operacional.md`](../research/catalogo-operacional.md) §12.

1. **Answered**: settlement cadence (multilateral malla, several intraday liquidations) and the guarantee mechanism (reserve-derived risk ceiling with automatic blocking), §9–§10.
2. Key claiming and portability — still only cancel-and-re-register. Our top RFC candidate.
3. Request-to-pay and bulk-disbursement messaging — still absent.
4. Dispute and return flows — no `pacs.004` analog or fraud-return mechanism appears anywhere in the catálogo.
5. Fee policy — now an argument against a specified default (see the use-case table above).
6. **Partly answered**: confirmation-of-payee display and the 60-second and 20-second windows are mandated (§5); a common consumer brand is still absent.
7. **New**: the actual per-stage timeout values, which the catálogo leaves parameterized and unpublished.
8. **New**: data minimization — full customer identity travels in every `pacs.008`.
