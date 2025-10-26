import { invoke } from "@tauri-apps/api/core";

export function play(fsPath: string) {
  return invoke<void>("play", {
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
