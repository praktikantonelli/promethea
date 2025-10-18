import { open } from '@tauri-apps/plugin-dialog';

async function createNew() {
  const folder = await open({
    multiple: false,
    directory: true,
  })
  console.log(folder);
}

const button = document.getElementById('btn');
const new_button = document.getElementById('create-new-btn');
button?.addEventListener('click', function handleClick(event) {
  console.log('button clicked');
  console.log(event);
  console.log(event.target);
});

new_button?.addEventListener('click', () => {
  console.log('new button clicked');
  createNew();
});
