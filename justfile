# run cargo tests
test:
  cargo test --all-targets --locked --manifest-path backend/Cargo.toml

# run cargo clippy
clippy:
  cargo clippy --manifest-path backend/Cargo.toml
