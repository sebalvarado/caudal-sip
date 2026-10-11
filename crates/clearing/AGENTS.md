# caudal-clearing — the malla and its settlement

**Owns** multilateral netting of posted obligations, the bilateral report, and the settlement entry that closes positions against `scheme:liquidado` and releases committed headroom ([`posting-patterns.md`](../../docs/architecture/posting-patterns.md) §1.3–1.4).

**Never depends on** anything but `caudal-types` and `caudal-ledger`, or on anything that does I/O.

**Rules**

- Settlement runs on the multilateral malla; the bilateral one is reporting only.
- A malla covers every payment posted since the last settlement entry. Pending reservations are never in it.
- The settlement instruction's id is the transfer id, so a replayed instruction is a ledger repeat, not a second posting.
