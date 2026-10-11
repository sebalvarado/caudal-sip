# 0019. Rust CI runs on Azure Pipelines; the code stays on GitHub

- **Status**: Accepted
- **Date**: 2026-10-10
- **Deciders**: Founding maintainer
- **Issue**: [#6](https://github.com/sebalvarado/caudal-sip/issues/6), part of [#1](https://github.com/sebalvarado/caudal-sip/issues/1)

## Context

[`epics.md`](../../plan/epics.md) planned CI on GitHub Actions. The demo network deploys to Azure, and the maintainer already runs a working Azure DevOps setup for another GitHub-hosted project. That setup has Terraform for the project, a GitHub App connection that reports checks on pull requests, and YAML pipelines. Reusing it means one CI and CD platform to operate instead of two.

## Decision

The Rust checks run on Azure Pipelines, reading this GitHub repository through the Azure Pipelines GitHub App. [`azure-pipelines-ci.yml`](../../../azure-pipelines-ci.yml) runs the same checks as `just ci`: `cargo fmt`, `clippy` with warnings denied, tests against a `postgres:16` service, and `cargo deny`. It also runs a DCO job that fails a pull request with an unsigned commit. The Rust job skips until the Cargo workspace exists ([#5](https://github.com/sebalvarado/caudal-sip/issues/5)), so the pipeline is green from day one.

The project and the pipeline definition are Terraform in [`infra/azure-devops/`](../../../infra/azure-devops/main.tf). The project is public, so a contributor whose fork pull request fails can read the log. CI holds no secrets: credentials are read from the environment at apply time, and fork pull requests get no secrets because none exist.

The site build stays on GitHub Actions ([`site.yml`](../../../.github/workflows/site.yml)), where it is already a required check. CD to Azure is decided with the Azure demo-host ADR.

## Alternatives considered

- **GitHub Actions, as planned.** Equally capable for these checks, and simpler for contributors. Rejected for a single platform alongside the Azure deployment, and for reuse of a setup that already works.
- **Both platforms running the same checks.** Rejected: two configurations that drift, and two places to look when a check fails.
- **A private Azure DevOps project.** Rejected. Fork contributors would see a red check they cannot open.

## Consequences

Easier: one platform for CI and CD on Azure, and a pipeline proven in another project.

Harder: contributors meet a second tool. The organization must allow public projects and request the free parallel-job grant. The first setup has a manual step: installing the GitHub App and importing its connection.

Committed: the checks in `azure-pipelines-ci.yml` stay identical to `just ci`. The CI check names are marked required on `main` once the pipeline first reports.
