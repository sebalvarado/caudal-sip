---
name: worktree-management
description: Creates, hands off, and cleans up git worktrees for parallel agent work in Caudal using Worktrunk (wt). Use when starting work on a GitHub issue, spawning a subagent, running several agents at once, or cleaning up merged branches.
---

# Worktree management

One GitHub issue, one branch, one worktree, one PR. Worktrees let several agents build at once without touching each other's files, `target/`, or database.

## Before you create a worktree

1. **Check the gate.** If the issue is labelled `blocked`, stop. Implementation issues stay blocked until the Domain foundations epic (#17) is merged; only decision drafts run before that.
2. **Check scope.** The issue's *May touch* list is the boundary. Two worktrees whose *May touch* lists overlap must not run at the same time.

## Setup (once per machine)

```bash
brew install worktrunk just sccache && wt config shell install
rustup show                       # installs the toolchain pinned in rust-toolchain.toml
wt config state default-branch set main
wt hook approvals add             # approve the project hooks in .config/wt.toml
```

Optional, faster cold builds across worktrees: `export RUSTC_WRAPPER=sccache` in your shell profile. Do **not** share one `CARGO_TARGET_DIR` between worktrees; concurrent builds fight over its lock and invalidate each other.

## Create

```bash
wt switch -c issue-9-ledger-core   # branch name: issue-<number>-<slug>
```

The hooks in `.config/wt.toml` then:

- **post-create** (blocking): `cargo fetch`, and `just db` creates this worktree's own database `caudal_<directory>` on the shared `caudal-postgres` container.
- **post-start** (background): copies the gitignored files listed in `.worktreeinclude`.
- **pre-merge**: runs `just ci`.
- **pre-remove**: drops the worktree's database.

`just` reads `DATABASE_URL` for the current worktree; never hard-code a database name.

## Hand off to a subagent

Write `WORKTREE_CONTEXT.md` in the worktree root (it is gitignored), then launch the agent there with "Read WORKTREE_CONTEXT.md and execute the task."

```markdown
# Worktree task

## Issue
#<number> — <title> · <url>

## Objective
<one paragraph, copied from the issue's Goal>

## May touch
<copied from the issue; nothing else>

## Read first
<copied from the issue, plus the crate's AGENTS.md>

## Acceptance
<copied from the issue>

## Out of scope
<what to leave alone, especially other crates>

## When done
1. Run the self-verification ladder, ending with `just ci`
2. Commit with `git commit -s`, imperative subject, `Refs #<number>`
3. `git push -u origin HEAD && gh pr create --fill`, then link the issue
4. Report: what changed, how it was verified, anything left open
```

## Finish

After the PR merges: `wt remove <branch>`. Bulk cleanup of merged branches: `wt step prune --dry-run`, then without `--dry-run`. If a worktree was deleted by hand: `git worktree prune`.

## Troubleshooting

| Problem | Fix |
|---|---|
| Hooks did not run | `wt hook approvals add`, then `wt hook show` |
| No database | `just db` |
| Slow first build | Expected per worktree; enable `sccache` |
| Port clash with the demo API | Use `{{ branch \| hash_port }}` from wt for the port |
