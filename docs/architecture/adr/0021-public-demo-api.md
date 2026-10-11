# 0021. The public demo API: a closed set of operations over a seeded catalogue

- **Status**: Accepted
- **Date**: 2026-10-11
- **Deciders**: Founding maintainer
- **Issue**: [#8](https://github.com/sebalvarado/caudal-sip/issues/8)

## Context

The site drives a live network ([ADR-0020](0020-azure-container-apps-demo-host.md)). Anything reachable from the public internet will be probed, and anything that accepts free text will eventually receive a real phone number or account number. The repository's rule is absolute: no real account data, ever (`AGENTS.md`, non-negotiable 5).

## Decision

**The API exposes exactly these operations; every other path returns 404.**

| Method and path | Does |
|---|---|
| `GET /api/network` | Lists the seeded redes and institutions, with each one's headroom, traffic light, and net position |
| `GET /api/llaves` | Lists the seeded llaves by catalogue id, with their type |
| `POST /api/resolve` | Resolves a seeded llave by catalogue id and returns the masked name and the beneficiary institution |
| `POST /api/payments` | Sends a payment from a seeded institution to a seeded llave, and returns every hop with its ISO 20022 message and the resulting states |
| `GET /api/payments/{id}` | Reads one payment the demo made |
| `POST /api/settle` | Closes and settles the current malla |

**No identifier is free-form.** Institutions and llaves are referenced by catalogue id (`banco-alfa`, `llave-03`), never by a BIC, phone number, ID number, email, or account number. Names and document numbers in the seed are fictional. An amount is a decimal string of at most two decimals, between 0.01 and 1,000.00. A request with any other field, or with an unknown id, is rejected. Real data therefore cannot enter, because no field can hold it.

**Abuse limits.** A fixed-window rate limit allows 30 requests per minute per client address, held in memory and never logged. CORS allows only the site's origin. No cookies, no authentication, no stored request bodies. The network resets to the seed every fifteen minutes (ADR-0020), so nothing persists.

**Every response carries the disclaimer**, in the `X-Caudal-Disclaimer` header and in a `disclaimer` field of every JSON body: *Demostración con datos ficticios. Proyecto Caudal es independiente y no está afiliado al Banco Central del Ecuador.*

## Alternatives considered

- **Exposing the North API itself, sandboxed.** It is the real integration surface, but it accepts free-form llaves and accounts by design. Rejected for the public demo. A sandbox with issued credentials can expose it later.
- **Accepting any llave value but validating its format.** A valid format is exactly what real data has. Rejected.
- **Authentication with demo accounts.** It protects nothing that rate limiting and the closed catalogue do not already protect, and it adds friction.

## Consequences

Easier: the demo is safe to leave online, and its whole attack surface fits in one table.

Harder: visitors cannot try their own llave. The page says so, and points to the terminal for anyone who wants to.

Committed: the closed operation set, catalogue ids only, the disclaimer on every response, and periodic reset.
