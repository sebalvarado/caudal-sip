# 0018. The same nodes run in one process or many; only transport, clock, and storage change

- **Status**: Accepted
- **Date**: 2026-10-10
- **Deciders**: Founding maintainer
- **Issue**: [#23](https://github.com/sebalvarado/caudal-sip/issues/23), part of [#17](https://github.com/sebalvarado/caudal-sip/issues/17)
- **Refines**: [0012](0012-crate-boundaries.md) rule 2, on where ISO 20022 translation lives, and adds the `south-api` crate

## Context

The `caudal` terminal runs `sip`, two redes, and mock institutions in one process. Production runs each as its own service. If the two paths share no code, the terminal proves nothing ([`local-stack.md`](../../development/local-stack.md), rule 3). [ADR-0012](0012-crate-boundaries.md) left two things to this record: where translation between ISO 20022 and domain commands lives, and how `payments` and `msgplane` compose into the router of each app. [`red.md`](../red.md) bans store-and-forward on the payment path and requires a pluggable transport, because the BCE's connection standards are unpublished.

## Decision

**A node's behavior is a synchronous state machine in the domain.** `payments`, and `directory` for llave flows, model each role (SIP, red, ordering or beneficiary integrante) as a machine that takes an input and returns outputs. The inputs are a domain message received, a ledger outcome, or a deadline reached. The outputs are messages to send, ledger commands, deadlines to set, and events. Like every domain function, it takes `now` and ids as arguments ([ADR-0017](0017-time-calendar-identifiers.md)). Every catálogo rule about a payment's path lives here, once, for every profile.

**`msgplane` moves signed bytes and keeps time.** It owns the `Transport` trait: send a signed message to a peer named by BIC with a deadline and get the peer's reply or an unknown outcome, and serve inbound messages. It also owns signing and verification, and a `Clock` with timers. Transport has two implementations. The in-memory one uses channels inside one process. The network one uses mTLS by default and is configuration-replaceable until the BCE publishes its standards ([`technical-requirements.md`](../technical-requirements.md)). `msgplane` knows nothing of payments or ISO 20022. There is no queue or retry store in it. A send either gets an answer before its deadline or reports an unknown outcome, and the machine decides what that means. An unknown outcome never becomes a void ([ADR-0013](0013-ledger-is-the-aggregate.md)).

**`south-api` is the scheme-facing adapter**, the mandated half of [ADR-0004](0004-two-api-surfaces.md), next to `north-api`. It translates between `iso20022` messages and the machines' inputs and outputs, and it runs the loop: receive from the transport, translate, feed the machine, execute its outputs against the ledger port and the transport, and arm its deadlines on the clock. It depends on `msgplane`, `iso20022`, and the domain crates. It is the only crate that knows both the wire and the domain, so the catálogo técnico, when it arrives, changes `iso20022` and this translation and nothing else.

**Apps choose; they do not decide.** `sip`, `red`, and `gateway` each build `south-api` with a role, a transport, storage, and a clock. In production that means the network transport, the system clock, and Postgres. The terminal builds every node in one Tokio runtime with the in-memory transport and one simulated clock. That clock jumps straight to the next deadline whenever every node is idle, so a payroll day runs in seconds. The mock institutions are `gateway` with a simulated core-banking adapter, scripted to accept, reject with a Tabla 12 code, delay, or drop. Messages between in-process nodes are still serialized and signed with locally generated keys, so the terminal runs the real codec and signature path.

**Deadlines are configuration, nested.** Each stage's timeout is set per counterparty (§5, rule 13) and is shorter than the deadline of the stage above it. The defaults come from the 20-second limit ([`red.md`](../red.md), open question 4). A node refuses to start with deadlines that do not nest.

**What the terminal measures is a floor.** It reports per-hop computation (codec, signature, domain rules, and the ledger commit when Postgres is used) and the end-to-end time of each scenario. It leaves out the network, TLS handshakes, the institutions' core banking, and the BCE's infrastructure. Its numbers are evidence of what the software costs, not a production latency claim. A published benchmark runs on the network transport across real hosts. The terminal's own ADR ([#11](https://github.com/sebalvarado/caudal-sip/issues/11)) builds on this.

## Alternatives considered

- **Translation inside `msgplane`.** Rejected. `msgplane` would then depend on every domain crate, and the transport could no longer be tested without the domain.
- **Each app writes its own loop.** Rejected. Three loops would drift on deadline handling, which is where the catálogo's four failure cases live. [ADR-0012](0012-crate-boundaries.md) also keeps apps as wiring.
- **An in-memory shortcut that passes domain values between nodes without serializing.** Faster, but the terminal would skip the code most likely to break, and its numbers would flatter us.
- **A message broker between nodes.** Rejected. It is store-and-forward on the payment path ([`red.md`](../red.md)), and an extra dependency for a cooperativa to operate.

## Consequences

Easier: the terminal, the sandbox, and production run the same machines and loop. Failure injection is a scripted mock institution or a transport that drops, not a separate simulator.

Harder: domain machines have to be written as input-to-output functions, not as straight-line async code. `south-api` is a large crate, and it may later split by flow (payments and llaves) without changing this boundary.

Committed: the `south-api` crate joins the adapter layer of [ADR-0012](0012-crate-boundaries.md), depending on `iso20022`, the domain crates, and, by an edge `cargo xtask arch` allows by name, `msgplane` ([ADR-0016](0016-data-ownership-and-audit.md)). `msgplane` has no domain dependency and no store-and-forward.
