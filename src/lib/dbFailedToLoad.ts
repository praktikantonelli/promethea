"use client"

import { useCallback, useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

export function useDbFailure() {
  const [failed, setFailed] = useState(false);
  const [loading, setLoading] = useState(false);

  const refresh = useCallback(async () => {
    setLoading(true);
    try {
      const ok = await invoke<boolean>("database_loading_failed");
      setFailed(ok)
    } catch {
      setFailed(true)
    } finally {
      setLoading(false)
    }
  }, []);
  useEffect(() => {
    refresh()
  }, [refresh])

  return { failed, loading, refresh, setFailed }
}
