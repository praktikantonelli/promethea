import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle
} from "@/components/ui/dialog";
import { Button } from "@/components/ui/button";
import { debug, info } from "@tauri-apps/plugin-log";
import { open } from "@tauri-apps/plugin-dialog";
import { invoke } from "@tauri-apps/api/core";
import { useCallback, useEffect, useState } from "react";

type DbInitStatus =
  | { status: "loaded" }
  | { status: "needs_setup"; reason?: string };

function useDbInitStatus() {
  const [status, setStatus] = useState<DbInitStatus | null>(null);

  useEffect(() => {
    let alive = true;
    invoke<DbInitStatus>("get_init_status").then((s) => alive && setStatus(s)).catch(() => alive && setStatus({ status: "needs_setup", reason: "query failed" })
    );
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

async function createNew() {
  const folderPath = await open({
    multiple: false,
    directory: true,
  })
  invoke("create_new_db", { folder: folderPath })
}

async function openExisting() {
  const filePath = await open({
    filters: [{ name: "library", extensions: ["db", "sqlite", "db3"] }],
    multiple: false,
    directory: false
  })
  invoke("open_existing_db", { path: filePath });
}

export default function NoDatabaseDialog() {
  const { status, setStatus, refresh } = useDbInitStatus();

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
            <Button onClick={() => {
              info("Create new button was clicked");
              createNew();
            }}>
              Create New
            </Button>
            <Button onClick={() => {
              info("Open existing button was clicked");
              openExisting();
            }}>
              Open Existing
            </Button>

          </div>
        </DialogHeader>
      </DialogContent>
    </Dialog>
  )
}
