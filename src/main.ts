import { invoke } from "@tauri-apps/api/core";
import { load } from '@tauri-apps/plugin-store';

let greetInputEl: HTMLInputElement | null;
let greetMsgEl: HTMLElement | null;
let pathInputEl: HTMLInputElement | null;

async function greet() {
  if (greetMsgEl && greetInputEl) {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    greetMsgEl.textContent = await invoke("greet", {
      name: greetInputEl.value,
    });
  }
}

async function setPath() {
  if (pathInputEl) {
    let path = pathInputEl.value;
    // set path in store here
    const store = await load("promethea-config.json");
    await store.set("library_path", { value: path });
    await store.save();

    // call backend to to notify about setting path
    await invoke("notify_library_path_set");
  }
}

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  document.querySelector("#greet-form")?.addEventListener("submit", (e) => {
    e.preventDefault();
    greet();
  });
});

window.addEventListener("DOMContentLoaded", () => {
  pathInputEl = document.querySelector("#path-input");
  document.querySelector("#path-form")?.addEventListener("submit", (e) => {
    e.preventDefault();
    setPath();
  });
});
