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


function useDbInitStatus() {
  const [ready, setReady] = useState<boolean>(false);

  useEffect(() => {
    let alive = true;
    invoke<boolean>("get_init_status")
      .then((ready) => { if (alive) setReady(ready) })
      .catch(() => { if (alive) setReady(false) });
    return () => {
      alive = false;
    };
  }, []);

  const refresh = useCallback(async () => {
    const ready = await invoke<boolean>("get_init_status");
    setReady(ready);
  }, []);

  return { ready, setReady, refresh };
}

export default function NoDatabaseDialog() {
  const { ready, setReady: _setReady, refresh } = useDbInitStatus();

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

  const [open, setOpen] = useState<boolean>(true);
  useEffect(() => {
    setOpen(!ready);
  }, [ready]);


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
