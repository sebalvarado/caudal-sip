# 0017. Time is an argument, the calendar is data, and identifiers come from the catálogo or the edge

- **Status**: Accepted
- **Date**: 2026-10-10
- **Deciders**: Founding maintainer
- **Issue**: [#22](https://github.com/sebalvarado/caudal-sip/issues/22), part of [#17](https://github.com/sebalvarado/caudal-sip/issues/17)

## Context

The scheme runs on time. A payment has 60 seconds after resolution and 20 seconds end to end (Catálogo Operacional §5, rules 3–4). Every stage has a timeout (§5, rule 13). Payments ordered outside operating hours settle on the next business day (§9, rule 4). Reconciliation closes at 18:00 (§13.2, note 1). The risk ceiling follows a Thursday-to-Wednesday week (§10, Tabla 20). The `caudal` terminal has to run a payroll day in seconds, and tests have to reach 17:59:59 without waiting for it. Every one of those values except 60 s, 20 s, and 18:00 is "parametrizable" and unpublished.

Identifiers have the same split: some are fixed by the catálogo, and some are ours. A domain that generates its own ids at random cannot replay a run exactly.

## Decision

**Domain operations take `now` as an argument.** Every decision function receives `now: Timestamp`, and any id it needs to mint, from its caller. Decision functions never read a clock or a random source. Apps and the message plane hold the clock: a system clock in production, and a simulated one in the terminal and in tests. The only other holders are the implementations of a storage port, `MemLedger` included, which are given a time source when they are built so they can stamp commits ([ADR-0015](0015-ledger-interface.md)). An instant is a `jiff::Timestamp`, in UTC. `jiff` is the one time crate in the workspace.

**Scheme time is Ecuador's continental time, UTC−05:00, with no daylight saving.** **[assumption]**: the catálogo gives every hour without a zone. Operating hours, cut-offs, the reconciliation close, the risk week, and the date in a payment id are civil times in that zone. The kernel converts between `Timestamp` and scheme time in one place.

**The business calendar is a kernel type, built from data.** `BusinessCalendar` holds holidays as dates, the operating window (§5, rule 8), the settlement times (§9, rule 2), and the reconciliation close (default 18:00, §13.2). Its pure queries answer: is this a business day, what is the next one, which business date a payment's malla belongs to (§9, rule 4), and when that date's close falls. `clearing`, `reconciliation`, `payments`, and `risk` all need these answers, so the type sits in `types` ([ADR-0014](0014-kernel-types.md)). The risk week is built on it, inside `risk`. The catálogo publishes no value for holidays, operating hours, or settlement times, so they have no defaults: a production node without them refuses to start. The sandbox and the terminal ship a fictional calendar.

**Who creates each identifier:**

- **Payment id**: created by the ordering integrante's software, which in this project means `gateway` and the terminal's mock institutions. `sip` and `red` only validate it. The format is the catálogo's: the ordering BIC, then date and time as `DDMMAAAAHHMMSS` in scheme time, as Tabla 14's example shows, then a hash suffix filling the code to 35 characters (§7). The suffix is the uppercase base32 hash of a fresh UUIDv7. A collision is refused by the ledger as a `Conflict`, never booked as a second payment ([ADR-0015](0015-ledger-interface.md)).
- **Message ids** (`GrpHdr/MsgId`, `AppHdr/BizMsgIdr`): created by the sending node, as a UUIDv7 written as 32 hex characters **[assumption]** until the catálogo técnico gives a format. The SIP enforces uniqueness with `DU01` (Tabla 12).
- **Internal ids** (events, audit records, reconciliation items, settlement instructions): UUIDv7, created at the edge and passed into the domain like `now`. With a seeded generator and a simulated clock, a terminal run replays exactly.
- **Ledger entries need no new id**. A reservation's entries are identified by the payment id and the phase. A transfer carries an id from its caller, such as the malla's settlement instruction, so a replay cannot post twice ([`posting-patterns.md`](../posting-patterns.md) §1.3).

Deadlines and per-stage timeouts belong to the message plane ([ADR-0018](0018-composition-and-message-plane.md)).

## Alternatives considered

- **An injected `Clock` trait in domain crates.** Testable too, but any domain function could then read the time, and a test would have to set a clock up instead of passing a value. Rejected: an argument shows in every signature which operations depend on time.
- **`chrono` or `time`.** Both work. `jiff` models instants and civil times as separate types, which matches the split between UTC instants and scheme time. Pick one and keep it.
- **ULID for internal ids.** Equivalent ordering properties, but UUIDv7 is the IETF standard and Postgres has a native type for it.
- **A sequence number as the payment id suffix.** Allowed by the catálogo, but it needs coordination between gateway nodes. A hash needs none.

## Consequences

Easier: every cut-off, close, and weekly cycle is a unit test with a literal timestamp. The terminal can compress a day. Any run with a fixed seed is reproducible.

Harder: signatures carry `now` and ids through every layer. Holiday data has to be maintained by the operator every year.

Committed: no decision function reads a clock or random source. Every unpublished timing value is configuration that fails loudly when missing. Payment ids follow Tabla 14.
