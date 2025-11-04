import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle
} from "@/components/ui/dialog";
import { Button } from "@/components/ui/button";
import { info } from "@tauri-apps/plugin-log";
import { open } from "@tauri-apps/plugin-dialog";
import { invoke } from "@tauri-apps/api/core";
import { useState } from "react";

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
  const [open, setOpen] = useState<boolean>(true);

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
