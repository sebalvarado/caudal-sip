# Glossary / Glosario

The vocabulary of record for this project. Spanish terms from Ecuadorian regulation are **used as-is in all languages** — write *llave*, not "alias key". Where a term comes from a specific resolution, it's cited.

Términos del ecosistema ecuatoriano de pagos, en ambos idiomas. Los términos regulatorios en español se usan tal cual en todos los idiomas del proyecto.

## Ecuador's scheme (regulatory terms)

| Término (ES) | English | Definition |
|---|---|---|
| **Llave de pago** | payment key | Public, unique identifier that a customer links to one account to send/receive payments. Four types only: phone `0001`, identification number `0002` (cédula or RUC — **not passport**), email `0003`, alphanumeric `0004`. One llave → exactly one account. (BCE-GG-007-2026, def. 10; Catálogo Operacional §3) |
| **Credencial de pago** | payment credential | The data a llave resolves to: beneficiary's identification number, name, account number, account status, account type, and institution BIC. (BCE-GG-007-2026, def. 5; Catálogo Operacional §11.1) |
| **Sistema Central de Llaves** | central key registry | BCE-operated registry that guarantees llave uniqueness nationally and stays synchronized in real time with each red's Directorio Distribuido. (Catálogo Operacional §3) |
| **Directorio Distribuido** | distributed directory | Per-red store of llaves and the associated customer credentials. Reconciled against the Sistema Central de Llaves. (Catálogo Operacional, glosario) |
| **Integrante** | participant institution | A member of a red de pagos: private/public banks, cooperativas, mutualistas, cajas centrales, neobanks, SEDPEs, authorized processors, and the BCE itself. (BCE-GG-007-2026, def. 6) |
| **Integrante ordenante / beneficiario** | ordering / beneficiary participant | The sender's institution / the receiver's institution in a payment. |
| **Red de pagos** | payment network | Private technological infrastructure that routes real-time payments among its integrantes (e.g. BANRED). Runs a distributed llave directory for its members. |
| **SIP — Sistema Integrador de Pagos** | payments integrator system | BCE-operated infrastructure that resolves llaves and clears **inter-network** payments, feeding settlement. |
| **RPI — Red de Pagos Instantáneos** | instant payments network | The BCE's own payment network, participating alongside private redes. |
| **Código único de la orden de pago** | payment id | The identifier the ordering integrante generates for each payment: its BIC, the date and time, and a sequence or hash, 20 to 35 characters. Carried in `PmtId/InstrId`; not the ISO `EndToEndId`. (Catálogo Operacional §7, Tabla 14) |
| **Orden de pago intra-red / inter-redes** | intra-/inter-network payment order | Payment between integrantes of the same network / of different networks (the latter routed through the SIP). |
| **Irrevocabilidad** | irrevocability | The point after which a payment is final, binding, and enforceable against third parties. For inter-network payments it attaches when the SIP has notified and the beneficiary red has **received** the order — before the customer credit is confirmed. State `ACSP`. (BCE-GG-007-2026, defs. 9, arts. 12–13; Catálogo Operacional §11.3) |
| **Malla (de compensación)** | netting cycle | The set of bilateral and multilateral net positions the SIP builds in real time from irrevocable payments. Liquidation runs on the multilateral malla, several times a day. (Catálogo Operacional §9) |
| **Por conciliar** | pending reconciliation | State of a llave or payment whose outcome is unresolved after a communication failure. Payments must be given a final state the same day by 18:00 (parameterized), and reconciliation is run by the redes. (Catálogo Operacional §11.4, §13.2) |
| **Nivel de riesgo aceptable** | acceptable risk level | Per-institution ceiling on committed outgoing payments, derived weekly from its reserve requirement. Crossing it makes the SIP block the institution's outgoing payments automatically; re-enablement is manual. (Catálogo Operacional §10) |
| **Catálogo técnico / operacional** | technical / operational catalog | BCE documents defining exact message structures, fields, validations, and flows. **The operational catalog is public** and decoded in [research/catalogo-operacional.md](research/catalogo-operacional.md); the technical one is still outstanding. |
| **CCE — Cámara de Compensación Especializada** | specialized clearing house | BCE clearing system whose specifications govern clearing/settlement information. (BCE-GG-008-2025, art. 9) |
| **Sistema Auxiliar de Pagos** | auxiliary payment system | BCE-authorized private payment infrastructure (e.g. BANRED's services). |
| **SPI — Sistema de Pagos Interbancarios** | interbank payment system | Ecuador's legacy deferred transfer system (three daily windows) at the BCE. Not to be confused with Brazil's SPI. |
| **BCE** | Central Bank of Ecuador | Banco Central del Ecuador. Regulator and operator of SIP/RPI. This project is **not** affiliated with it. |
| **JPRFM** | — | Junta de Política y Regulación Financiera y Monetaria: issues the enabling resolutions. |
| **SEDPE** | — | Sociedad Especializada de Depósitos y Pagos Electrónicos: licensed e-money institution. |
| **Cooperativa de ahorro y crédito** | savings & credit cooperative | Ecuador's ~365 credit unions; supervised by SEPS. The long tail this project serves first. |
| **Caja central** | central facility for cooperatives | Second-tier institution providing services (including tech) to member cooperativas — a natural multi-tenant gateway host. |

## Messages and standards

| Term | Definition |
|---|---|
| **ISO 20022** | Global financial messaging standard. Ecuador mandates it for real-time payments up to USD 15,000. |
| **pacs.008** | ISO 20022 "FI to FI Customer Credit Transfer" — the payment order, sent by the integrante ordenante. |
| **pacs.002** | ISO 20022 "FI to FI Payment Status Report" — accept/reject responses and irrevocability declarations. |
| **acmt** | ISO 20022 account management message family — used for llave resolution (query → credencial de pago). |
| **BIC** | Bank Identifier Code (SWIFT). Mandatory identifier for integrantes; BCE assigns alternatives to institutions without one. |
| **EMV QRCPS** | EMV QR Code Specification for Payment Systems (merchant-presented). The regional QR standard (used by Bre-B); Ecuadorian QRs must carry the beneficiary's llave. |
| **mTLS** | Mutual TLS — both sides authenticate with certificates. Baseline transport security for scheme connections. |
| **JWS / XMLDSig** | Detached message signature schemes (Pix uses JWS for SPI, XMLDSig for DICT) — non-repudiation on top of mTLS. |

## Payments concepts

| Term | Definition |
|---|---|
| **Clearing (compensación)** | Exchanging and validating payment instructions and calculating obligations. Happens in real time in this scheme. |
| **Settlement (liquidación)** | The actual transfer of funds between institutions — in central bank money at the BCE. |
| **RTGS** | Real-Time Gross Settlement: each payment settles individually and immediately (Pix's SPI, Bre-B's MOL). Our reference `settlement/` model. |
| **Deferred net settlement (DNS)** | Obligations accumulate and settle periodically on a net basis. Cheaper in liquidity, but institutions carry intraday credit exposure. |
| **Prefunding / no-overdraft** | RTGS discipline where participants must hold balance before paying — eliminates credit risk (Pix's Conta PI rule). |
| **Confirmation of payee** | Showing the (masked) beneficiary name after llave resolution, before the payer confirms — key anti-error/anti-fraud UX from Pix/Bre-B. |
| **Idempotency key** | Client-supplied unique key that makes payment initiation safely retryable — no double sends. |
| **Directory (directorio de llaves)** | The service mapping llaves → credenciales de pago; distributed per red, integrated via the SIP. Analog of Pix's DICT and Bre-B's DICE. |
| **Four-party model** | Payer app/PSP + payer bank + payee bank + payee — the UPI participation topology. |

## Terms in code

The Rust type each term becomes. The shapes are fixed in [ADR-0014](architecture/adr/0014-kernel-types.md) and [ADR-0017](architecture/adr/0017-time-calendar-identifiers.md); the payment and ledger language is in [`CONTEXT.md`](../CONTEXT.md).

| Term | Type | Crate |
|---|---|---|
| Amount in dollars | `Usd` (integer cents) | `types` |
| BIC (integrante or red) | `Bic` | `types` |
| Código único de la orden de pago | `PaymentId` (holds the ordering `Bic`) | `types` |
| Payment state (Tabla 11) | `PaymentState` | `types` |
| Reason code (Tabla 12) | `ReasonCode`; a code received from a counterparty is a `ReceivedReason` | `types` |
| Llave type (Tabla 1) | `LlaveType` | `types` |
| Business day, operating hours, cut-offs, reconciliation close | `BusinessCalendar` | `types` |
| Llave de pago | `Llave` | `directory` |
| Credencial de pago | `Credencial` | `directory` |
| Masked beneficiary name | `MaskedName` | `directory` |
| Ledger account, headroom | `AccountId`, `Headroom` | `ledger` |

## Project components (this repo)

The full crate map and its dependency rules are in [ADR-0012](architecture/adr/0012-crate-boundaries.md).

| Component | What it is |
|---|---|
| `iso20022` | Library: read/write/validate the BCE profile of pacs.008, pacs.002, acmt. |
| `south-api` | Translation between `iso20022` and the domain, and the loop that runs each node's role over `msgplane` ([ADR-0018](architecture/adr/0018-composition-and-message-plane.md)). |
| `directory` | Llaves: registration, uniqueness, resolution, synchronization with the Sistema Central de Llaves. |
| `payments` | The payment state machine, shared by `sip`, `red` and `gateway`. |
| `ledger` + `clearing` | Obligation and headroom ledgers; mallas, netting and settlement instructions. |
| `fees` | Fee schedules and quotes (the *tariff module* of ADR-0010). |
| `gateway` | The participant connector: simple North API inward, compliant ISO 20022 outward. Single- and multi-tenant. |
| `caudal` | The terminal: the whole network in one process, for exploring and measuring it. |
| `tck` | Test & certification kit: conformance scenarios against the catálogos. |
| `sandbox/` | "Ecuador's payment network in a box" — full simulated scheme via docker compose. |
| **North API** | Our institution-facing REST/webhook API (simple JSON; no ISO 20022 knowledge needed). |
| **South API** | The scheme-facing mandated interface: signed ISO 20022 over mTLS. |
