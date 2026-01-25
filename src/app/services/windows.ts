// This is the module for application windows, not Windows (the OS) interop.
import { WebviewWindow } from "@tauri-apps/api/webviewWindow";
import { invoke } from "@tauri-apps/api/core";

export async function launchWindow(
  label: string,
  windowOptions?: ConstructorParameters<typeof WebviewWindow>[1],
) {
  const window = new WebviewWindow(label, windowOptions);

  void window
    .once("tauri://created", function () {
      setTimeout(() => {
        invoke("make_window_effect", {
          name: label,
        });
      });
    })
    .catch((err) => console.error(err));

  void window
    .once("tauri://error", function (e) {
      console.error(e);
    })
    .catch((err) => console.error(err));

  return window;
}

export async function launchSettingsWindow() {
  return await launchWindow("settings", {
    title: "Settings",
    center: true,
    fullscreen: false,
    // Eventually want to change this to `false`
    resizable: true,
    alwaysOnTop: true,
    focus: true,
    focusable: true,
    skipTaskbar: true,
    visible: true,
    hiddenTitle: true,
    titleBarStyle: "overlay",
    url: "/settings",
  });
}
