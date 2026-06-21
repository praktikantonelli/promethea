import { createContext, useContext } from "react";
import type { ReactNode } from "react";
import type { ApiClient } from "./api";

const ApiContext = createContext<ApiClient | null>(null);

export function ApiProvider({
  api,
  children,
}: {
  api: ApiClient;
  children: ReactNode;
}) {
  return <ApiContext.Provider value={api}>{children}</ApiContext.Provider>;
}

export function useApi(): ApiClient {
  const api = useContext(ApiContext);

  if (api === null) {
    throw new Error("useApi must be used inside ApiProvider");
  }

  return api;
}
