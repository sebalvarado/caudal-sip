# Language Policy

Language follows **audience**, not document type. The question is never "is this technical?" — it is "who has to read this to do their job?"

This replaces the earlier "English for docs, Spanish for public copy" rule, which cut along the wrong axis and contradicted [`product/north-star.md`](product/north-star.md), where Spanish documentation written for engineers is named as one of the three things that make this project durable.

## The split

**Spanish** — anything an Ecuadorian institution reads to do its job:

- The Catálogo Operacional reference and any regulatory analysis
- Conformance material and certification reports
- Technical proposals aimed at the BCE, the redes, or institutions — **including scheme-change RFCs**, whose readers are the people who would have to adopt the change
- Operator runbooks and incident procedures
- The participant and network consoles, including error messages
- Tutorials, integration guides, and onboarding material
- The manifesto, the website, and all outreach

**English** — anything whose audience is the toolchain or the global contributor:

- Code, identifiers, API field names, code comments
- Commit messages and pull request descriptions
- ADRs (notes to future maintainers about decisions of record)
- RFCs about this project's own governance and internal mechanics
- Contributor mechanics: `CONTRIBUTING.md`, `AGENTS.md`, issue templates
- Internal strategy documents (`product/`, unless institution-facing)

Domain terms stay Spanish in both languages, always: *llave*, *integrante*, *red de pagos*, *credencial de pago*, *por conciliar*, *malla*. See [`glossary.md`](glossary.md), which is bilingual and is the vocabulary of record.

## The rules

1. **One canonical language per document.** Never maintain a translated mirror. Mirrors drift, and drift in a specification is not an inconvenience — if the Spanish and the English disagree about when irrevocability attaches, someone implements the wrong one. The `MANIFIESTO.md` / website pair is the one grandfathered exception, and it needs a checklist item in the PR template to survive.
2. **Flagship Spanish documents carry a short English abstract**, not a translation: three or four sentences at the top, enough for a multilateral or a foreign contributor to know what the document contains and whether they need it translated. Applies to the catálogo reference, the conformance matrix, and the technical proposals.
3. **Spanish must read native**, per [`spanish-style.md`](spanish-style.md). A translated-sounding Spanish technical document is worse than an English one, because it signals the author was not writing for the reader.
4. **When in doubt, ask who is blocked without it.** If the answer is a Jefe de Sistemas in Loja, write Spanish. If it is a contributor in Berlin reading the code, write English.

## Precedent

Banco Central do Brasil publishes the Pix specifications in Portuguese on GitHub; Bre-B's materials are Spanish. No national payment scheme documents itself in English, and the audiences who would prefer it — the IDB and World Bank payments teams in [`product/adoption.md`](product/adoption.md) — are well served by an abstract.

## Migration

Existing English documents convert as they are touched rather than in one pass, with one exception: the catálogo reference was flipped immediately, because it is the highest-value artifact we have and its reader is unambiguously Spanish-speaking.
