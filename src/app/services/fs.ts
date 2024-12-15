import {open} from "@tauri-apps/plugin-dialog";

export function pickSong() {
  return open({
    multiple: false,
    directory: false,
    filters: [{ name: 'MP3', extensions: ['mp3'] }, { name: 'FLAC', extensions: ['flac'] }]
  });
}
