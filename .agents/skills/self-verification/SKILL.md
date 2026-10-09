---
name: self-verification
description: The verification ladder for Caudal. Use after every change and before claiming any task is done, to pick the cheapest check that can catch the mistake and to know which extra checks a domain needs (ledger invariants, catálogo citations, Spanish, mirrors).
---

# Self-verification

Verify at the **cheapest level that can catch your mistake**, and never claim done on an unverified change.

## The ladder

| Level | When | Command | Cost |
|---|---|---|---|
| 0. Check one crate | After each edit batch | `just check <crate>` | seconds |
| 1. Test one crate | When a task in a crate is complete | `just test <crate>` | under a minute |
| 2. Whole workspace | When a change crosses crates (kernel types, traits) | `just test` | minutes |
| 3. Done-gate | Once, before opening the PR | `just ci` | minutes |

Level 3 is a gate, not a loop step.

## Domain lanes

| Change touches | Also verify |
|---|---|
| `ledger` or anything that posts | The property-test suite passes and every invariant in `docs/architecture/ledger.md` has an assertion; no `UPDATE`/`DELETE` on posting tables |
| Postgres code | Tests run against this worktree's `DATABASE_URL` (`just db` first) |
| Messages, reason codes, states, llaves | Every value traces to `docs/research/catalogo-operacional.md` or carries **[assumption]**; golden files updated deliberately, never regenerated blindly |
| Crate dependencies | The dependency rules in the crate-map ADR still hold; the crate's `AGENTS.md` still describes it |
| Regulatory text | Searched in `research/bce/*.md`, then checked in the PDF |
| Spanish text | Reads native per `docs/spanish-style.md` |
| Manifesto | `MANIFIESTO.md` and `site/src/pages/manifiesto.md` identical |
| Site or `docs/` | `just site-build` |

## Anti-patterns

- "It compiles" is not verification.
- Weakening a property test or an invariant to make it pass.
- Asking the maintainer to check something a ladder level can check.

## Done checklist

- [ ] Levels 0 and 1 on every touched crate
- [ ] The domain lanes that apply
- [ ] `just ci` once, green
- [ ] New docs linked from their index
