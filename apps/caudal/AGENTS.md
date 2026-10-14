# caudal — the network in a box

**Owns** composition: the fictional seed (`sandbox`), the command language shared by the CLI and the TUI (`commands`), the scenario language and its runner (`script`), the built-in scenarios (`scenarios`), the ratatui terminal (`tui`), and the WebAssembly entry points of the browser simulator (`wasm`, ADR-0022). It is the only package allowed `ratatui` and `wasm-bindgen`.

**Never** holds domain rules. If a decision about a payment, a ceiling or a malla is made here, it belongs in a domain crate.

**Rules**

- Seed data is fictional and stays fictional: no real institution names, people, documents or accounts. Documents start with province `00`, which no real cédula uses.
- A script can only name seeded institutions and llaves, so it cannot carry real data. Keep it that way when adding statements.
- Every public output carries the disclaimer.
- Scripts are deterministic for a seed: same seed, same report, on every platform, except the compute timings.
- Commands print through `commands::execute`, so the CLI, the TUI and scripts never diverge.
- The library builds for `wasm32-unknown-unknown`: no threads, no clock but `web_time`, no filesystem. The terminal's dependencies stay behind `cfg(not(target_arch = "wasm32"))`.
- The simulator under `site/public/simulador/pkg/` is committed. After changing any Rust it is built from, run `just playground` and commit the result; CI fails on a stale one.
