# 0020. The public demo runs on Azure Container Apps, deployed from GitHub Actions without stored secrets

- **Status**: Superseded by [0022](0022-browser-playground.md)
- **Date**: 2026-10-11
- **Deciders**: Founding maintainer
- **Issue**: [#7](https://github.com/sebalvarado/caudal-sip/issues/7)

## Context

Epic 1 ends with the network live on Azure, driven from a page on the site ([`epics.md`](../../plan/epics.md)). This is *our* demo, not what an institution runs ([`deliverables.md`](../../product/deliverables.md)). It carries only synthetic data ([ADR-0021](0021-public-demo-api.md)), it is reset regularly, and nobody depends on it. Hosting has to be cheap, deploy on every merge to `main`, and keep cloud credentials out of the repository.

## Decision

**One container on Azure Container Apps.** The image runs `caudal serve`: the whole in-process network of [ADR-0019](0019-caudal-terminal.md) behind the demo API. It is built from the repository's `Dockerfile` and pushed to GitHub Container Registry, and its definition lives in `infra/azure/` as Bicep.

**No database for the demo.** The demo uses the in-memory ledger. Its state is synthetic and is reset anyway, so a managed Postgres would add cost and an operational surface without carrying anything worth keeping. CI exercises the Postgres backend on every pull request. A deployment that needs persistence adds Azure Database for PostgreSQL through the same Bicep, without a new decision.

**Scale to zero.** The app runs with a minimum of zero replicas and a maximum of one. A cold start costs a few seconds and starts from the seed, which is also the reset. The expected monthly cost is under USD 10 at demo traffic **[assumption]**.

**Reset.** The process reseeds the network on a fixed interval (fifteen minutes by default), on every deploy, and on every cold start.

**OIDC federation, no long-lived secrets.** The deploy workflow signs in with `azure/login` using a federated credential bound to this repository's `main` branch. The repository stores only non-secret identifiers as variables: client, tenant and subscription ids, resource group, app name. When those variables are absent, as on forks, the deploy job is skipped and does not fail.

**Nothing in the OSS core depends on Azure.** Azure appears only in `infra/azure/` and in the deploy workflow. The image runs on any container host.

## Alternatives considered

- **Fly.io.** Comparable cost and simpler. Rejected because the authorized-red workstream already plans on Azure, and one cloud keeps operations to one set of skills.
- **Cloudflare Workers or Containers.** The site already deploys to Cloudflare Pages. Rejected for the backend: Workers cannot run the native binary, and their container offering was less mature for long-running processes when this was decided.
- **A self-hosted VM.** The cheapest at steady state, but it needs patching, a TLS setup, and a person. It costs more in attention than it saves in money.

## Consequences

Easier: one image, one command to deploy, no secrets to rotate, and a demo that costs close to nothing while idle.

Harder: the first request after an idle period waits for a cold start. Demo state never survives a restart, by design.

Committed: OIDC-only deployment, scale to zero, reset by reseeding, and no Azure dependency outside `infra/` and the workflow.
