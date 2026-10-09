---
name: authorized-red
description: Work on the authorized-red workstream — becoming a BCE-authorized red de pagos / partícipe del Sistema Auxiliar de Pagos in Ecuador. Use when the task touches BCE authorization, the licensing dossier (BCE-GG-024-2024), adhesion contracts, the visita técnica, SEDPE/fintech onboarding as members, or business planning for the hosted-connectivity company.
---

# Authorized Red Workstream

The business/licensing track lives in `workstream/authorized-red/`. Read its `README.md` first — it is the index and holds the working rules.

## Where things are

| Need | File |
|---|---|
| The execution plan, tracks, calendar | `workstream/authorized-red/plan.md` |
| Investor/partner business plan, comparables, financial model | `workstream/authorized-red/business-plan.md` |
| Dossier requirements, item by item (BCE-GG-024-2024 art. 3 + service add-ons) | `workstream/authorized-red/regulatory/dossier-psap.md` |
| Unresolved legal/regulatory questions with routes to answers | `workstream/authorized-red/regulatory/open-questions.md` |
| Member pipeline and adhesion-contract tracking | `workstream/authorized-red/commercial/pipeline.md` |
| What must be demoable for the BCE's visita técnica | `workstream/authorized-red/technical/readiness.md` |
| What to build, test, and unblock for production | `workstream/authorized-red/technical/que-construir.md` |
| What a red technically is and must do | `docs/architecture/red.md` (architecture — do not duplicate it in the workstream) |
| The normative scheme spec | `docs/research/catalogo-operacional.md` |
| Primary BCE text for search and verification | `research/bce/README.md` (read the matching `.md`, then verify its PDF) |

## Rules specific to this stream

1. **Cite and verify the norm**: search the matching Markdown in `research/bce/` first, then check its vendored PDF before relying on a table or citation. Cite BCE-GG-024-2024 (authorization procedure), JPRM-2024-018-M arts. 22–23 (who/which services), BCE-GG-008-2025 (interoperability + Art. 10 security), BCE-GG-007-2026 (messaging), or a Catálogo Operacional section. The operational catalogue is not the unpublished technical catalogue; do not invent field-level rules. Unverified readings are marked **[supuesto]** — P1 in `open-questions.md` (which authorization makes a red) is still one.
2. **Never commit** signed contracts, notarized documents, financial statements, or personal data. Reference them by name; artifacts stay in the company's private store.
3. **Language**: all documents in `workstream/authorized-red/` are **Spanish** (they are worked through with Ecuadorian partners, lawyers, and counterparties) and must read native per `docs/spanish-style.md` — use the standard's vocabulary (llave, malla, empadronamiento, calificación) and keep protocol identifiers untranslated. Unverified regulatory readings are marked **[supuesto]**.
4. Keep statuses current: dossier checklist boxes, pipeline stages, and readiness table are trackers, not prose — update them as part of any task that changes their facts.
5. The clock is **2026-11-30** (institutions' deadline); the controllable target is **dossier filed by mid-October 2026**. When prioritizing, anything on the filing critical path (dossier items, adhesion contracts, visita readiness) beats everything else.
