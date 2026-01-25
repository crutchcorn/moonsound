import { Component, effect } from "@angular/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { injectQuery } from "@tanstack/angular-query-experimental";
import { listen, TauriEvent } from "@tauri-apps/api/event";
import { getPlatform } from "../../utils/platform";
import { UnderTitlebarMacos } from "./under-titlebar-macos";

@Component({
  selector: "under-titlebar",
  imports: [UnderTitlebarMacos],
  templateUrl: "./under-titlebar.html",
})
export class UnderTitlebar {
  currentPlatform = getPlatform();

  theme = injectQuery(() => ({
    queryKey: ["theme"],
    queryFn: async () => (await getCurrentWindow().theme()) ?? "dark",
  }));

  _themeChange = effect((onCleanup) => {
    let cleanup = () => {};
    listen(TauriEvent.WINDOW_THEME_CHANGED, () => {
      this.theme.refetch();
    }).then((c) => (cleanup = c));
    onCleanup(() => cleanup());
  });
}
