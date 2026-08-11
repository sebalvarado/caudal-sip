# Adoption: how this gets buy-in

The hard problem is not the software. It is that a payment system is worthless until several institutions that do not know us agree to use it. This document is about that: what each group gets in exchange for saying yes, what they will object to, and what evidence changes their mind.

The sequence of *actions* is in [`../plan/action-plan.md`](../plan/action-plan.md); who these people are is in [`stakeholders.md`](stakeholders.md). This is the "why would they say yes" layer between them.

## The one principle

**Artifacts open doors; meetings do not.** We have no institutional relationships and no permission to ask for. Every door in this project opens the same way: publish something so obviously useful that not engaging looks careless. A cold email asking for a meeting gets ignored. A cold email linking to a running simulation of Ecuador's payment network, plus the clearest existing explanation of the institution's own obligations, gets forwarded internally.

This is also the risk-reduction strategy. An institution can evaluate us without talking to us, which is exactly how conservative organizations prefer to evaluate anything.

## The adoption ladder

Each rung is a smaller ask than the one before it, and each produces the evidence that makes the next one credible.

| Rung | The ask | What it costs them | What we get |
|---|---|---|---|
| 1 | Read the research | 20 minutes | Credibility, and often the first reply |
| 2 | Run the sandbox | An afternoon for one engineer | They now know it works; the internal conversation changes |
| 3 | Use the message library in their own build | A dependency, no commitment | A real user, real bug reports, and a reference |
| 4 | Run the gateway in their test environment | A VM and a week | A design partner and a genuine case study |
| 5 | Pass qualification with our certification kit | Their existing compliance work | Proof that the path works end to end |
| 6 | Production | Real risk | The rung everything else exists to reach |
| 7 | Contribute back, or host for their members | Ongoing | The project stops depending on us |

Never ask for rung 6 first. The most common way this project could fail is proposing production to someone who has not yet spent twenty minutes at rung 1.

## What each group actually gets

Buy-in is a trade. Stated bluntly, per group:

**Institutions facing the deadline** get a dated obligation met with the team they already have, and a second option that changes what a vendor can charge them. The pitch is never "open-source payments infrastructure" — it is *"you will make the date, and you can read every line of what you are deploying."* For small cooperativas the honest framing is starker: compliance is an existential cost, and free, operable software is the difference between participating in the national rail and being cut out of it.

**Core-banking vendors** get to ship compliance to dozens of clients without building ISO 20022 themselves, and to resell operation on top. Apache-2.0 makes us a component in their product rather than a competitor for their customer, which is the only framing under which they cooperate. One vendor is worth more than twenty direct conversations.

**Federations and cajas centrales** get to protect their members from a cost shock while keeping the relationship — they can operate the multi-tenant gateway themselves. This matters because trust in the cooperativa sector is relational: a member institution will accept from its federation what it would never accept from a startup.

**Redes de pagos** get their members ready on schedule and fewer support calls from unprepared ones. Some will see us as competing with their integration services; the answer is that we serve the long tail they cannot profitably serve, and the tail is most of the institutions.

**The BCE** gets a public test environment it did not have to build, a pre-certification tool that reduces the qualification burden on its own staff, and technical proposals for what the catálogo does not yet cover — offered, never demanded. The precedent to invoke every time is Banco Central do Brasil publishing Pix specifications openly on GitHub. What it must never hear from us is anything resembling a parallel rail.

**Multilaterals** get a reusable digital public good in a country they already advise, which is precisely what their mandates reward. A warm introduction from one of them is worth ten cold emails.

**Contributors** get a real payments system to learn on, in public, in Spanish, with their name on the infrastructure of their own country. For the Ecuadorian engineering diaspora that motivation is unusually strong and almost entirely untapped.

## Objections, and the honest answers

These will come. Rehearsed, short answers matter more than being right.

**"Open source is unsafe — you are handing attackers a map."** The systems protecting the most money on earth are open: TLS, AES, Linux, and Pix's own specifications. Secrecy protects keys, not designs; a design whose safety depends on nobody reading it has not been reviewed, only unexamined. The long version is in the [manifesto](../../MANIFIESTO.md).

**"Who supports this at 2am?"** Today, honestly: nobody, and we say so. That is exactly why the hosted option exists, why federations and vendors are the intended operators, and why the software is built so a four-person team can run it. Pretending we offer enterprise support we do not have would end the relationship at the first incident.

**"You are competing with the BCE."** We implement the BCE's standard and build nothing it has claimed. The non-affiliation disclaimer is on every artifact. The value we add is on the participant side and in education — the part nobody is funding.

**"Who is behind this, and what happens when you lose interest?"** Apache-2.0 with no CLA, all decisions recorded in [ADRs](../architecture/adr/README.md) and [RFCs](../rfcs/README.md), governance documented and heading toward a neutral entity ([`../../GOVERNANCE.md`](../../GOVERNANCE.md)). An institution can fork and continue without us. Being forkable is the answer to key-person risk, and saying it out loud is more reassuring than claiming permanence.

**"We already have a vendor quote."** Then the certification kit is free and useful to you regardless, and the comparison is easy to make. Institutions that start with rung 1 in order to check a vendor's claims are among our best entry points.

**"It cannot be that easy — this takes years and costs millions."** This is the objection the sandbox exists to answer, and the only correct response is a demonstration plus published load-test numbers on ordinary hardware.

## The proof artifacts

Adoption depends on a small number of things existing. In priority order:

1. **A two-minute video of a payment clearing** in the simulated network, with the timer visible. The single highest-leverage asset the project can produce.
2. **The research document** ([`../research/landscape.md`](../research/landscape.md)) — already better than any public account of Ecuador's framework, which is why it opens doors before any code exists.
3. **A conformance report** an institution can carry into a qualification conversation.
4. **Published load tests** that make the cost-and-timeline argument concrete.
5. **A public paper trail with the BCE** — records requests answered, forum participation requested in writing. Legitimacy accumulates in public and cannot be granted retroactively.
6. **One named institution in production.** Everything above is preamble to this.

## How we talk about it

Spanish first, engineer to engineer, per [`../spanish-style.md`](../spanish-style.md). Never claim endorsement, affiliation, or a customer we do not have. Never disparage the BCE, a red, or a vendor by name — the ecosystem is small, and this project needs almost everyone in it eventually. Publish weekly whether or not there is a launch, because consistency is what makes a one-person project look institutional. And when we are wrong about a regulatory detail, correct it publicly and quickly; being visibly careful about facts is the entire basis on which anyone will trust the code.

## What would tell us this is working

Leading indicators, in the order they should appear: someone we have never met runs the sandbox and files an issue; a core-banking vendor asks a technical question; a federation asks for a presentation; the BCE answers a records request substantively; an institution asks about the certification kit unprompted; a contributor from outside Ecuador submits a fix. Stars and traffic are not on this list.

## Changelog

- 2026-08-05 — Initial version.
