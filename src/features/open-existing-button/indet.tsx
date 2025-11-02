import { Button } from "@/components/ui/button";
import { info } from "@tauri-apps/plugin-log";

export default function OpenExistingButton() {
  return (
    <Button onClick={() => {
      info("Open existing button was clicked");
    }}>
      Open Existing
    </Button>
  )
}
