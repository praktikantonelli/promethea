# run cargo tests
test:
  cargo test --manifest-path backend/Cargo.toml

# run cargo clippy
clippy:
  cargo clippy --manifest-path backend/Cargo.toml
