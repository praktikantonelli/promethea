import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle
} from "@/components/ui/dialog";
import { Button } from "@/components/ui/button";
import { debug, info, error } from "@tauri-apps/plugin-log";
import { open as dialogOpen } from "@tauri-apps/plugin-dialog";
import { invoke } from "@tauri-apps/api/core";
import { useCallback, useEffect, useState } from "react";

type DbInitStatus =
  | { status: "loaded" }
  | { status: "needs_setup"; reason?: string };

function useDbInitStatus() {
  const [status, setStatus] = useState<DbInitStatus | null>(null);

  useEffect(() => {
    let alive = true;
    invoke<DbInitStatus>("get_init_status")
      .then((s) => alive && setStatus(s))
      .catch(() => alive && setStatus({ status: "needs_setup", reason: "query failed" }));
    return () => {
      alive = false;
    };
  }, []);

  const refresh = useCallback(async () => {
    const s = await invoke<DbInitStatus>("get_init_status");
    setStatus(s);
  }, []);

  return { status, setStatus, refresh };
}

export default function NoDatabaseDialog() {
  const { status, setStatus: _setStatus, refresh } = useDbInitStatus();

  const handleCreateNew = useCallback(async () => {
    try {
      info("Create new button was clicked");
      const folderPath = await dialogOpen({ multiple: false, directory: true });
      if (!folderPath) {
        debug("Create new cancelled by user!");
        return;
      }
      await invoke("create_new_db", { folder: folderPath });
      await refresh();
    } catch (e: any) {
      error(`create_new_db failed: ${e?.message ?? String(e)}`);
    }
  }, [refresh]);

  const handleOpenExisting = useCallback(async () => {
    try {
      info("Open existing button was clicked");
      const filePath = await dialogOpen({ multiple: false, directory: false, filters: [{ name: "library", extensions: ["db", "db3", "sqlite"] }] });
      if (!filePath) {
        debug("Open existing cancelled by user!");
        return;
      }
      await invoke("open_existing_db", { path: filePath });
      await refresh();
    } catch (e: any) {
      error(`open_existing_db failed: ${e?.message ?? String(e)}`)
    }
  }, [refresh]);

  const [open, setOpen] = useState<boolean>(false);
  useEffect(() => {
    if (!status) return;
    setOpen(status.status === "needs_setup");
  }, [status]);


  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>No Database Configured Yet</DialogTitle>
          <DialogDescription>Either select an existing database file or choose a location to create a new one</DialogDescription>
          <div>
            <Button onClick={handleCreateNew}>Create New</Button>
            <Button onClick={handleOpenExisting}>Open Existing</Button>

          </div>
        </DialogHeader>
      </DialogContent>
    </Dialog>
  )
}
