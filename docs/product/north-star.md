# North Star

> **Read this before scoping any component, RFC, or outreach push.** It defines where the project is going and how it earns the right to get there. What exists *today* is in the repo root [`README.md`](../../README.md); how the system is built is in [`../architecture/blueprint.md`](../architecture/blueprint.md).
>
> **Owner:** Sebastián · **Adopted:** 2026-08-05

## The north star

> **Any Ecuadorian institution — from the largest bank to a cooperativa with four people in IT — can move money in real time, to anyone, by running software it did not have to pay for and can read line by line.**

Success is not this repository having stars. Success is a payment leaving a cooperativa in Loja and landing in a bank account in Guayaquil in under five seconds, on a Sunday, for free, through code that anyone can audit.

## The trust ladder

Each rung earns the right to build the next. Skipping a rung breaks the project, because in payments credibility is the scarce resource, not code.

1. **Explain it truthfully.** Be the clearest public account of what Ecuador's regulation actually requires — better than anything else available in Spanish. Cost of entry: research, not permission. *(Where we are.)*
2. **Simulate it end to end.** A full network in a box: two redes, a SIP-like integrator, mock institutions, directory, settlement. Anyone can watch a payment clear in seconds on their laptop. This is what turns "this is very hard" into a claim someone has to defend.
3. **Make one real institution compliant.** A cooperativa or its core-banking vendor passes BCE technical qualification using our gateway and certification kit. One institution in production beats a hundred stars.
4. **Become the default way institutions connect.** Enough participants and vendors run the stack that it is the obvious choice, and the hosted option exists for those without infrastructure teams.
5. **Be adopted as public infrastructure.** The BCE or the redes use the sandbox as the official test environment, the certification kit as the qualification tool, or the reference implementation itself — the way Banco Central do Brasil publishes Pix specs on GitHub.

Rungs 1 and 2 are ours alone to climb; no one can block them. Rungs 3 to 5 require other people to say yes, which is why [`adoption.md`](adoption.md) exists.

## What makes this durable

Not any single component. Three things, in order:

1. **The developer experience of the North API.** The wire protocol is mandated and identical for everyone; the experience of integrating with it is not regulated at all. That gap is the whole opportunity.
2. **Being the reference everyone tests against.** Whoever writes the conformance tests defines what "correct" means in practice. That is influence no procurement process grants.
3. **Documentation in Spanish that treats the reader as an engineer, not a customer.** Nobody else is doing this, and it is what recruits contributors and reassures regulators.

## Sequencing rules

- **Read before write.** Every component ships read/validate/simulate paths before it touches money. The settlement ledger is simulated until an institution needs it real.
- **Standards before opinions.** Where the BCE has specified something, we implement it exactly, even when we would have designed it differently. Where it is silent, we propose via [RFC](../rfcs/README.md) and mark it **[proposal]** — never quietly invent scheme behavior.
- **No component without a user.** Each one ships against a named institution, contributor, or demo that needs it. The full topology in the blueprint is a map, not a build order.
- **Nothing ships that a four-person IT department cannot run.** A single binary they can operate beats a distributed system they cannot. This is the constraint that kills most payment platforms in the long tail.
- **Advocacy is a deliverable.** Running code that quantifies a policy question (settlement risk, fee-free P2P, latency) is worth more than a position paper, and produces both.

## Guardrails — what the north star does not authorize

- No custody, wallet, KYC/AML engine, or anything that puts us between a customer and their money. We ship hooks, not liability.
- No blockchain, ever, in this rail. Settlement is central bank money; saying otherwise costs the credibility everything else depends on.
- No competing with the BCE. The framing is always *complement*: the participant-side tooling and the education layer around the mandate.
- No paid tier inside the core. Hosting and operation can be sold ([`../../GOVERNANCE.md`](../../GOVERNANCE.md)); scheme compliance never gets paywalled.
- No feature that only serves an institution outside the [stakeholder map](stakeholders.md) — especially bespoke integration work dressed up as a contribution.

## Changelog

- 2026-08-05 — Initial version, adopted alongside the stakeholder and adoption docs.
