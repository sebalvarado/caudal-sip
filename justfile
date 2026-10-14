# Caudal task runner. `just` lists the recipes.

set dotenv-load

# One Postgres container serves every worktree; each worktree gets its own database.
# Not 5432, so it never collides with another project's Postgres. Override with CAUDAL_PG_PORT.
pg_container := "caudal-postgres"
pg_port := env("CAUDAL_PG_PORT", "55432")
db_name := "caudal_" + replace_regex(file_name(justfile_directory()), "[^A-Za-z0-9]+", "_")
export DATABASE_URL := env("DATABASE_URL", "postgres://postgres@localhost:" + pg_port + "/" + db_name)

default:
    @just --list

# Check the tools this repo expects are installed
doctor:
    #!/usr/bin/env bash
    for t in cargo rustup just wt docker node npm sccache cargo-nextest cargo-deny wasm-bindgen; do
      if command -v "$t" >/dev/null; then printf '  ok       %s\n' "$t"; else printf '  missing  %s\n' "$t"; fi
    done

# Start the shared Postgres container and create this worktree's database
db:
    #!/usr/bin/env bash
    set -euo pipefail
    if ! docker ps --format '{{{{.Names}}' | grep -qx {{pg_container}}; then
      docker start {{pg_container}} 2>/dev/null || docker run -d --name {{pg_container}} -p {{pg_port}}:5432 \
        -e POSTGRES_HOST_AUTH_METHOD=trust postgres:16 >/dev/null
      until docker exec {{pg_container}} pg_isready -h localhost -U postgres >/dev/null 2>&1; do sleep 0.5; done
    fi
    if ! docker port {{pg_container}} 5432 | grep -q ":{{pg_port}}$"; then
      echo "{{pg_container}} is not published on port {{pg_port}}: free the port or set CAUDAL_PG_PORT," >&2
      echo "then recreate it with: docker rm -f {{pg_container}} && just db" >&2
      exit 1
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

# Tests for one crate, or the whole workspace. Starts Postgres for the adapter tests
test crate="": db
    cargo test {{ if crate == "" { "--workspace" } else { "-p " + crate } }}

# Format the workspace
fmt:
    cargo fmt --all

# The done-gate: everything CI runs. Once, before opening a PR
ci: db
    cargo fmt --all --check
    cargo clippy --workspace --all-targets -- -D warnings
    cargo test --workspace
    cargo xtask arch
    cargo xtask playground --check
    cargo clippy -p caudal --lib --target wasm32-unknown-unknown -- -D warnings
    node site/scripts/playground-smoke.mjs
    cargo deny check
    just site-build

# The network in one process: no args opens the full-screen terminal, e.g. `just terminal pay --from banco-alfa --llave llave-01 --amount 25.40`
terminal *args:
    cargo run -p caudal -- {{args}}

# Run a scenario script, e.g. `just run my-day.caudal`; see `just terminal help` for the language
run file *args:
    cargo run --release -p caudal -- run {{file}} {{args}}

# Rebuild the browser simulator in site/public/simulador/pkg. Needs `cargo install wasm-bindgen-cli --version 0.2.129`
playground:
    cargo xtask playground

# Regenerate the published one-day national simulation
simulation:
    cargo run --release -p caudal -- scenario national --json > site/src/data/simulacion-nacional.json

# Website dev server
site:
    cd site && npm ci --silent && npm run dev

# Website production build
site-build:
    cd site && npm ci --silent && npm run build
