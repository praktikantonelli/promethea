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
  const [ready, setReady] = useState<boolean | null>(null);

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

  const handleLibrary = useCallback(async (action: string) => {
    try {
      info(`${action} clicked`);
      let args;
      if (action == "CreateNew") {
        args = { multiple: false, directory: true };
      } else {
        args = { multiple: false, directory: false, filters: [{ name: "library", extensions: ["db", "db3", "sqlite"] }] };
      }
      let pathStr = await dialogOpen(args);
      if (!pathStr) {
        debug("Dialog cancelled by user");
        return;
      }
      await invoke("open_db", { pathStr });
      await refresh();

    } catch (e: any) {
      error(`opening DB failed: ${e?.message ?? String(e)}`);
    }
  }, [refresh]
  )


  const [open, setOpen] = useState<boolean>(false);
  useEffect(() => {
    if (ready !== null) {
      setOpen(!ready);
    }
  }, [ready]);

  if (ready === null) {
    return null;
  }


  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>No Database Configured Yet</DialogTitle>
          <DialogDescription>Either select an existing database file or choose a location to create a new one</DialogDescription>
          <div>
            <Button onClick={() => handleLibrary("CreateNew")}>Create New</Button>
            <Button onClick={() => handleLibrary("OpenExisting")}>Open Existing</Button>

          </div>
        </DialogHeader>
      </DialogContent>
    </Dialog>
  )
}
