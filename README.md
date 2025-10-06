# Tauri + Vanilla TS

This template should help get you started developing with Tauri in vanilla HTML, CSS and Typescript.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

# Windows

Currently using the following programs and versions:

- Tauri CLI: 4.6.2
- pnpm: 10.18.0

# Debugging

Managed to run the debugger using nvim-dap with overseer:

1. Use overseer to run the task defined in `.vscode/tasks.json`: `:OverseerRun`, then select `dev server (vite)`
2. Launch the debugger by opening `main.rs` and pressing F5

Theoretically, it should be possible to automatically launch the Vite dev server when debugging starts, but I haven't managed it yet.

Also works in VS Code by starting vite server with `pnpm vite` and then clicking the "Debug" button in main function