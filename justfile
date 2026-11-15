set shell := ['nu', '-c']
build backend:
  cargo build --manifest-path backend/Cargo.toml

dev:
  cargo watch -C backend -w . -w ../frontend -x 'run -p server'

_test-backend:
  cargo test --manifest-path backend/Cargo.toml

clippy:
  cargo clippy --workspace --all-targets --all-features --manifest-path backend/Cargo.toml -- -D warnings
