# Promethea

This project aims to be a replacement for a collection of tools I use regularly for managing my ebook library, among which are calibre and calibre-web. While both of those tools are amazing, the fact that I need at least those two to get everything I want is enough reason for me to try and improve my situation with something new.

## Features

As this project is still in its early phases, I'm focusing on creating a minimum viable product that can replace calibre and calibre-web, as well as improve on some key missing features. This includes

- [ ] Adding/Editing/Deleting ebooks in my library
- [ ] Managing metadata associated with each book
- [ ] Having books that may belong to multiple series/universes
- [ ] Having proper views of authors and series
- [ ] Using automatic fetching of metadata with sensible standards and ways of replacing data

## Technical Planning

Currently, I'm using the following app components and versions to build everything:

- Tauri CLI: 4.6.2
- pnpm: 10.18.0

As soon as I reach the point where I can use Promethea as a complete replacement for calibre, I'll start using it as such and will look into and work on other improvements. For example, in order to replace calibre-web as well (which is also a goal but not as important as replacing calibre), I'll need to create a backend service (e.g. with the axum crate) that uses the same frontend and offers alternative implementations for all the backend stuff handled by Tauri.

### Commit Linting

To enforce cleaner commit messages, I've decided to try commit linting with husky and commitlint. Whenever a commit message is created via `git commit -m "some-message"`, these two tools enforce the structure `"type(scope): message"` where `type` can be things such as `"feat"`, `"fix"`, `"revert"` etc. The `"scope"` is intended to show _where_ the type of change has been made roughly. Finally, the message is just any type of message.

Since both husky and commitlint were added to the project via `pnpm`, the packages should be automatically installed on any system that uses this code base. As such, no additional setup effort should be required.

# Debugging

Managed to run the debugger using nvim-dap with overseer:

1. Use overseer to run the task defined in `.vscode/tasks.json`: `:OverseerRun`, then select `dev server (vite)`
2. Launch the debugger by opening `main.rs` and pressing F5

Theoretically, it should be possible to automatically launch the Vite dev server when debugging starts, but I haven't managed it yet.

Also works in VS Code by starting vite server with `pnpm vite` and then clicking the "Debug" button in main function
