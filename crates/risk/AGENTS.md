# caudal-risk — the liquidity ceiling

**Owns** the nivel de riesgo aceptable (encaje × risk factor, Tabla 19 and 20), the traffic light, and funding adjustments to an integrante's headroom account ([ADR-0009](../../docs/architecture/adr/0009-risk-ceiling-as-funded-account.md), [`posting-patterns.md`](../../docs/architecture/posting-patterns.md) §3).

**Never depends on** anything but `caudal-types` and `caudal-ledger`, or on anything that does I/O.

**Rules**

- Rounding is half-up to the cent at each step, as in Tabla 20.
- A decrease never takes headroom below zero: the remainder is a pending reduction, applied as commitments clear.
- Nothing here re-enables an integrante after a block. Re-enablement is an explicit, audited funding entry.
- Reserve figures are seeded until the BCE feed exists **[assumption]**.
