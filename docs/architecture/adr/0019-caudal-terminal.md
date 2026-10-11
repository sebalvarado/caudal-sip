# 0019. The `caudal` terminal: a deterministic simulation of the whole network, driven from one binary

- **Status**: Accepted
- **Date**: 2026-10-11
- **Deciders**: Founding maintainer
- **Issue**: [#2](https://github.com/sebalvarado/caudal-sip/issues/2) (supersedes [#11](https://github.com/sebalvarado/caudal-sip/issues/11))
- **Refines**: [0018](0018-composition-and-message-plane.md), on how the in-process clock runs

## Context

Epic 1 promises one binary that runs `sip`, two redes, and mock institutions in one process, with no Docker. From it you send payments, resolve llaves, settle mallas, watch headroom and positions, and run load scenarios such as a payroll day ([`epics.md`](../../plan/epics.md)). [ADR-0018](0018-composition-and-message-plane.md) fixed that the terminal runs the same machines, codec, and signatures as production, with an in-memory transport and a simulated clock that jumps to the next deadline whenever every node is idle. It left the terminal's own shape to this record.

## Decision

**One binary, `caudal`, in `apps/caudal`.** Every operation is a subcommand (`caudal pay`, `caudal resolve`, `caudal settle`, `caudal scenario`, `caudal positions`), so a script or CI job can drive it. `caudal` with no arguments opens the full-screen interface. `caudal serve` exposes the public demo API ([ADR-0021](0021-public-demo-api.md)).

**The full-screen interface uses `ratatui` with `crossterm`.** It has a command line that accepts the same commands as the subcommands. Each command shows the request, the ISO 20022 messages it produced, and the response. Around it sit four panels: the payment blotter with each payment's state, each integrante's headroom with the catálogo's traffic light, the malla's net positions, and execution statistics per operation.

**The in-process network is a discrete-event simulation.** It keeps one queue of events ordered by simulated time: a signed message arriving at a node, or a deadline firing. The loop pops the next event, sets the clock to its time, and hands it to that node's machine through `south-api`. The machine's outputs go back on the queue: messages after a configurable per-hop latency, and deadlines. This is the clock of ADR-0018, which jumps when every node is idle. It needs no async runtime, and a run with the same seed replays exactly. The network transport still runs on Tokio when the nodes run as separate services. `msgplane` therefore models a send as one signed message with no implicit reply: a `pacs.002` is its own message, and a missing answer is a deadline event. That carries the same meaning as ADR-0018's "reply or unknown outcome".

**What it measures.** Each hop's real computation, timed with a monotonic clock, is reported as p50, p95, and p99 per operation: codec, signature, domain rules, and the ledger commit. Throughput is reported in operations per second of computation, and each payment's simulated time from reservation to post. As ADR-0018 says, this is a floor, not a production latency claim.

**Seed data is fictional and built in.** It covers the SIP, red alfa and red beta, and at least three institutions (`banco-alfa`, `coop-beta`, `fintech-gamma`) with their BICs. It includes llaves of every type with fictional holders, and risk ceilings computed from Tabla 19 on seeded deposit figures. No real institution or person appears.

**Scenarios are named and seeded**: a single payment, a payroll day, a cut-off rush, and one day of national traffic. The last produces the public artifact.

## Alternatives considered

- **One Tokio task per node, with a simulated clock advanced when every task is idle.** This works, but detecting "every node is idle" across tasks is fiddly, and scheduling makes runs nondeterministic. A single event queue gives the same semantics, deterministic.
- **A web UI instead of a terminal UI.** Rejected for the developer surface: the terminal needs no browser and no second language. The public demo page covers the browser case.
- **Separate binaries started together by a script.** That is the production shape, and the network transport will use it. It needs ports, processes, and certificates, which the fifteen-minute promise cannot afford.

## Consequences

Easier: a payroll day runs in seconds and replays exactly. Every failure case is a scripted mock institution or a dropped message. The demo API and the terminal share one engine.

Harder: machines have to be written as input-to-output functions. The terminal's numbers must always be presented as a floor.

Committed: `caudal` is the binary's name, every operation is scriptable, and the in-process network is deterministic.
