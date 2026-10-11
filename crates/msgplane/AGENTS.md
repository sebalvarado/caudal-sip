# caudal-msgplane — signed bytes and time

**Owns** the signed envelope (ed25519 over sender, receiver and body), the keyring, the simulated clock, and the event schedule of the in-process network ([ADR-0019](../../docs/architecture/adr/0019-caudal-terminal.md)).

**Never depends on** a domain crate, `caudal-iso20022`, or anything with a queue or retry store on the payment path.

**Rules**

- `Signer::derive` is for in-process networks only. No key is ever committed.
- The clock only moves forward; events at the same instant keep their scheduling order, so runs replay exactly.
