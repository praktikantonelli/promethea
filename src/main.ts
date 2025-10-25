import { open } from '@tauri-apps/plugin-dialog';
import { invoke } from '@tauri-apps/api/core';

async function createNew() {
  const folderPath = await open({
    multiple: false,
    directory: true,
  })
  console.log(folderPath);
  invoke('create_new_db', { folder: folderPath });
}

async function openExisting() {
  const filePath = await open({
    filters: [{ name: 'library', extensions: ['db', 'sqlite', 'db3'] }],
    multiple: false,
    directory: false,
  })
  console.log(filePath);
  invoke('open_existing_db', { path: filePath });
}

const existing_button = document.getElementById('open-existing-btn');
const new_button = document.getElementById('create-new-btn');

new_button?.addEventListener('click', () => {
  console.log('new button clicked');
  createNew();
});

existing_button?.addEventListener('click', () => {
  console.log('existing button clicked');
  openExisting();
})
