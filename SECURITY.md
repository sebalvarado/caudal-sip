# Security Policy

This project builds payment infrastructure. We treat security reports as the highest-priority work in the repository.

## Reporting a vulnerability

**Do not open a public issue for security vulnerabilities.**

- Preferred: [report a vulnerability privately on GitHub](https://github.com/sebalvarado/caudal-sip/security/advisories/new) (*Security* → *Report a vulnerability*).
- If you cannot use GitHub, email the maintainer listed in [CODEOWNERS](.github/CODEOWNERS) — an address at the project domain will replace this once DNS is live, and this file on `main` always has the current one.

Include: affected component and version/commit, reproduction steps or proof of concept, and impact assessment if you have one. Reports in Spanish or English are equally welcome.

## What to expect

- **Acknowledgment within 72 hours**, an assessment within 7 days.
- We coordinate disclosure with you. Default embargo: 90 days or until a fix ships, whichever is first.
- Credit in the advisory and changelog if you want it (or anonymity if you prefer).

## Safe harbor

We will not pursue legal action for good-faith security research that respects these rules: no access to data that isn't yours, no service disruption of third parties, no social engineering of contributors or institutions, and private reporting per the above.

Note that this project ships **software**; we do not operate production payment systems. If you find a vulnerability in a deployed instance operated by an institution, report it to that institution — and to us if the root cause is in this codebase.

## Scope notes for researchers

Especially valuable areas: message parsing/validation (`iso20022/`), signature verification and mTLS handling, llave directory logic (enumeration, uniqueness races, claim flows), settlement/ledger invariants (double-spend, negative balance, idempotency bypass), and the multi-tenant isolation of the gateway.

## Supported versions

Pre-1.0: only the latest release and `main` receive security fixes.
