import { invoke } from "@tauri-apps/api/core";

export function play(fsPath: string) {
  return invoke<void>("play", {
    path: fsPath,
  });
}

export function addFolder(fsPath: string) {
  return invoke<void>("add_folder", {
    path: fsPath,
  });
}

// TODO: Implement this function
export function importSong(_fsPath: string) {}

export function stop() {
  return invoke<void>("stop");
}

export function resume() {
  return invoke<void>("resume");
}

export function pause() {
  return invoke<void>("pause");
}

export function seek(seconds: number) {
  return invoke("seek_to", {
    position: {
      secs: Math.floor(seconds),
      nanos: 0,
    },
  });
}

export function setVolume(volume: number) {
  return invoke<void>("set_volume", {
    volume: Math.max(0, Math.min(1, volume)),
  });
}
