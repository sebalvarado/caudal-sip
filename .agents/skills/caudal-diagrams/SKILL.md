---
name: caudal-diagrams
description: Designs and implements Proyecto Caudal diagrams for the public site and technical documentation. Use whenever creating, changing, reviewing, or embedding a diagram, flowchart, sequence, infographic, payment journey, llave flow, participant map, ledger flow, or Mermaid source.
---

# Caudal Diagrams

Create diagrams in two stages: **agree on meaning in Mermaid, then design the final artifact**. Never style an unapproved flow.

## Required workflow

1. Write the one question the diagram answers.
2. List actors, systems, stores, events, and boundaries.
3. Separate these channels before drawing:
   - customer funds in an integrante's core ledger;
   - messages between participantes;
   - interinstitutional obligations in the SIP ledger;
   - settlement in BCE money.
4. Create or update a Markdown source with one Mermaid block in `docs/diagrams/`. Keep the question, channel, status, and review notes beside the diagram.
5. Review the Mermaid with the user. Check every node and arrow against project documentation.
6. Only after approval, choose an output profile:
   - **Technical:** render or embed Mermaid.
   - **Editorial:** create a bespoke SVG plus a separate mobile composition.
7. Embed the artifact next to the paragraph it explains, with useful alt text and a one-sentence caption.
8. Inspect desktop and mobile renders in a browser. `cd site && npm run build` must pass.

Do not delete the approved Mermaid document after producing the final artifact. It remains the conceptual reference; SVG is the presentation layer.

## Meaning before appearance

- One diagram answers one question. Split a diagram that needs “and also”.
- Label every arrow with what crosses the boundary.
- Never use an unlabeled arrow that could mean either a message or money.
- A red de pagos routes messages and owns a distributed directory; do not imply that it holds customer money.
- The SIP obligation ledger is not an integrante's customer ledger and is not the BCE settlement account.
- `ACSP` and `ACSC` are different moments. Show both only when the distinction is the subject.
- A timeout is not automatically a rejection. Show unknown outcomes and reconciliation explicitly where relevant.
- Use domain vocabulary from `docs/glossary.md`.

## Mermaid draft rules

- Prefer `sequenceDiagram` for time and request/response flows.
- Prefer `flowchart` for topology, ownership, and static relationships.
- Prefer `stateDiagram-v2` for payment and llave lifecycles.
- Keep actor labels short; put explanation in arrow labels or adjacent prose.
- Use `rect` or `box` blocks to expose institutional boundaries.
- Do not add colors, icons, custom themes, or layout hacks during conceptual review.
- If a diagram exceeds seven primary participants, split it.

See [templates.md](templates.md) before drafting Mermaid.

## Final visual profiles

Read [visual-language.md](visual-language.md) before producing or reviewing SVG, HTML, CSS, or interactive diagrams.

### Editorial

For the manifesto, blog, and public pages. Optimize for a reader who knows nothing about payment infrastructure. Use plain Spanish, human entry and exit points, generous space, and one emphasized idea.

### Technical

For architecture and implementation documentation. Preserve exact component names, states, boundaries, and protocol terms. Clarity and diffability matter more than brand expression.

## Interaction threshold

Add interaction only when the reader makes a meaningful choice or advances through a sequence. Good candidates: resolving a llave, replaying a lost response, or advancing a payment through states.

Do not add interaction for a static topology. Never require JavaScript to understand the core point. The initial state and a static fallback must be complete.

## Review checklist

- [ ] The question answered is stated.
- [ ] Actors and system boundaries are correct.
- [ ] Messages, obligations, customer funds, and BCE settlement are not conflated.
- [ ] Every ambiguous arrow is labeled.
- [ ] No node contains a paragraph.
- [ ] Mermaid was approved before final styling.
- [ ] Editorial and technical outputs derive from the same Mermaid document.
- [ ] Desktop and mobile use deliberate compositions, not a shrunk desktop image.
- [ ] Color is never the only carrier of meaning.
- [ ] Alt text explains the relationship, not the decoration.
- [ ] The site builds and the rendered artifact was inspected.
