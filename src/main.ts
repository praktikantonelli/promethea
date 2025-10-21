import { open } from '@tauri-apps/plugin-dialog';

async function createNew() {
  const folder = await open({
    multiple: false,
    directory: true,
  })
  console.log(folder);
}

async function openExisting() {
  const file = await open({
    filters: [{ name: 'library', extensions: ['db', 'sqlite', 'db3'] }],
    multiple: false,
    directory: false,
  })
  console.log(file);
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
