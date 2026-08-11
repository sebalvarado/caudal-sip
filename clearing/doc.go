// Package clearing accumulates irrevocable inter-red obligations into
// bilateral and multilateral mallas, nets them, and produces the settlement
// instructions executed against the integrantes' accounts at the BCE.
//
// Settlement runs on the multilateral malla; the bilateral one is computed for
// reporting only. Cycles run several times a day on a parameterized schedule,
// and the administrator can trigger one manually under liquidity stress.
// Settlement instructions are idempotent: a replayed instruction must not
// double-post.
//
// Closing a malla is also what releases headroom, which makes this package the
// pacing mechanism of the whole scheme — an institution's capacity between
// cycles is bounded by its ceiling. RFC 0001 argues that this is a problem
// outside operating hours.
//
// Postings: docs/architecture/posting-patterns.md
package clearing
