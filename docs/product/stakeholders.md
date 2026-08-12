# Stakeholders: who this is for

> **Read before scoping a feature, writing copy, or deciding whether a request deserves work.** The short answer to "is this just a developer tool?" is no — it is three products sharing one codebase, aimed at audiences that measure success differently.
>
> **Owner:** Sebastián · **Last reviewed:** 2026-08-05 · **Update trigger:** when a new operator profile appears in the regulation, or when a design partner changes the priority order.

## Three products, one codebase

| Product | Who it serves | What "good" looks like | Where it lives |
|---|---|---|---|
| **A developer tool** | Engineers integrating with the standard | A payment clears within 15 minutes of `git clone`; a core-banking developer never reads an XSD | `iso20022/`, `spec/`, `qr/`, SDKs, `sandbox/` |
| **An operable product** | Institutions that must comply, and the people who run them daily | A cooperativa passes BCE qualification and operates the thing with existing staff, not new hires | `gateway/`, participant console, `tck/`, runbooks |
| **A public argument** | Regulators, networks, multilaterals, press | "Building this is hard and slow" stops being credible, because it is running and measurable | `sandbox/`, `docs/research/`, load-test numbers, the manifesto |

> The directory names above predate the decision to build the central system, and a fourth product — the scheme core the BCE would run — is missing from the table. [`deliverables.md`](deliverables.md) is the current list of what ships and who touches it; this section is due a rewrite against it.

Confusing these is the main failure mode. A beautiful Go library that no institution can operate fails product 2. A working gateway nobody outside one cooperativa has heard of fails product 3.

## Primary: institutions that must comply

The wedge. Roughly 400 entities — 24 banks, 365+ cooperativas across segments, mutualistas, plus SEDPEs and fintechs — face BCE deadlines starting **2026-11-30** for intra-network real-time payments.

| Segment | Situation | What they need from us |
|---|---|---|
| **Large banks** | Have engineers, budget, and probably a vendor quote already | Mostly the spec, the conformance kit, and a credible second option that resets vendor pricing |
| **Mid-size banks and segment-1 cooperativas** | Small IT team, no ISO 20022 experience, real deadline pressure | Self-hosted gateway with a console their operations staff can use |
| **Small cooperativas and mutualistas** | No infrastructure team; compliance is an existential cost | The hosted option, or their federation/vendor running it for them. They will never deploy anything themselves |
| **Fintechs and SEDPEs** | Fast-moving, API-native, want in as licensed integrantes | The North API and sandbox; they are also our earliest and loudest contributors |

### The people inside the institution

Building for "a cooperativa" is meaningless. These four decide whether we are adopted:

1. **Gerente de Tecnología / Jefe de Sistemas** — the technical buyer. Pain: a deadline they cannot meet with current staff, and a vendor quote they cannot afford. Question they ask: *can my team actually run this, and who do I call at 2am?*
2. **Jefe de Operaciones / Medios de Pago** — the daily user of the participant console, and the person nobody builds for. Pain: reconciling failed payments, answering "where is my money" from a customer, certificate expiry nobody warned them about. **If this person cannot do their job in a UI, the institution cannot adopt us regardless of how good the API is.**
3. **Oficial de Riesgos / Cumplimiento** — the veto. Pain: has to defend the choice to the SEPS or the board. Needs the security model, the audit trail, and the non-affiliation disclaimer in writing.
4. **Gerente General** — signs. Cares about the deadline, the cost, and not being the institution whose customers cannot receive payments.

### The buying trigger

A concrete, dated obligation: qualification with the BCE, a red de pagos setting an integration date, a vendor quote arriving, or a competitor announcing instant payments first. We sell "you will make the date", not "open-source payments infrastructure".

## Multipliers: the people who bring many institutions at once

Ranked by institutions reached per unit of effort. These are the highest-leverage relationships in the project.

| Multiplier | Reach | What they want |
|---|---|---|
| **Core-banking vendors** serving cooperativas | Dozens of institutions per vendor | To ship compliance to their clients without building ISO 20022 themselves. Apache-2.0 makes us a component they resell, not a competitor |
| **Cajas centrales and federations** (RFEC, UCACSUR and peers) | Their whole membership | To protect members from a cost shock, and to keep the relationship. Ideal operators of a multi-tenant gateway |
| **Redes de pagos** | Every participant connected to them | Intra-network compliance on schedule, and fewer support calls from unprepared members |
| **Multilaterals** (IDB, World Bank payments teams) | Institutional doors, including the BCE's | Reusable digital public infrastructure — exactly what their mandates reward |

## The regulator and network operators

**The BCE** is simultaneously the rule-maker, the operator of the SIP and the settlement rail, and our most consequential audience. It is not a customer and must never be addressed as one. What it can plausibly want: a public test environment it did not have to build, a pre-certification tool that reduces the qualification burden on its own staff, and technical proposals for what the catálogo does not yet cover. The precedent to invoke is always Banco Central do Brasil publishing Pix specs on GitHub.

Failure mode to avoid: anything that reads as a parallel or competing rail. Every artifact carries the non-affiliation disclaimer for this reason.

**Redes de pagos** (BANRED today, plausibly others) run the intra-network switching and clearing. They are both potential adopters of the network profile and incumbent sellers of integration services. Treat them as adopters; expect some to treat us as competition. That tension is manageable because the long tail they underserve is precisely who we serve first.

## Contributors

The project has no future without engineers who did not start it. Three pools: Ecuadorian engineers at fintechs abroad (skills plus motivation), local developers and students at ESPOL, USFQ and EPN (need a gentle first issue and Spanish docs), and payments specialists worldwide who find the problem interesting. What they need is the same thing institutions need — the stack running locally in one command — which is why [`../development/local-stack.md`](../development/local-stack.md) is a strategic document, not a convenience.

Non-engineer contributors matter too, and we under-recruit them: lawyers reading the resolutions, economists modeling settlement risk and fee policy, technical writers, and designers. The [RFC process](../rfcs/README.md) exists partly to give them a way in.

## Beneficiaries, not users

Citizens, merchants, and small businesses never touch this software — they use their institution's app. They are why the project exists and the audience for the [manifesto](../../MANIFIESTO.md), but "would a user want this?" is the wrong question for a component. The right one is: *does this make the institution's app faster, cheaper, or safer for them?*

## Anti-stakeholders

Saying no to these is what keeps the project coherent:

- **Anyone who wants a wallet, custody, or a consumer app built.** That is our participants' business and their liability. We would be competing with the people we need to adopt us.
- **Blockchain and stablecoin projects** looking for a national rail to attach to. The settlement asset is central bank money. This is not a philosophical position, it is what keeps regulators in the room.
- **Institutions wanting bespoke integration consulting** without any intention of using or contributing to the shared implementation. We are a project, not a services firm; that path consumes all available time and produces nothing reusable.
- **Enterprises that want a private fork with a proprietary feature.** The license permits it and we will not build it.
- **Anyone asking us to reverse-engineer or leak an unpublished BCE catálogo.** We build profile-driven precisely so we do not need to, and the paper trail of asking properly is itself an asset.

When a request arrives, ask: does it help a named institution meet the mandate, a multiplier reach many institutions, or a contributor get productive? If not, it is a distraction by default.

## Changelog

- 2026-08-05 — Initial version.
