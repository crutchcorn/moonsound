import { open } from "@tauri-apps/plugin-dialog";

export function pickSong() {
  return open({
    multiple: false,
    directory: false,
    filters: [
      { name: "MP3", extensions: ["mp3"] },
      { name: "FLAC", extensions: ["flac"] },
    ],
  });
}

export function addFolder() {
    return open({
        title: "Add folder",
        directory: true,
        multiple: false,
        recursive: true,
        canCreateDirectories: true,
        //defaultPath: `/Users/${user}/Music`
    })
}