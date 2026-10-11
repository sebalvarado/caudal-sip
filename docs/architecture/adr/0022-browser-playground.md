# 0022. The public demo is a playground that runs the network in the visitor's browser

- **Status**: Accepted
- **Date**: 2026-10-11
- **Deciders**: Founding maintainer
- **Issue**: [#2](https://github.com/sebalvarado/caudal-sip/issues/2)
- **Supersedes**: [0020](0020-azure-container-apps-demo-host.md), [0021](0021-public-demo-api.md)

## Context

Epic 1 ends with a public demo of the network ([`epics.md`](../../plan/epics.md)). ADR-0020 put it on Azure Container Apps and ADR-0021 fenced it behind a closed HTTP API. Both work. But a server only lets a visitor do one thing at a time: send a payment, then settle. The questions worth asking take a whole day of traffic: what happens to a segment-3 cooperative on payroll day, or with one more settlement window.

The network already runs as a deterministic simulation in one process ([ADR-0019](0019-caudal-terminal.md)), and the core crates compile to WebAssembly unchanged. [Numscript's playground](https://playground.numscript.org/) shows the shape: an editor, a script, a run button, results beside it.

We do not need a Numscript-style language for postings. The catálogo fixes the posting patterns ([`posting-patterns.md`](../posting-patterns.md)), so letting visitors write their own would demonstrate something the scheme forbids. What visitors need to vary is the day itself: when payments happen, how many, how big, when the malla settles, and what each institution's ceiling is.

## Decision

**A scenario language drives the simulation.** A script is the terminal's commands placed on the simulated clock (`at 09:00`, `wait 1m`), plus a few statements that generate load: `repeat 300 every 12s: pay …`, `traffic 20000 until 17:30`, `retry`, and `ceiling` to override an institution's ceiling before the day starts. The built-in scenarios (`caudal scenario …`) are scripts in this language, `caudal run <file>` runs any script, and a parse error names its line.

**The playground runs that engine in the browser.** `apps/caudal` builds as a library for `wasm32-unknown-unknown`. A Web Worker on the site's `/simulador/` page loads it and runs the script; the page shows the summary, the settlements, each institution's headroom over the day, and the payments the script named. Scripts are shared through the URL. Every message is still built, signed, and verified, as it is in the terminal.

**No server, so no attack surface.** Nothing the visitor types leaves their machine. The script can only name the seeded institutions and llaves, so it cannot hold real data. A script sends at most 50,000 payments.

**The WebAssembly build is committed** under `site/public/simulador/pkg/`, with a hash of the Rust sources it was built from. CI rebuilds it, fails if the hash is stale, and runs every template through it in Node. The site build stays a plain Astro build.

The container image, the Azure deploy workflow, the Bicep template, and `caudal serve` are removed.

## Alternatives considered

- **Keep the hosted API (ADR-0020, ADR-0021) and add the playground.** Two public surfaces to maintain, and the server is the weaker of the two. Rejected.
- **A Numscript-style posting language.** It would let visitors invent posting patterns the catálogo does not allow. Rejected; the posting patterns stay code.
- **Build the WebAssembly in the site's CI instead of committing it.** The site build would need the Rust toolchain. Rejected while the artifact is under a megabyte; revisit if it grows.
- **Run scripts on a server.** It brings back hosting, cost, and abuse limits for no gain. Rejected.

## Consequences

Easier: the demo costs nothing to run, cannot be abused, and stays up as long as the static site does. Visitors can ask the payroll question themselves and share the answer as a link. The same scripts run in the terminal, in CI, and on the page.

Harder: the committed WebAssembly has to be rebuilt (`just playground`) whenever the Rust sources change, and CI enforces it. A full national day signs and verifies some 160,000 messages, which takes tens of seconds in a browser, so the page shows progress. The terminal's live view of one payment's hops stays in the terminal.
