# caudal — the network in a box

**Owns** composition: the fictional seed (`sandbox`), the command language shared by the CLI, the TUI and `batch` (`commands`), the synthetic scenarios (`scenarios`), the ratatui terminal (`tui`) and the public demo API (`serve`, ADR-0021). It is the only package allowed `axum`, `tokio` at the edge, and `ratatui`.

**Never** holds domain rules. If a decision about a payment, a ceiling or a malla is made here, it belongs in a domain crate.

**Rules**

- Seed data is fictional and stays fictional: no real institution names, people, documents or accounts. Documents start with province `00`, which no real cédula uses.
- The demo API accepts catalogue ids only (institutions, llaves, amounts within limits). Nothing free-form reaches the network, and client addresses are never logged.
- Every public output carries the disclaimer; the `x-caudal-disclaimer` header is the ASCII form because header values are ASCII.
- Scenarios are deterministic for a seed: same seed, same report, except the compute timings.
- Commands print through `commands::execute`, so the CLI, the TUI and `batch` never diverge.
