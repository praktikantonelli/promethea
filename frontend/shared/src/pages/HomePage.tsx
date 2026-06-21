import { useEffect, useState } from "react";
import { useApi } from "../ApiProvider";

export function HomePage() {
  const api = useApi();
  const [message, setMessage] = useState("Loading...");

  useEffect(() => {
    let alive = true;

    api
      .hello()
      .then((text) => {
        if (alive) setMessage(text);
      })
      .catch((error: unknown) => {
        if (!alive) return;

        if (error instanceof Error) {
          setMessage(error.message);
        } else {
          setMessage("Unknown error");
        }
      });

    return () => {
      alive = false;
    };
  }, [api]);

  return (
    <section className="rounded-2xl border border-slate-200 bg-white p-6 shadow-sm">
      <h2 className="text-xl font-semibold text-slate-950">Home</h2>
      <p className="mt-3 text-slate-700">{message}</p>
    </section>
  );
}
