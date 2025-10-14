
import { invoke } from "@tauri-apps/api/core";
import { load } from "@tauri-apps/plugin-store";
import { open } from "@tauri-apps/plugin-dialog";
import { join } from "@tauri-apps/api/path";

// ---- DOM refs ----
let dbSetupEl: HTMLElement | null;
let btnCreateNew: HTMLButtonElement | null;
let btnOpenExisting: HTMLButtonElement | null;

const STORE_FILE = "promethea-config.json";
const STORE_KEY = "library_path";
const DEFAULT_DB_FILENAME = "my_app.db";

type DbStatus = {
  has_path: boolean;
  db_path: string | null;
  ready: boolean;
};

// ---- store helpers ----
async function readStoredPath(): Promise<string | null> {
  const store = await load(STORE_FILE);
  const raw = (await store.get(STORE_KEY)) as unknown;
  if (!raw) return null;
  if (typeof raw === "string") return raw;
  if (typeof raw === "object" && raw !== null && "value" in raw) {
    const v = (raw as { value?: unknown }).value;
    return typeof v === "string" ? v : null;
  }
  return null;
}

async function writeStoredPath(path: string) {
  const store = await load(STORE_FILE);
  await store.set(STORE_KEY, { value: path }); // keep your existing shape
  await store.save();
}

// ---- backend helpers ----
async function initializeBackendWithPath(path: string) {
  // rename if your command is different
  await invoke("set_db_path", { path });
}

async function getBackendStatus(): Promise<DbStatus> {
  try {
    return await invoke<DbStatus>("get_db_status");
  } catch {
    return {
      has_path: !!(await readStoredPath()),
      db_path: await readStoredPath(),
      ready: true,
    };
  }
}

// ---- UI wiring ----

// "Create new database…" → pick directory, create DEFAULT_DB_FILENAME inside
async function onCreateNewClicked() {
  console.log("Create new clicked!");
  const dir = await open({
    directory: true,
    multiple: false,
    title: "Choose a folder for the new SQLite database",
  });
  if (!dir || Array.isArray(dir)) return;

  const dbPath = await join(dir, DEFAULT_DB_FILENAME);
  await writeStoredPath(dbPath);
  await initializeBackendWithPath(dbPath);
  await syncVisibility();
  alert(`Created new database at:\n${dbPath}`);
}

// "Open existing…" → pick an existing .db / .sqlite file
async function onOpenExistingClicked() {
  const picked = await open({
    multiple: false,
    directory: false,
    title: "Open an existing SQLite database file",
    filters: [{ name: "SQLite Database", extensions: ["db", "sqlite"] }],
  });
  if (!picked || Array.isArray(picked)) return;

  const dbPath = picked as string;
  await writeStoredPath(dbPath);
  await initializeBackendWithPath(dbPath);
  await syncVisibility();
  alert(`Using existing database:\n${dbPath}`);
}

// Show/hide setup block & manual form based on whether a path exists
async function syncVisibility() {
  const stored = await readStoredPath();
  const status = await getBackendStatus();

  if (dbSetupEl) dbSetupEl.hidden = !!stored; // show setup only when missing
  const pathForm = document.querySelector<HTMLFormElement>("#path-form");
  if (pathForm) pathForm.hidden = !stored ? true : false; // hide manual form until configured

  // Optional: reflect current path somewhere else in your UI
  console.log("DB status:", status);
}

window.addEventListener("DOMContentLoaded", () => {
  // setup buttons
  dbSetupEl = document.querySelector("#db-setup");
  btnCreateNew = document.querySelector("#btn-create-new");
  btnOpenExisting = document.querySelector("#btn-open-existing");

  btnCreateNew?.addEventListener("click", onCreateNewClicked);
  btnOpenExisting?.addEventListener("click", onOpenExistingClicked);
});

window.addEventListener("DOMContentLoaded", async () => {
  await syncVisibility();

  // First run: if no path, setup section stays visible.
  const stored = await readStoredPath();
  if (!stored) {
    // Do nothing; let the user choose via the two buttons.
    return;
  }

  // If already configured, ensure backend is initialized (idempotent on your side)
  await initializeBackendWithPath(stored);
  await syncVisibility();
});
