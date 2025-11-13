import { Button } from "@/components/ui/button";
import { useCallback } from "react";
import { open as dialogOpen } from "@tauri-apps/plugin-dialog";
import { invoke } from "@tauri-apps/api/core";
import { info } from "@tauri-apps/plugin-log";



export default function AddBookButton() {
  const handleAddNewBook = useCallback(async () => {
    info("Add new book button clicked");
    const path = await dialogOpen({ multiple: false, directory: false, filters: [{ name: "Book", extensions: ["epub"] }] });
    await invoke("add_book", { path });
  }, [])

  return (
    <Button onClick={handleAddNewBook}>Add Book</Button>
  )
}
