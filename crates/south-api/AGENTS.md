# caudal-south-api — the scheme-facing adapter

**Owns** translation between domain messages and ISO 20022 (`wire`), and the network loop: sign and send with a latency, verify and decode, step the receiving machine, execute its ledger commands and arm its deadlines. Also the payment blotter and per-hop timing (`Stats`).

**Never depends on** an app, `tokio`, `axum`, or a database driver.

**Rules**

- Every message between nodes is encoded and signed, even in one process.
- Timings are a floor: computation only, no network, TLS, or core banking.
- A `LedgerError::Unknown` is retried, never treated as a refusal.
