# Frontend

This is a Nx workspace that contains all frontend code for Promethea. It is split as follows:

```sh
frontend/
├── apps/web/            # web frontend package
│   └── src/
│       ├── routes/      # TanStack Router routes
│       └── main.tsx     # entry point
├── apps/desktop         # future desktop frontend package
├── packages/ui/         # shared React library for both platforms
│   └── src/
│       ├── components   # shadcn/ui components
│       ├── hooks        # shared hooks
│       └── pages        # shared pages, displayed with TanStack Router
└── packages/lib/        # shared TypeScript library
    └── src/
        └── lib          # shared types, types generated from Rust backend (HTTP responses etc.)
```

## Architecture

Each platform has its own `app` that tries to define as little logic as possible. The goal is to have a mostly identical user interface, regardless of platform.

Each platform defines its own route tree for TanStack Router, simply because the routes will not be identical. The components behind a route are to be defined in `packages/ui/src/pages` if they're present on both platforms. As such, the route tree on each platform defines all available routes, but each route will mostly be a thin wrapper around a main component defined in `packages/ui/src/pages`.

Shared TypeScript code goes into `packages/lib/src`. This includes shared types and maybe also types inferred/generated from backend code. For example, if the backend returns a custom type on some HTTP request, it might make sense to auto-generate an equivalent type, for example with [1password's typeshare](https://github.com/1Password/typeshare).

## Common Nx Commands

Display a graph of all apps and packages:

```sh
pnpm dlx nx graph
```

Run the dev server:

```sh
pnpm dlx nx serve web
```

Create a production bundle:

```sh
pnpm dlx nx build web
```

See all available targets to run for a project:

```sh
pnpm dlx nx show project web
```

These targets are either [inferred automatically](https://nx.dev/concepts/inferred-tasks?utm_source=nx_project&utm_medium=readme&utm_campaign=nx_projects) or defined in the `project.json` or `package.json` files.

## Add new projects

Generate a new React application:

```sh
pnpm dlx nx g @nx/react:app demo
```

Generate a new React library:

```sh
pnpm dlx nx g @nx/react:lib mylib
```

List all installed Nx plugins:

```sh
pnpm dlx nx list
```

Learn more about a plugin:

```sh
pnpm dlx nx list <plugin-name>
```
