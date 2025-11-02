import { Button } from "@/components/ui/button";
import { info } from "@tauri-apps/plugin-log";
import { open } from "@tauri-apps/plugin-dialog";
import { invoke } from "@tauri-apps/api/core";

async function openExisting() {
  const filePath = await open({
    filters: [{ name: "library", extensions: ["db", "sqlite", "db3"] }],
    multiple: false,
    directory: false
  })
  invoke("open_existing_db", { path: filePath });
}

export default function OpenExistingButton() {
  return (
    <Button onClick={() => {
      info("Open existing button was clicked");
      openExisting();
    }}>
      Open Existing
    </Button>
  )
}
