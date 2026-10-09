# Caudal task runner. `just` lists the recipes.
# Recipes that need the Cargo workspace say so until it lands (#5).

set dotenv-load

# One Postgres container serves every worktree; each worktree gets its own database.
pg_container := "caudal-postgres"
db_name := "caudal_" + replace_regex(file_name(justfile_directory()), "[^A-Za-z0-9]+", "_")
export DATABASE_URL := env("DATABASE_URL", "postgres://postgres@localhost:5432/" + db_name)

default:
    @just --list

# Check the tools this repo expects are installed
doctor:
    #!/usr/bin/env bash
    for t in cargo rustup just wt docker node npm sccache cargo-nextest cargo-deny; do
      if command -v "$t" >/dev/null; then printf '  ok       %s\n' "$t"; else printf '  missing  %s\n' "$t"; fi
    done

# Start the shared Postgres container and create this worktree's database
db:
    #!/usr/bin/env bash
    set -euo pipefail
    if ! docker ps --format '{{{{.Names}}' | grep -qx {{pg_container}}; then
      docker start {{pg_container}} 2>/dev/null || docker run -d --name {{pg_container}} -p 5432:5432 \
        -e POSTGRES_HOST_AUTH_METHOD=trust postgres:16 >/dev/null
      until docker exec {{pg_container}} pg_isready -U postgres >/dev/null 2>&1; do sleep 0.5; done
    fi
    docker exec {{pg_container}} psql -U postgres -tc "SELECT 1 FROM pg_database WHERE datname='{{db_name}}'" | grep -q 1 \
      || docker exec {{pg_container}} createdb -U postgres {{db_name}}
    echo "DATABASE_URL=$DATABASE_URL"

# Drop this worktree's database (run before removing the worktree)
db-drop:
    docker exec {{pg_container}} dropdb -U postgres --if-exists {{db_name}}

# Fast type check of one crate, or the whole workspace
check crate="":
    cargo check {{ if crate == "" { "--workspace" } else { "-p " + crate } }} --all-targets

# Tests for one crate, or the whole workspace
test crate="":
    cargo test {{ if crate == "" { "--workspace" } else { "-p " + crate } }}

# Format the workspace
fmt:
    cargo fmt --all

# The done-gate: everything CI runs. Once, before opening a PR
ci: db
    cargo fmt --all --check
    cargo clippy --workspace --all-targets -- -D warnings
    cargo test --workspace
    cargo deny check
    just site-build

# Run the caudal terminal (lands with #12)
terminal *args:
    cargo run -p caudal -- {{args}}

# Website dev server
site:
    cd site && npm ci --silent && npm run dev

# Website production build
site-build:
    cd site && npm ci --silent && npm run build
