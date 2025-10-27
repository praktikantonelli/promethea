import { open } from '@tauri-apps/plugin-dialog';
import { warn, debug, trace, info, error } from '@tauri-apps/plugin-log';
import { invoke } from '@tauri-apps/api/core';

async function createNew() {
  const folderPath = await open({
    multiple: false,
    directory: true,
  })
  await info("${folderPath}");
  invoke('create_new_db', { folder: folderPath });
}

async function openExisting() {
  const filePath = await open({
    filters: [{ name: 'library', extensions: ['db', 'sqlite', 'db3'] }],
    multiple: false,
    directory: false,
  })
  await info("${filePath}");
  invoke('open_existing_db', { path: filePath });
}

const existing_button = document.getElementById('open-existing-btn');
const new_button = document.getElementById('create-new-btn');

new_button?.addEventListener('click', () => {
  info('new button clicked');
  createNew();
});

existing_button?.addEventListener('click', () => {
  info('existing button clicked');
  openExisting();
})
