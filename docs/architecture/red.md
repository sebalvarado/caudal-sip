# The `red` binary: architecture of the network profile

What a *red de pagos* has to do, what the software has to be, and what running it does and does not give you.

This document is deliberately light on regulatory citations. The paragraph-by-paragraph sourcing lives in [`../research/catalogo-operacional.md`](../research/catalogo-operacional.md); here the goal is that someone who has never read the norm can understand the system.

## What a red actually is

A *red de pagos* is a licensed company that sits between financial institutions and the country's central payment system. It is not a network in the cabling sense and not a VPN. The closest analogy is "the Visa network": a company whose product is routing, not wire.

Four exist today — BANRED, COONECTA, ETIKO's and DEUNA — and the rules allow more to be authorized.

The scheme requires this middle layer. An institution never talks to the central system directly, and the central system never talks to an institution. Every message goes institution → red → central system → red → institution. A red is therefore a mandatory hop in both directions, which makes it a mandatory single point of failure for every institution that uses it. That fact shapes most of the engineering below.

A red has four jobs:

**Route payments.** Take an order from one of its member institutions, check it, sign it, pass it up; take an order coming down, deliver it to the right member, collect the answer, pass it back. Inside a very short time budget.

**Run a directory.** The country keeps a central register of payment keys — a phone number, a national ID, an email, an alias — but that central register holds only enough to guarantee that a key exists exactly once nationally and to know which red owns it. The actual account behind the key lives in the red's own directory. So when anyone in the country resolves a key belonging to one of this red's members, the answer comes from here.

**Certify its members.** Before an institution can operate, its red has to technically qualify it. The red is the gatekeeper for everyone beneath it. This is a regulated responsibility, not a courtesy, and it means a red needs an onboarding and testing workflow, not just a runtime.

**Reconcile.** When a payment ends in an unknown state — and the design guarantees this will happen — resolving it is the red's job, using data from the central system, and it has to be finished the same day.

## The two faces

Everything in the design falls out of the fact that a red has a north face and a south face, and they are not symmetric.

The **north face** talks to the central system. One counterparty, one protocol, no negotiation: signed ISO 20022 XML over whatever transport the BCE ends up mandating. We do not get to design this side, only implement it.

The **south face** talks to member institutions. Many counterparties of wildly different sophistication — a large bank with a payments team and a segment-4 cooperativa whose core banking is a vendor product. The norm does not specify this side at all. It is the red's own business decision.

That asymmetry is the most important thing in this document, because it is where a red running our software gets to be better than a red that isn't. A red can require its members to speak full ISO 20022 XML — which is what will happen by default, and which pushes the integration cost onto four hundred institutions that mostly cannot pay it. Or it can offer a simple JSON API and take that cost on itself once. We ship both: XML for the institutions that want it, and the same North API the gateway exposes for everyone else. A cooperativa should be able to join a red by making HTTP calls with a JSON body.

## What a red owns, and what it does not

A red **does not hold customer money and does not hold customer accounts**. It never has a balance for a person. Everything it stores about a payment is evidence and state, not value.

Whether it holds *obligations* is the one genuinely open question in this design, and it turns on a single point: payments between two institutions that both belong to the same red do not go to the central system. They are settled inside the red. That means a red is running its own small clearing arrangement for intra-red traffic, and if so it needs a ledger — accounts per member, balanced entries, positions, a settlement instruction at the end of the cycle.

If that reading is right, the good news is that we already have it: the `ledger` and `clearing` packages built for the central system are the same code with a smaller chart of accounts and a different settlement counterparty. Nothing new is needed. But how a red's own intra-red settlement reaches central bank money is not documented anywhere we have found, and it is on the list of questions for the BCE.

Everything else a red owns is state, not money:

## Domain model

Eleven things, grouped by what they are for.

### Membership

| Entity | What it is |
|---|---|
| `member` | An institution that takes service from this red: BIC, legal identity, type and segment, status, operational contacts |
| `service_enrollment` | Which services this member takes *here* — keys, credentials, or both. Reported upward so the central system knows where to route |
| `certification` | The qualification this red performed on this member: which test suite, which version, the results, who signed off, when it expires |
| `connection` | How to reach a member and how it reaches us: endpoints, certificates, timeouts, health |

The rule that makes enrollment interesting: an institution may take the key service from one red and the credential service from another, but never the same service from two redes at once. The red can only assert its half of that — global uniqueness is enforced centrally — so the local model has to treat enrollment as a request that can be refused, not as a local write.

Certification deserves more weight than it usually gets in an architecture document. It is the red's regulated obligation, it is the thing that decides whether a cooperativa goes live in three weeks or nine months, and it is the natural home for the conformance kit. A red running our software can hand a prospective member a command that produces a pass/fail report, instead of scheduling six weeks of coordinated manual testing.

### Directory

| Entity | What it is |
|---|---|
| `llave` | The public identifier a payer types: value, type, owning member, status |
| `credencial` | What it resolves to: holder document type and number, name, account number, account type, institution BIC |
| `reservation` | An in-flight registration, held between the central reservation and the member's confirmation |
| `sync_state` | Per key: last confirmed agreement with the central register, and any divergence found |

The directory is the only part of a red with hard uniqueness requirements, and registration is a two-phase operation — the red reserves, the central register checks nationally and reserves or refuses, then the red and the member confirm. The dangerous case is a failure partway through: a naive retry after a timeout must not leave a key reserved to nobody. This is the single most subtle piece of code in the whole binary and it should be written with a state machine and property tests, not with retries and hope.

Divergence between the red's directory and the central register is a defined condition with its own state, not an exception. Keys that disagree get parked and worked, the same way payments do.

### Payments

| Entity | What it is |
|---|---|
| `payment_journal` | One row per payment this red touched, in either direction: both BICs, amount, direction, current state, a timestamp per hop, reason code |
| `message` | Every wire message in and out: type, direction, payload hash, signature reference, correlation id |
| `reconciliation_item` | A payment whose outcome is unknown, with its deadline, its owner and its resolution |

The journal is not a ledger. It records that a payment happened and what state it reached, so that months later somebody can be told exactly what this red did and when. Rows are immutable except for state and timestamps.

Reconciliation items exist because of a specific and unavoidable failure mode: there are cases where the payment is already irrevocable and the sending side does not know it yet. Turning that timeout into a rejection creates double credits. So an unknown outcome stays unknown, becomes a work item, and gets a final state the same day.

### Everything else

| Entity | What it is |
|---|---|
| `audit_record` | Append-only, spanning all of the above, written in the same transaction as the change it describes |
| `operator` / `role` | Console users, scoped so that a member's staff can see their own institution and nothing else |

## The codebase

> **Crate names follow [ADR-0012](adr/0012-crate-boundaries.md).** The sketch below predates it: `cmd/red/` is `apps/red/`, `membership/` lives in the `registry` crate, `directory/` is the `directory` crate, and `routing/` is `payments` composed with `msgplane` inside `apps/red`.

`red` is not a separate program. It is the same repository and the same packages as `sip`, wired into a different binary. That is the decision that makes the project buildable by a small team, and it is worth being concrete about how much is genuinely shared.

```
cmd/red/            Wiring: config, profile, lifecycle

Shared with sip and gateway
  iso20022/         Message structs, codecs, validation
  msgplane/         Transport, signing, verification, idempotency, retry,
                    timeout enforcement. Transport is an interface because
                    we do not yet know what the BCE will mandate.
  ledger/           Accounts, balanced entries, no-overdraft invariant
  clearing/         Netting and settlement instructions
  audit/            The append-only log
  console/          Operator UI framework, role-scoped

Specific to red
  membership/       Members, enrollment, the certification workflow
  directory/        The distributed directory node: two-phase registration,
                    resolution, synchronization and divergence handling
  routing/          The payment path: member ↔ SIP forwarding, intra-red
                    short-circuit, per-hop deadlines
  reconciliation/   The unknown-outcome work queue and its daily close
```

Four properties of this arrangement matter more than the file layout.

**The routing core keeps no session state that cannot be rebuilt.** It journals every message, but it holds nothing in memory that a restart loses. Two nodes behind a load balancer, no stickiness, no coordination.

**Store-and-forward is banned on the payment path.** Twenty seconds end to end means the red forwards synchronously with hard deadlines, and a queue with retry semantics in the middle of that path is a correctness bug dressed as resilience. Queues are for outbound notifications to members, reporting and reconciliation — never for a payment in flight.

**The directory is the only strongly consistent store**, and it is read-heavy with serialized writes per key. Reads cache; writes do not.

**Transport is pluggable and nothing above it may assume otherwise.** The BCE has assigned itself the job of publishing connection standards and has not done it. Until then, no component hard-codes a certificate authority, an addressing scheme or the assumption that the peer is on the public internet.

## The flows

Four paths carry essentially all the behaviour.

**Outbound payment.** A member sends an order. The red authenticates it, validates the message, checks the member is live and not suspended, signs, forwards north, waits with a deadline, receives the confirmation or rejection, journals both, returns the answer. If the deadline passes, the payment does not become a rejection — it becomes a reconciliation item.

**Inbound payment.** The central system sends an order for one of this red's members. The red validates and routes it to that member, waits with a deadline shorter than its own, and returns the answer north. If the member does not answer in time, the red still has to say something, and what it says determines whether money moves. This is the hop where the four documented failure cases live.

**Intra-red payment.** Both parties are this red's members. It never leaves. The red resolves, routes, records the obligation between the two members, and settles it in its own cycle. Faster and cheaper than the inter-red path, which is a real competitive property for a red with many members.

**Key operations.** Register, cancel, suspend, re-enable, resolve. Registration is the two-phase reservation described above; resolution comes in from the central system for keys this red owns and is answered from the local directory.

## What the application has to do

Ten functional requirements. If the binary does these, it is a red.

1. Accept payment orders from its members, validate and sign them, forward them, and return the answer inside the time budget.
2. Accept payment orders for its members from the central system, deliver them, and return the answer.
3. Complete payments between two of its own members without leaving the red, and settle them.
4. Register, cancel, suspend and re-enable payment keys for its members, coordinating national uniqueness with the central register, without ever leaving an orphaned reservation.
5. Answer resolution requests for keys it owns, from its own directory, and keep that directory in agreement with the central register — detecting, flagging and resolving divergence rather than silently drifting.
6. Onboard a member: enroll its services, run it through certification, record the result, and turn it live or refuse.
7. Run the unknown-outcome queue and empty it every day before the close.
8. Give operators a console that answers "where is this payment", "is every member connected", "what is expiring", "what is unresolved" without anyone opening a terminal.
9. Produce the reports the BCE expects and the reports a member expects about its own traffic.
10. Reconstruct any payment, years later, from the journal and the message log alone.

## What the application has to be

**Latency.** The red is two of the five hops on every inter-red payment, and it is a hop on the way out and again on the way back. Budget is 100 ms per traversal. The rule is simply that the red must never be the reason a payment approaches the twenty-second cliff, and since none of its work on the hot path requires an external call, that is comfortable.

**Availability.** This is the requirement that is genuinely harder for `red` than for anything else we build. A gateway going down takes one institution offline; a red going down takes every one of its members offline simultaneously, and their customers will not experience that as "the red is down" — they will experience it as "my bank is broken". So: no maintenance windows, rolling upgrades, backward-compatible migrations, at least two active nodes, and a documented path for a member to be moved if a red fails for an extended period. Aim above 99.95%, and treat the number as a floor rather than a goal.

**Correct behaviour when things break.** More important than availability, and much easier to get wrong. Never convert an unknown outcome into a rejection. Never release a reservation on a timeout. Deduplicate on the payment id, which carries the originating institution's BIC, because retries are part of the protocol and will arrive. Every one of these has a test derived from the documented failure cases, and those tests are the ones worth writing first.

**Capacity, stated plainly so nobody over-builds.** A country three times Ecuador's size peaks around sixty transactions per second nationally. A red with a third of Ecuador's traffic is well under twenty on an average day. This is one server with a warm standby. The engineering problem here is correctness and uptime, not throughput, and any design that trades correctness for scale is solving a problem this system does not have.

**Operability by a small team.** A red is a company with maybe a dozen people in IT. One binary, one database, configuration that fails loudly at startup rather than running ambiguously, runbooks in Spanish next to the code.

**Data protection, which is a bigger surface here than anywhere else.** The payment profile carries the full identity of both customers in every message, and a red sees all of it for every payment its members send or receive. It is the largest concentration of personal financial data in the scheme after the institutions themselves. That means encryption at rest as a default the operator cannot forget, masked output in the console and in resolution responses, no personal data in logs or metrics, and strict per-member isolation enforced in the database rather than by application filters — a member must never be able to see another member's traffic, and the red's own staff should need a reason to see a customer's identity.

**Auditability.** Append-only, written in the same transaction as the change, hash-chained, and sufficient to answer a regulator without touching production.

## Is this everything a red needs?

The goal is yes for the software, and the honest answer is that software is roughly half of what it takes to be a red.

**What the binary gives you.** Everything technical: routing both directions, the directory and its synchronization, key lifecycle, intra-red settlement, member onboarding and certification, the reconciliation queue, the operator console, reporting, and the audit trail. Deploy it, configure your members, connect it upstream, and you are technically a functioning red de pagos.

**What it cannot give you.** Five things, and none of them are code:

- **Authorization.** Being a red is a licensed activity. The BCE authorizes you; a repository does not.
- **Connectivity and identity.** The telecom links, the certificates, the BIC codes — all issued by the BCE, on its terms, on its schedule.
- **The settlement relationship.** Accounts, arrangements, and the legal ability to move central bank money.
- **A rulebook.** Contracts with your members, fee schedules, liability allocation, dispute handling, service levels. The software enforces whatever you decide; it will not decide it.
- **Operations.** Twenty-four hour staffing, continuity plans, disaster recovery drills, security monitoring, incident response. We ship the hooks, the health checks, the runbooks and sensible defaults; somebody still has to be awake at three in the morning on a Sunday.

**Why this is still the interesting outcome.** Today the technical cost of becoming a red is the barrier — it is a real payment switch, a national-uniqueness directory, a certification programme and a 24/7 operation, which is why there are four of them and why three are incumbents. If that cost drops to deploying a binary, the remaining barrier is purely regulatory, and the question "should a federation of cooperativas be allowed to run its own red?" becomes a policy conversation instead of a budget conversation.

That is worth saying out loud in public, because it is the most direct competitive effect this project can have on the scheme, and it does not require the BCE to adopt a single line of our code.

## What we can build now

Most of it. The parts that depend on the unpublished field-level technical catalogue are the wire formats on the north face — the exact XML the red sends and receives. Everything else is specifiable today: the domain model, the directory and its two-phase registration, membership and certification, the reconciliation queue, intra-red routing and settlement, the console, and the whole south face, which is ours to design anyway.

The practical sequence is to build against the ledger and clearing work already underway, keep the north face behind an interface with a stub implementation and a simulated counterparty in the sandbox, and swap in the real message profile when it is published. That is the same discipline the transport requirement already imposes, applied one layer up.

## Open questions

1. **Intra-red settlement.** Does a red settle its own internal traffic, and if so, how does that reach central bank money? This decides whether `red` includes a ledger, and it is the largest structural unknown in this document.
2. **What the central register actually stores.** The split between it and the distributed directory is described in principle but not field by field, and it determines how much a resolution can be answered locally.
3. **Divergence resolution.** When the two directories disagree about a key, who wins, and how is it repaired?
4. **Timeout values.** Every stage has a parameterized deadline and none of the parameters are published. Ours are configurable with defaults derived from the twenty-second ceiling.
5. **Member failover.** If a red is down for a day, what recourse does a member have? Nothing in what we have read addresses it, and it is the natural consequence of making the red a mandatory hop.
6. **Certification scope.** How much of a member's qualification is the red's own judgement versus a standard the BCE defines? This changes whether the conformance kit is a shared national artifact or one red's internal tool.
