import { NavLink, Outlet, Route, Routes } from "react-router";
import type { ApiClient } from "./api";
import { ApiProvider } from "./ApiProvider";
import { HomePage } from "./pages/HomePage";
import { AboutPage } from "./pages/AboutPage";

export function App({ api }: { api: ApiClient }) {
  return (
    <ApiProvider api={api}>
      <Routes>
        <Route path="/" element={<Layout />}>
          <Route index element={<HomePage />} />
          <Route path="about" element={<AboutPage />} />
          <Route path="*" element={<NotFoundPage />} />
        </Route>
      </Routes>
    </ApiProvider>
  );
}

function Layout() {
  return (
    <main className="min-h-screen bg-slate-50 px-6 py-12 text-slate-950">
      <div className="mx-auto max-w-3xl">
        <header className="mb-8">
          <p className="text-sm font-medium uppercase tracking-wide text-slate-500">
            My app
          </p>

          <h1 className="mt-2 text-4xl font-bold tracking-tight">
            React + Axum starter
          </h1>

          <nav className="mt-6 flex gap-2">
            <NavItem to="/" label="Home" end />
            <NavItem to="/about" label="About" />
          </nav>
        </header>

        <Outlet />
      </div>
    </main>
  );
}

function NavItem({
  to,
  label,
  end = false,
}: {
  to: string;
  label: string;
  end?: boolean;
}) {
  return (
    <NavLink
      to={to}
      end={end}
      className={({ isActive }) =>
        [
          "rounded-lg px-3 py-2 text-sm font-medium transition",
          isActive
            ? "bg-slate-950 text-white"
            : "text-slate-600 hover:bg-slate-200 hover:text-slate-950",
        ].join(" ")
      }
    >
      {label}
    </NavLink>
  );
}

function NotFoundPage() {
  return (
    <section className="rounded-2xl border border-red-200 bg-red-50 p-6">
      <h2 className="text-xl font-semibold text-red-950">Not found</h2>
      <p className="mt-3 text-red-800">This page does not exist.</p>
    </section>
  );
}
