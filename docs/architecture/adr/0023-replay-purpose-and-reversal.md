# 0023. A repeated command reports a replay, and a transfer is reversed by mirroring every leg

- **Status**: Accepted
- **Date**: 2026-10-13
- **Deciders**: Founding maintainer
- **Refines**: [0013](0013-ledger-is-the-aggregate.md), [0015](0015-ledger-interface.md)

## Context

`Reserve`, `Post`, `Void`, and `Transfer` are idempotent: a repeat returns the committed outcome and appends nothing. The rule core already knew which of the two had happened, because a repeat carries no new draft. The port threw that away and returned only the reservation or the entry id. A caller that retries after an unknown outcome cannot tell a fresh commit from one the books already held.

Transfers had the same problem in another form. A ceiling grant, a malla settlement, a suspense memo, and a hand-built correction were all `Transferred`, distinguished only by a UUID and a message hash. The one test that undid a payment built the mirror itself and dropped the headroom leg. That is a posting the property tests will accept, and it is the wrong undo for a posted payment: `ACSP` is irrevocable.

## Decision

Every idempotent command returns `Applied { value, effect }`. `Effect::Appended` means this call wrote an entry. `Effect::Replay` means the books already held the command and nothing was appended. A replay does not mean the caller has already sent its message. The payment process still performs the side effects it has not recorded. `Unknown` is still never a refusal, and it is still answered by repeating the command.

A transfer carries a closed purpose: `headroom`, `settlement`, `suspense`, or `correction`. The same id with a different purpose or different legs is a conflict. The cause, the event and the message hash, is evidence of this attempt and is not compared, so a retry from a new event still replays.

`reverse(transfer_id)` appends one entry whose legs are the mirror of every leg on that transfer, including headroom. The original stays. A second reverse of the same id is a replay. An id that is not a transfer is `NotTransferred`. A reversal that would take a constrained balance below zero is `InsufficientHeadroom`, and nothing is appended. A payment reservation is not a transfer, so this command cannot void or unpost one. Clearing treats a settlement as closed only while it has not been reversed, so a reversed malla is open again.

## Alternatives considered

- **Put `replayed` on the stored reservation.** Rejected. It describes this call, not the hold. A stored flag would become part of the book of record.
- **Suppress the payment's messages when the ledger reports a replay.** Rejected. The network retries an unknown ledger call before the message is sent. The attempt that committed and the attempt that observed it are the same step. Swallowing the message there would drop a payment that had reserved and never left.
- **Configurable posting routes, chosen at runtime.** Rejected. The purpose is a closed label so the log can be read. The legs of a grant, a settlement, and a suspense memo are still computed by the crate that owns that scheme event. A route table would let a deployment post a pattern the property tests never saw.
- **Keep letting callers build the mirror.** Rejected. The test that did so omitted headroom. The ledger is the place that knows every leg.

## Consequences

Easier: a retry is visible, a settlement is not mistaken for a correction, and undoing a grant or a malla cannot drop a leg. Harder: every caller of the port reads `.value` or `.effect`, and Postgres grows a purpose column and a `reversed` kind. Committed: `reverse` never targets a payment; a posted payment is undone only by the rules that already refuse `Void`.
