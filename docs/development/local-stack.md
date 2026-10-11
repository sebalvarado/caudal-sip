# Running the Network Locally

## The commitment

**One command runs a complete simulated Ecuadorian payment network. A second sends a payment through it.** From `git clone` to watching a llave-addressed payment clear between two networks: **under 15 minutes, on a laptop, with no cloud account, no credentials and no Docker.**

That number is not a nicety. It is the same experience for a curious developer, a cooperativa's core-banking vendor evaluating us, and a regulator we are trying to convince — and for the last two, the demo *is* the argument ([`../product/north-star.md`](../product/north-star.md), rung 2).

## What runs

`caudal` is one binary ([ADR-0018](../architecture/adr/0018-composition-and-message-plane.md), [ADR-0019](../architecture/adr/0019-caudal-terminal.md)). It starts every node of the network in one process, on a simulated clock and the in-memory ledger backend (the same `Ledger` trait the Postgres backend implements):

| Node | BIC | Role |
|---|---|---|
| `sip` | `SIPCECEQ` | Inter-red routing, validation, the scheme ledger, the malla |
| red alfa | `RALFECEQ` | A payment network with banco-alfa and mutual-delta |
| red beta | `RBETECEQ` | A competing network with coop-beta and fintech-gamma — Ecuador is multi-network, and a single-network demo hides the hard parts |
| banco-alfa | `BALFECEQ` | Sector 0002, ceiling 4,800,000.00 |
| mutual-delta | `MDELECEQ` | Sector 0004, ceiling 1,920,000.00 |
| coop-beta | `CBETECEQ` | Sector 0006, a segment-3 cooperativa, ceiling 60,000.00 |
| fintech-gamma | `FGAMECEQ` | Sector 0009, ceiling 15,000.00 |

Every message between nodes is a signed ISO 20022 envelope on our **[assumption]** profile, encoded and verified on every hop even in one process. Ceilings follow Tabla 20 of the catálogo from seeded deposits.

Institutions, holders, documents and accounts are fictional. No real institution's name appears in seed data, fixtures or demos — a demo that implies a real bank's participation is a problem we do not need.

## The demo

```bash
just terminal
```

Opens the full-screen terminal: a command line, the payment blotter, headroom with the catálogo's traffic light, malla net positions and per-hop compute stats. `help` lists the commands. Then do it by hand, which is the part that convinces people:

```text
resolve llave-02                    masked name and institution, for payer confirmation
pay coop-beta llave-02 25.40        a cooperativa on red beta pays a bank on red alfa through the SIP
show last xml                       every hop, every message, every posting
settle                              the malla closes: ACSP becomes ACSC
pay coop-beta llave-02 59000        coop-beta spends its ceiling down
pay coop-beta llave-05 2000         blocked by the ledger: RJCT AM04
settle                              headroom comes back, and the same payment goes through
fault 3                             the next payment loses its confirmation and lands por conciliar
scenario payroll                    300 salaries against a 60,000.00 ceiling
```

Every command is also a scriptable subcommand:

```bash
caudal llave resolve --llave llave-02
caudal pay --from coop-beta --llave llave-02 --amount 25.40 --settle --xml
caudal payment show <payment-id>
caudal scenario national --json
printf 'pay coop-beta llave-02 25.40\nsettle\nnetwork\n' | caudal batch
caudal validate crates/iso20022/tests/golden/pacs008.xml
```

(`just terminal <args>` is `cargo run -p caudal -- <args>`.)

## Seeded state

The sandbox starts with a network that is already interesting, because an empty system demonstrates nothing: llaves of every catálogo type, one blocked and one closed account, and a short history with payments in each terminal state — settled, accepted, rejected for each reason, and one **por conciliar**, because that state is the one everybody gets wrong and it must be visible from the first minute.

Document numbers start with province `00`, which no real cédula uses.

## Exercising failure

The catálogo's four exceptions are commands, not afterthoughts. `fault 1` through `fault 4` drop exactly the message each exception describes; `fault off` clears them. The same flows are covered by the network tests in `crates/south-api/tests/network.rs`.

## Working on it day to day

| Command | Does |
|---|---|
| `just check <crate>` / `just test <crate>` | The fast loop |
| `just test` | Every test. Starts this worktree's Postgres for the adapter tests |
| `just terminal` | The network in a box |
| `just simulation` | Regenerate the published one-day simulation |
| `just ci` | Everything CI runs, once before a PR |

Each worktree gets its own Postgres database on a port that never collides with another project's (55432 by default). Nothing but the Postgres adapter tests needs it.

## Prerequisites

The pinned Rust toolchain and `just`. Docker only for the Postgres adapter tests. Deliberately nothing else — no second language runtime, no Kubernetes, no cloud CLI, no account anywhere.

If setting this up takes more than fifteen minutes on a clean machine, that is a bug worth an issue, not a documentation problem.

## Design rules this experience imposes

1. **Every component runs standalone.** A developer working on the message library never starts a network.
2. **No component requires a secret to start in development.** Signing keys are derived from fictional seeds at start-up.
3. **The sandbox runs the same code as production**, configured differently: the same role machines, codec and ledger trait. A simulator that shares no code with the real thing proves nothing.
4. **Any state a developer can reach, the terminal can show.** If something is only visible in the database, the operations staff at an institution will be unable to see it either ([`../product/requirements.md`](../product/requirements.md), surfaces).
