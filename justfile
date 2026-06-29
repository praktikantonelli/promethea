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

test target:
  @just _test-{{target}}

[private]
_test-backend:
  cargo test --manifest-path backend/Cargo.toml
