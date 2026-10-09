# Caudal Diagram Visual Language

Use this only after the Mermaid flow is approved.

## Character

Editorial, calm, and precise. Diagrams should look drawn for the article around them, not exported from enterprise architecture software. Avoid gradients, shadows, glass effects, 3D objects, stock illustrations, and decorative arrows.

## Canvas

| Profile | Desktop | Mobile |
|---|---:|---:|
| Editorial | `1200 × 675` preferred; up to `1200 × 900` for sequences | `720 × 960` or `720 × 1080`; recompose vertically |
| Technical | Natural Mermaid dimensions; maximum content width `1200` | Allow horizontal pan only when exact topology cannot be recomposed |

- Editorial outer padding: `64 px` desktop, `40 px` mobile.
- Minimum gap between primary nodes: `40 px`.
- Internal card padding: `28–36 px`.
- Corners: `16 px` for cards, `24 px` for groups.
- Strokes: `2 px`; emphasized path: `3 px`.
- Do not shrink a desktop SVG to mobile. Produce a second composition from the same Mermaid.

## Palette

Use the site tokens; do not sample arbitrary colors.

| Role | Value | Use |
|---|---|---|
| Paper | `#faf8f4` | canvas and neutral cards |
| Raised paper | `#f3efe8` | institutional groups and secondary regions |
| Ink | `#1c1917` | primary text and structural outlines |
| Soft ink | `#44403c` | explanatory text |
| Muted | `#78716c` | metadata and secondary connectors |
| Hairline | `#e5dfd6` | separators and quiet borders |
| River teal | `#14666c` | active route, selected state, primary emphasis |
| Dark teal | `#0e4d52` | hover/focus and high-contrast teal |
| Obligation ochre | `#9a5b20` | interinstitutional obligation only |
| Error red | `#a63d40` | rejection or failure only |

Never distinguish flows by color alone. Pair color with labels, line patterns, or shapes.

## Semantic connectors

| Meaning | Treatment |
|---|---|
| Request or payment message | solid river-teal line, arrowhead, explicit label |
| Response or confirmation | dashed river-teal line, arrowhead, explicit label |
| Customer ledger movement | solid ink line labeled `reserva`, `débito`, or `crédito` |
| Interinstitutional obligation | solid ochre line labeled `obligación` |
| BCE settlement | double ink line labeled `liquidación BCE` |
| Ownership or containment | thin muted line, no arrowhead |
| Unknown or reconciliation path | dotted error-red line with state label |

Do not use “money” as an arrow label. Name the exact event or asset.

## Typography

- Editorial heading: Fraunces, `500`.
- Labels and body: Inter.
- Technical Mermaid: Inter or its default sans-serif fallback.
- Desktop diagram title: `28–34 px`.
- Primary node label: `22–26 px`, `600`.
- Supporting text: `18–20 px`.
- Eyebrow/state label: `14–16 px`, `650`, uppercase, tracking `0.08em`.
- Minimum final rendered text: `16 px` desktop and `18 px` mobile.
- Maximum text per node: one title plus two short lines.
- Never reduce text size to solve a layout problem; recompose or split the diagram.

Use sentence case in Spanish. Reserve uppercase for short categories such as `SISTEMA CENTRAL`, never full explanations.

## Components

### Person

Use a simple outline figure or a text actor. Name the action: `Quien paga`, `Quien cobra`. Do not use demographic illustrations unless the article depends on that story.

### Institution

Neutral paper card with an ink outline. Show role first (`Integrante ordenante`) and example second only if it helps.

### Red de pagos

Raised-paper group or card. Label it as an institution, never as a VPN, cloud, or telecom link.

### Central service

River-teal card only when it is the focal component. If several central services appear, group them under `SIP` and keep individual cards neutral.

### Ledger or store

Use a ledger/card-stack symbol, not a generic database cylinder, unless the storage technology is the subject. Always name whose ledger it is.

### State

Rounded pill for a state code (`ACSP`, `ACSC`, `PDNG`) followed by a plain-Spanish explanation outside the pill.

## Composition

1. Put the reader's entry point at the upper left or top.
2. Keep the primary reading path unidirectional.
3. Place responses on a separate return lane; do not draw opposing arrows on top of each other.
4. Use institutional grouping before color to communicate ownership.
5. Put the conclusion below the diagram, not inside a large decorative banner.
6. Captions explain why the relationship matters; they do not repeat the title.

## Editorial versus technical

Editorial:
- begin and end with people or familiar actions;
- translate protocol events into plain Spanish;
- reveal at most one protocol code if the article explains it;
- use 4–6 primary nodes.

Technical:
- retain official actors and state codes;
- show trust and storage boundaries;
- label interfaces and message names where useful;
- may use 7–10 nodes if grouping keeps the structure legible.

## Accessibility and behavior

- Include SVG `<title>` and `<desc>`, plus page-level `alt`.
- Contrast must meet WCAG AA.
- Interactive diagrams use buttons, keyboard focus, and `aria-live` for changing explanations.
- Respect `prefers-reduced-motion`.
- The static first frame must communicate the complete relationship.
- Do not make mobile users horizontally scroll an editorial diagram.
