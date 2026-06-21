set shell := ['nu', '-c']
build backend:
  cargo build --manifest-path backend/Cargo.toml

run backend:
  cargo run --manifest-path backend/Cargo.toml

