# E2E-Tests
This directory contains all end-to-end tests for Promethea. 

## Setup
In order for this to work, there are a couple of prerequisites:
- On Windows, install `msedgedriver` by first running

```
cargo install tauri-driver --locked & cargo install --git https://github.com/chippers/msedgedriver-tool
```

The latter is a tool in order to install `msedgedriver`. When `msedgedriver-tool` is executed, it installs the newest version of `msedgedriver` in the current working directory.
A good place to install this is in `$USER/.cargo/bin`.

To run the tests locally, navigate into `e2e-tests` and run `pnpm test`.
