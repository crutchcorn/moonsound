import { WebviewWindow } from "@tauri-apps/api/webviewWindow";
import { LogicalPosition, LogicalSize } from "@tauri-apps/api/dpi";

export async function launchSettingsWindow() {
  const appWindow = new WebviewWindow("settings", {
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
    url: "/settings",
  });

  void appWindow
    .once("tauri://created", function () {})
    .then(() => {
      appWindow.setSize(
        new LogicalSize({
          height: 640,
          width: 800,
        }),
      );
      appWindow.setPosition(
        new LogicalPosition({
          x: 0,
          y: 0,
        }),
      );
    })
    .catch((err) => console.error(err));

  void appWindow
    .once("tauri://error", function (e) {
      console.error(e);
    })
    .catch((err) => console.error(err));
}
