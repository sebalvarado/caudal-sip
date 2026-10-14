# caudal-payments — the payment path, one machine per role

**Owns** the state machines of the SIP, a red, and an integrante ([ADR-0018](../../docs/architecture/adr/0018-composition-and-message-plane.md)): each takes an input (a message, a ledger outcome, a deadline, a settlement) and returns outputs (messages, ledger commands, deadlines, events). Every catálogo rule about a payment's path lives here once.

**Never depends on** `caudal-iso20022`, `caudal-msgplane`, any adapter, or anything that does I/O. Time and ids arrive in `Ctx`.

**Rules**

- The SIP reserves before routing, posts on `ACSP`, voids on a rejection, and is idempotent on the payment id: a repeat is answered again, a conflicting one is `DU01`.
- The four exceptions are deadlines: undelivered is void with `TA01`, delivered but unconfirmed is post plus a suspense memo (por conciliar), and an ordering side that never hears back reports "en proceso".
- Deadlines nest inside the 20-second limit; `Deadlines::new` refuses values that don't.
- Intra-red payments stay in the red and never touch the scheme ledger **[assumption]**.
- Commissions post as zero until `fees` exists.
