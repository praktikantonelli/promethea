import { Button } from '@workspace/ui/components/button';

export function App() {
  return (
    <main className="min-h-screen bg-background px-6 py-10 text-foreground">
      <section className="mx-auto flex max-w-3xl flex-col gap-6">
        <div className="space-y-2">
          <p className="text-sm font-medium text-muted-foreground">
            Nx + Vite + shadcn/ui
          </p>
          <h1 className="text-3xl font-semibold tracking-normal">
            Promethea frontend
          </h1>
          <p className="max-w-xl text-muted-foreground">
            The web app is importing a shadcn button from the shared UI package.
          </p>
        </div>

        <div>
          <Button>Shared UI button</Button>
        </div>
      </section>
    </main>
  );
}

export default App;
