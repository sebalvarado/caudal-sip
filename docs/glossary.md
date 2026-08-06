# Glossary / Glosario

The vocabulary of record for this project. Spanish terms from Ecuadorian regulation are **used as-is in all languages** — write *llave*, not "alias key". Where a term comes from a specific resolution, it's cited.

Términos del ecosistema ecuatoriano de pagos, en ambos idiomas. Los términos regulatorios en español se usan tal cual en todos los idiomas del proyecto.

## Ecuador's scheme (regulatory terms)

| Término (ES) | English | Definition |
|---|---|---|
| **Llave de pago** | payment key | Public, unique identifier (phone number, cédula, email, alphanumeric code) that a customer links to one account to send/receive payments. One llave → exactly one account. (BCE-GG-007-2026, def. 10) |
| **Credencial de pago** | payment credential | The data a llave resolves to: beneficiary's cédula/RUC/passport + account number + account type + institution BIC. (BCE-GG-007-2026, def. 5) |
| **Integrante** | participant institution | A member of a red de pagos: private/public banks, cooperativas, mutualistas, cajas centrales, neobanks, SEDPEs, authorized processors, and the BCE itself. (BCE-GG-007-2026, def. 6) |
| **Integrante ordenante / beneficiario** | ordering / beneficiary participant | The sender's institution / the receiver's institution in a payment. |
| **Red de pagos** | payment network | Private technological infrastructure that routes real-time payments among its integrantes (e.g. BANRED). Runs a distributed llave directory for its members. |
| **SIP — Sistema Integrador de Pagos** | payments integrator system | BCE-operated infrastructure that resolves llaves and clears **inter-network** payments, feeding settlement. |
| **RPI — Red de Pagos Instantáneos** | instant payments network | The BCE's own payment network, participating alongside private redes. |
| **Orden de pago intra-red / inter-redes** | intra-/inter-network payment order | Payment between integrantes of the same network / of different networks (the latter routed through the SIP). |
| **Irrevocabilidad** | irrevocability | The point after which a payment is final, binding, and enforceable against third parties; declared via `pacs.002` by the red (intra) or the SIP (inter). (BCE-GG-007-2026, defs. 9, arts. 12–13) |
| **Catálogo técnico / operacional** | technical / operational catalog | BCE documents defining exact message structures, fields, validations, and flows. The ground truth for `iso20022/` and `tck/`. |
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

## Project components (this repo)

| Component | What it is |
|---|---|
| `iso20022/` | Go library: read/write/validate the BCE profile of pacs.008, pacs.002, acmt. |
| `llaves/` | Directory service: registration, uniqueness, resolution, portability. |
| `switch/` | Stateless validator/router (UPI-inspired). |
| `settlement/` | Ledger + RTGS simulator (and DNS mode for comparison). |
| `gateway/` | The participant connector: simple North API inward, compliant ISO 20022 outward. Single- and multi-tenant. |
| `tck/` | Test & certification kit: conformance scenarios against the catálogos. |
| `sandbox/` | "Ecuador's payment network in a box" — full simulated scheme via docker compose. |
| **North API** | Our institution-facing REST/webhook API (simple JSON; no ISO 20022 knowledge needed). |
| **South API** | The scheme-facing mandated interface: signed ISO 20022 over mTLS. |
