# Research sources

Primary documents live here **locally only** — third-party papers are copyrighted by their authors, so we link them instead of redistributing them (`research/*.pdf` is gitignored). Our synthesis of all of this lives in [`docs/research/landscape.md`](../docs/research/landscape.md), which is the document to read.

## Reference systems

| Document | Source |
|---|---|
| Bre-B: Colombia's Interoperable Instant Payments System — Design, Implementation, and Outlook (March 2026) | [Banco de la República](https://d1b4gd4m8561gs.cloudfront.net/sites/default/files/publicaciones/archivos/bre-b-working-paper-march-2026.pdf) |
| Pix — API specifications, open on GitHub | [github.com/bacen/pix-api](https://github.com/bacen/pix-api) |
| Pix — regulation and statistics | [Banco Central do Brasil](https://www.bcb.gov.br/en/financialstability/pix_en) |
| UPI — product overview and circulars | [NPCI](https://www.npci.org.in/what-we-do/upi/product-overview) |

## Ecuadorian regulation

| Document | Source |
|---|---|
| **Catálogo Operacional para transferencias electrónicas de pago en tiempo real — SIP**, v1.0, March 2026 | [bce.fin.ec](https://www.bce.fin.ec/storage/BCE/resoluciones/Catalogo-Operacional.pdf) |
| BCE-GG-007-2026 — messaging standard for real-time payments | [bce.fin.ec](https://www.bce.fin.ec/storage/BCE/resoluciones/BCE-GG-007-2026.pdf) · [Registro Oficial 2º Supl. 245](https://asobanca.org.ec/wp-content/uploads/2026/03/RO-Res.-BCE-GG-007-2026-Estandar-mensajeria-transferencias-electronicas-pago-tiempo-real.pdf) |
| BCE-GG-008-2025 — SIP/RPI operating rules, qualification, clearing | [bce.fin.ec](https://www.bce.fin.ec/storage/BCE/resoluciones/BCE-GG-008-2025.pdf) |
| JPRM-2024-029-M — reform defining SIP, RPI, llaves, deadlines | [bce.fin.ec](https://www.bce.fin.ec/storage/JPRM/resoluciones/JPRM-2024-029-M.pdf) |
| JPRM-2024-018-M — original interoperability resolution | Registro Oficial |

Citations with article numbers are in [`docs/research/landscape.md`](../docs/research/landscape.md).

**The Catálogo Operacional is public** (found 2026-08-05) and supersedes most of what we previously marked **[assumption]**: payment states, rejection codes, llave types and states, timing windows, the payment identifier format, mandatory ISO 20022 fields, netting and liquidity-risk rules.

**Still missing**: the **catálogo técnico** — the field-by-field message profile and XSDs that accompany the operational catalog. Anything depending on it stays marked **[assumption]**.

## Adding a source

Drop the PDF here for local reading, add a row above with a stable public URL, and fold the findings into `docs/research/landscape.md` with a citation. Never paste long verbatim excerpts of third-party material into our docs.
