import { invoke } from "@tauri-apps/api/core";
import { load } from "@tauri-apps/plugin-store";
import { open } from "@tauri-apps/plugin-dialog";

let createNewButton: HTMLElement | null;
let openExistingButton: HTMLElement | null;

async function create_new() {
  const folder = await open({
    multiple: false,
    directory: true,
  });
  console.log(folder);
}

async function open_existing() {
  const file = await open({
    multiple: false,
    directory: false,
  });
  console.log(file);
}

window.addEventListener("DOMContentLoaded", () => {
  createNewButton = document.getElementById("#btn-create-new");
  openExistingButton = document.getElementById("#btn-open-existing");



  createNewButton?.addEventListener("click", () => {
    create_new();
  });

  openExistingButton?.addEventListener("click", () => {
    open_existing();
  });

})
