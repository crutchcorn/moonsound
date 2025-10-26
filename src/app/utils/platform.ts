import { platform } from "@tauri-apps/plugin-os";

export function getPlatform() {
  try {
    return platform();
  } catch {
    return "web";
  }
}
