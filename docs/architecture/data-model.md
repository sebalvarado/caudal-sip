# Data Model

The entities, lifecycles, and invariants shared by every component. This is the contract the code must not violate; wire-level message design is in [`scheme-design.md`](scheme-design.md) and the vocabulary is in [`../glossary.md`](../glossary.md).

Postgres first, per [ADR-0007](adr/0007-postgres-first.md). Field names here are conceptual — Spanish domain terms (*llave*, *integrante*, *credencial*) are kept deliberately, because translating them produces bugs in conversations with institutions.

Anything marked **[assumption]** is pending the BCE catálogo técnico; **[proposal]** is ours and needs an [RFC](../rfcs/README.md).

## The five domains

```
identity ──────┐
               ├──> payments ──> settlement
llaves ────────┘        │
                        └──> audit (append-only, spans everything)
```

Each component owns its tables and never reaches into another's. The gateway may cache directory data but is never the source of truth for a llave.

## 1. Identity and participation

| Entity | Purpose | Key fields |
|---|---|---|
| `red_de_pagos` | A payment network | id, name, status, settlement arrangement |
| `integrante` | An institution participating in the scheme | id, **BIC**, legal name, tax id, type (banco, cooperativa, mutualista, SEDPE, fintech), red membership, direct or indirect, status |
| `tenant` | One institution inside a multi-tenant gateway | id, integrante id, isolation boundary for every other table |
| `credential_material` | mTLS certificates and signing keys | id, owner, kind, fingerprint, not-before, not-after, rotation state, **never the private key itself** |

Rules that matter:

- **Every tenant-scoped row carries `tenant_id`, always, with no nullable exception.** Multi-tenancy is the feature that makes the long tail reachable, and retrofitting isolation is how data leaks happen. Enforce with row-level security in the database, not only in application code.
- An indirect participant is modeled as an `integrante` with its own BIC whose traffic flows through another party's infrastructure. Indirect participation is a deployment fact, not a lesser identity.
- `credential_material` stores metadata and references to a KMS or HSM. Private key bytes never enter the database or the repository.

## 2. Llaves and the directory

| Entity | Purpose | Notes |
|---|---|---|
| `llave` | The public identifier a payer types | value, type (phone, cédula/RUC, email, alphanumeric), status, owning integrante, timestamps |
| `credencial_de_pago` | What a llave resolves to | holder document type and number, account number, account type, institution BIC (BCE-GG-007-2026, def. 10) |
| `llave_claim` | Portability or reclaim in flight **[proposal]** | claiming integrante, losing integrante, state, deadline, resolution |
| `llave_event` | Every lifecycle change | append-only; who, when, what changed, source message |

Invariants, enforced in the database rather than trusted to code:

1. **A llave resolves to exactly one credencial at any instant** — unique index on the llave value across the directory. This is what makes "send it to my number" unambiguous, and it is the same rule as Pix and Bre-B.
2. **A person may hold many llaves**, each pointing wherever they choose, capped per account **[proposal]**.
3. **Re-pointing is a normal operation**, owned by the receiving customer, requiring no coordination with anyone who has paid them before.
4. **Resolution responses carry the minimum**: a masked name for confirmation, never full identity. Every resolution is rate-limited per caller and written to `llave_event`, because directory scraping is a known and expensive attack.
5. Deleting a llave is a status change plus an event, never a row deletion. Payments referencing it must remain explicable years later.

### Llave lifecycle

```
                  ┌─────────────────────────────┐
  register ──> ACTIVE ──block──> BLOCKED ──unblock──┘
                  │  ▲
       claim (other institution)
                  ▼  │ confirmed / expired
           CLAIM_PENDING
                  │
               removed
                  ▼
               REMOVED  (terminal; value may be re-registrable after a cooling period [proposal])
```

## 3. Payments

The central entity is the **payment instruction**, one row per attempt from the perspective of each participant that touches it.

| Entity | Purpose | Notes |
|---|---|---|
| `payment` | The instruction and its current state | end-to-end id, message id, amount, currency, initiation channel (llave, QR, explicit credencial), payer and payee credencial snapshots, purpose, timestamps per transition |
| `payment_message` | Every wire message sent or received | direction, type (pacs.008, pacs.002, acmt), payload hash, signature reference, timestamp, correlation ids |
| `payment_state_change` | Append-only transition log | from, to, reason code, actor, message reference |
| `idempotency_record` | Deduplication | (tenant, key) unique, request fingerprint, stored response |
| `webhook_delivery` | Outbound notification attempts | event, target, attempts, next retry, terminal state |

### Payment lifecycle

States are the scheme's own, from Catálogo Operacional §5 ([decoded here](../research/catalogo-operacional.md)) — do not invent an internal enum that has to be mapped:

```
              debit + reserve funds
CREATED ──> RCVD ──> ACTC ──send pacs.008──> in flight
                       │                         │
                  (local reject)      ┌──────────┼───────────┐
                       ▼              ▼          ▼           ▼
                     RJCT           ACSP       RJCT        PDNG
                                 (irrevocable, (return   (por conciliar,
                                  in today's    funds)    unknown outcome)
                                  malla)                      │
                                      │      resolved same day, by 18:00
                                      │  ◄───────────────────┘
                              malla liquidated
                                      ▼
                                    ACSC  (funds legally transferred)
```

Three properties carry most of the domain's difficulty:

- **`PDNG` is not a failure, it is an unknown outcome.** The ordering institution does not know whether the beneficiary was credited, and in exceptions 3 and 4 the payment is *already irrevocable* while the ordering side is still waiting. Funds stay reserved; they are neither released to the beneficiary's benefit nor returned. Treating a timeout as a rejection creates double credits. Reconciliation must assign a final state the same day by **18:00** (parameterized), and it is run by the red with data from the SIP.
- **Irrevocability attaches early** — when the SIP has notified and the beneficiary red has *received* the order, not when the customer is credited. Any logic that waits for the credit confirmation before treating the payment as committed is wrong.
- **Customer finality and interbank finality are different events.** `ACSP` is when the beneficiary can spend the money; `ACSC` is when the multilateral malla liquidates at the BCE, later the same day or the next business day. Store both timestamps and never conflate them; any report saying "settled" must say which one it means.

Reason codes are the scheme's too (`AC04`, `AC01`, `AC06`, `AC05`, `AC07` on the beneficiary side; `DT01` shared; `AM04`, `MS01`, `AM01`, `AM02`, `AM11`, `AC02`, `CH16`, `CH20`, `CH21`, `DT02`, `DS0K`, `DU01`, `TA01`, `FF02`, `FF10` on the SIP side — note `AM04` "insufficient funds" is the SIP's, signalling malla-level insufficiency rather than a customer balance). They are standard ISO 20022 external codes.

Additional rules:

- `payment` rows are **immutable except for state and timestamps**. Amounts, parties, and references never change after creation; a correction is a new instruction.
- The credencial is **snapshotted onto the payment**, not referenced by foreign key. What the directory said at the moment of payment is evidence, and it must not change when the customer later re-points the llave.
- End-to-end identifiers are unique per originating integrante **[assumption:** uniqueness window pending the catálogo**]**, and the pair (originator BIC, end-to-end id) is the natural deduplication key across the whole network.

## 4. Settlement

A double-entry ledger. Nothing else is acceptable in a payment system, and the temptation to keep a mutable balance column is the single most common way these systems become unauditable.

| Entity | Purpose |
|---|---|
| `settlement_account` | One per participant per settlement arrangement |
| `journal_entry` | An atomic, balanced financial event, referencing its payment |
| `posting` | One debit or credit line belonging to a journal entry |
| `settlement_cycle` | A deferred-net window, for the simulation model |
| `position` | Derived net obligation per participant per cycle |

Invariants:

1. **Every journal entry sums to zero** across its postings. Enforced by constraint, verified by a property-based test that no sequence of operations can break.
2. **Postings are append-only.** A reversal is a new, opposing entry that references the original. Nothing is ever updated or deleted.
3. **No overdraft**: a posting that would take a settlement account negative is rejected, not queued. This is the Pix model and the reference behavior we implement.
4. **Balance is derived** from postings, never stored as the truth. A materialized balance may exist as a cache with a reconciliation job that proves it matches.
5. Every entry references the payment and the message that caused it, so any figure on any screen can be traced to a signed wire message.

## 5. Audit

One append-only log spanning all domains, because in payments the question is never only "what is the state" but "what happened, in what order, and who can prove it".

Each record carries: timestamp, actor (institution, user, or system), action, affected entity, tenant, correlation id, and the hash of any wire message involved. Records are written in the same transaction as the change they describe, so a state change without an audit entry is impossible. The log is chained by hash **[proposal]** so tampering is detectable, and retention follows the regulatory period once we can cite it **[assumption]**.

What must never enter the audit log, or any log: full account numbers, full document numbers, private keys, or complete message payloads containing personal data. Log the hash and the correlation id, and keep the payload where access is controlled and expiring.

## Data protection

- **Minimize by default.** The switch is a router: it needs routing data and evidence, not customer identity. Store metadata, not people.
- **Mask on the way out.** Directory responses and console screens show partial names and truncated identifiers unless the viewer has a specific, audited reason for more.
- **Encrypt at rest** the columns holding document numbers and account numbers, with keys in a KMS and rotation designed in rather than added later.
- **Never in fixtures.** Test data is synthetic — generated document numbers with valid check digits, fictional institutions. No real institution names, no real people, not even in a demo. This is a hard rule from [`../../AGENTS.md`](../../AGENTS.md).

## Open modeling questions

1. Return and refund semantics — a `pacs.004` analog and whether a fraud-return mechanism like Pix's MED exists in Ecuador's catalog.
2. The exact reason-code taxonomy, which must match the catálogo exactly rather than being invented.
3. Whether the directory is authoritative centrally, federated per red, or both, and what the reconciliation model between them is ([`scheme-design.md`](scheme-design.md) §3).
4. Retention periods per entity class, pending the applicable norm.
5. Whether a bulk-disbursement instruction is a first-class entity or a batch of ordinary payments **[proposal:** the latter, so the settlement invariants stay unchanged**]**.
