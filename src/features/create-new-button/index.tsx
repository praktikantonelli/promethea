import { Button } from "@/components/ui/button";
import { info } from "@tauri-apps/plugin-log";
import { open } from "@tauri-apps/plugin-dialog";
import { invoke } from "@tauri-apps/api/core";

async function createNew() {
  const folderPath = await open({
    multiple: false,
    directory: true,
  })
  invoke("create_new_db", { folder: folderPath })
}

export default function CreateNewButton() {
  return (
    <Button onClick={() => {
      info("Create new button was clicked");
      createNew();
    }}>
      Create New
    </Button>
  )
}
