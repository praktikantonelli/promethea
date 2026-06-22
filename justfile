set shell := ['nu', '-c']
build backend:
  cargo build --manifest-path backend/Cargo.toml

run target:
  @just _run-{{target}}

[private]
_run-frontend:
  pnpm --dir frontend dev

[private]
_run-backend:
  cargo run --manifest-path backend/Cargo.toml
