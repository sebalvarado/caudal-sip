# Running the Full Stack Locally

> **Status: this is a design contract, not documentation of something that exists.** No Go code is in the repository yet. This document specifies the developer experience we are committing to build, because writing it first is what forces the architecture to stay operable. When the code lands, this file becomes the setup guide and this notice goes away.

## The commitment

**One command brings up a complete simulated Ecuadorian payment network. A second command sends a payment through it.** From `git clone` to watching a llave-addressed payment clear between two simulated institutions: **under 15 minutes, on a laptop, with no cloud account and no credentials.**

That number is not a nicety. It is the same experience for a curious developer, a cooperativa's core-banking vendor evaluating us, and a regulator we are trying to convince — and for the last two, the demo *is* the argument ([`../product/north-star.md`](../product/north-star.md), rung 2).

## What comes up

```
make dev
```

| Service | Port | Role in the simulation |
|---|---|---|
| `sip` | 7000 | The integrator: inter-network routing, central llave resolution, irrevocability for inter-network payments |
| `switch` (red alfa) | 7010 | Intra-network routing and clearing for one payment network |
| `switch` (red beta) | 7020 | A second, competing network — because Ecuador is multi-network, and a single-network demo hides the hard parts |
| `llaves` | 7100 | Directory: registration, uniqueness, resolution |
| `settlement` | 7200 | Double-entry ledger, gross real-time by default |
| `gateway` (banco-alfa) | 7301 | A direct participant on red alfa |
| `gateway` (coop-beta) | 7302 | A participant on red beta, running in multi-tenant mode |
| `gateway` (fintech-gamma) | 7303 | An indirect participant riding on coop-beta's deployment |
| `console` | 7400 | Watch messages traverse the network, inspect and replay them |
| `postgres` | 5432 | One instance, one schema per service |

Institutions are deliberately fictional (`banco-alfa`, `coop-beta`, `fintech-gamma`). No real institution's name appears in seed data, fixtures, or demos — not as a courtesy, but because a demo that implies a real bank's participation is a problem we do not need.

## The demo

```
make demo
```

Runs the scenario the whole project exists to make ordinary: María at `banco-alfa` pays Don Julio's tienda at `coop-beta` by scanning a QR containing his llave. Inter-network, so it traverses red alfa, the SIP, and red beta. The command prints each hop with its timing and finishes with the irrevocability notification and the resulting ledger postings.

Then do it by hand, which is the part that convinces people:

```bash
caudal llave register --tenant coop-beta --llave 0991234567 --account 4400123456
caudal llave resolve  --llave 0991234567          # masked name, for payer confirmation
caudal pay --from banco-alfa --llave 0991234567 --amount 25.40
caudal payment show <end-to-end-id>               # full state history and every message
```

The CLI name `caudal` is **[proposal]** pending an ADR; it should match whatever the Go module path becomes.

## Seeded state

`make dev` seeds a network that is already interesting, because an empty system demonstrates nothing:

- Three institutions with BICs, certificates, and signing keys — all self-signed and generated locally, never committed.
- Customer accounts with balances at each institution, and llaves of every supported type already registered.
- Settlement accounts funded so payments can actually clear.
- A handful of historical payments in each terminal state, including a **timed-out payment awaiting reconciliation**, because that state is the one everybody gets wrong and it must be visible from the first minute.

Synthetic identifiers only: generated document numbers with valid check digits, fictional names, no real people.

## Verifying it works

```bash
make check          # health of every service, then the end-to-end demo, then exit non-zero on any failure
```

Then confirm by hand: `curl localhost:7301/health` reports the gateway and its dependencies; the console at `localhost:7400` lists the three institutions and the seeded payments; `caudal validate examples/pacs008-valid.xml` passes and `examples/pacs008-invalid.xml` fails with a readable reason.

Exercising failure is a first-class part of the local experience, not an afterthought:

```bash
make chaos-timeout      # beneficiary stops responding: payment must land in TIMED_OUT and reconcile
make chaos-duplicate    # same message twice: exactly one credit, exactly one set of postings
make chaos-badsig       # tampered signature: rejected at the switch, nothing reaches the beneficiary
```

## Working on it day to day

| Command | Does |
|---|---|
| `make dev` | Start everything, seeded, in the foreground with aggregated logs |
| `make down` | Stop everything, keep data |
| `make reset` | Drop and reseed the databases |
| `make test` | Unit and property tests |
| `make test-e2e` | Full network scenarios against a running stack |
| `make tck` | Conformance suite against a target, producing the shareable report |
| `make lint` | `golangci-lint` plus format check |

Two conventions worth adopting from day one because retrofitting them is painful: **deterministic per-worktree port offsets and database names**, derived from the directory name, so several branches run simultaneously without conflict; and **a database snapshot you can restore** rather than only a seed script, so debugging a specific state does not require replaying a scenario.

## Prerequisites

Go 1.23+, Docker with Compose, and `make`. Deliberately nothing else — no language runtime beyond Go, no Kubernetes, no cloud CLI, no account anywhere. Postgres runs in Docker; a client is useful but optional.

If setting this up takes more than fifteen minutes on a clean machine, that is a bug worth an issue, not a documentation problem.

## Design rules this experience imposes

Stated explicitly because they constrain the architecture, and because the temptation to break them arrives early:

1. **Every component must run standalone.** A developer working on the message library never starts a network. Components that only work in a full topology are undebuggable and unadoptable.
2. **No component may require a secret to start in development.** Keys and certificates are generated locally on first run.
3. **The sandbox uses the same binaries as production**, configured differently. A simulator that shares no code with the real thing proves nothing, which is the failure mode of most reference implementations.
4. **Any state a developer can reach, the console can show.** If something is only visible in the database, the operations staff at an institution will be unable to see it either ([`../product/requirements.md`](../product/requirements.md), surfaces).
