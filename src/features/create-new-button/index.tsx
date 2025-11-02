import { Button } from "@/components/ui/button";
import { info } from "@tauri-apps/plugin-log";
export default function CreateNewButton() {
  return (
    <Button onClick={() => {
      info("Create new button was clicked");
    }}>
      Create New
    </Button>
  )
}
