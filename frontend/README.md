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

[Learn more about Nx plugins &raquo;](https://nx.dev/concepts/nx-plugins?utm_source=nx_project&utm_medium=readme&utm_campaign=nx_projects) | [Browse the plugin registry &raquo;](https://nx.dev/plugin-registry?utm_source=nx_project&utm_medium=readme&utm_campaign=nx_projects)

## Set up CI!

### Step 1

To connect to Nx Cloud, run the following command:

```sh
npx nx connect
```

Connecting to Nx Cloud ensures a [fast and scalable CI](https://nx.dev/ci/intro/why-nx-cloud?utm_source=nx_project&utm_medium=readme&utm_campaign=nx_projects) pipeline. It includes features such as:

- [Remote caching](https://nx.dev/ci/features/remote-cache?utm_source=nx_project&utm_medium=readme&utm_campaign=nx_projects)
- [Task distribution across multiple machines](https://nx.dev/ci/features/distribute-task-execution?utm_source=nx_project&utm_medium=readme&utm_campaign=nx_projects)
- [Automated e2e test splitting](https://nx.dev/ci/features/split-e2e-tasks?utm_source=nx_project&utm_medium=readme&utm_campaign=nx_projects)
- [Task flakiness detection and rerunning](https://nx.dev/ci/features/flaky-tasks?utm_source=nx_project&utm_medium=readme&utm_campaign=nx_projects)

### Step 2

Use the following command to configure a CI workflow for your workspace:

```sh
npx nx g ci-workflow
```

[Learn more about Nx on CI](https://nx.dev/ci/intro/ci-with-nx#ready-get-started-with-your-provider?utm_source=nx_project&utm_medium=readme&utm_campaign=nx_projects)

## Install Nx Console

Nx Console is an editor extension that enriches your developer experience. It lets you run tasks, generate code, and improves code autocompletion in your IDE. It is available for VSCode and IntelliJ.

[Install Nx Console &raquo;](https://nx.dev/getting-started/editor-setup?utm_source=nx_project&utm_medium=readme&utm_campaign=nx_projects)

## Useful links

Learn more:

- [Learn more about this workspace setup](https://nx.dev/getting-started/tutorials/react-monorepo-tutorial?utm_source=nx_project&utm_medium=readme&utm_campaign=nx_projects)
- [Learn about Nx on CI](https://nx.dev/ci/intro/ci-with-nx?utm_source=nx_project&utm_medium=readme&utm_campaign=nx_projects)
- [Releasing Packages with Nx release](https://nx.dev/features/manage-releases?utm_source=nx_project&utm_medium=readme&utm_campaign=nx_projects)
- [What are Nx plugins?](https://nx.dev/concepts/nx-plugins?utm_source=nx_project&utm_medium=readme&utm_campaign=nx_projects)

And join the Nx community:

- [Discord](https://go.nx.dev/community)
- [Follow us on X](https://twitter.com/nxdevtools) or [LinkedIn](https://www.linkedin.com/company/nrwl)
- [Our Youtube channel](https://www.youtube.com/@nxdevtools)
- [Our blog](https://nx.dev/blog?utm_source=nx_project&utm_medium=readme&utm_campaign=nx_projects)
