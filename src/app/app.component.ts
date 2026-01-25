import { Component, effect } from "@angular/core";

import { RouterOutlet } from "@angular/router";
import { injectDispatch, injectSelector } from "@reduxjs/angular-redux";
import { Event, listen } from "@tauri-apps/api/event";
import { setPosition, musicSync, Duration } from "./store/tauri";
import { AppDispatch, RootState } from "./store";
import { Metadata } from "./injectables/metadata";
import { injectQuery } from "@tanstack/angular-query-experimental";
import { getMenu } from "./utils/menu";

@Component({
  selector: "app-root",
  imports: [RouterOutlet],
  providers: [Metadata],
  template: `
    <router-outlet></router-outlet>
  `,
})
export class App {
  dispatch = injectDispatch<AppDispatch>();

  _musicSync = effect((onCleanup) => {
    this.dispatch(musicSync());

    // Listen for the server to tell us to sync the music player, like when the song changes or song is paused
    let unlisten = () => {};

    listen("SERVER_SYNC_EVENT", (_event) => {
      this.dispatch(musicSync());
    }).then((listener) => {
      unlisten = listener;
    });

    onCleanup(() => {
      unlisten();
    });
  });

  tauri = injectSelector((state: RootState) => state.tauri);

  menuQuery = injectQuery(() => ({
    queryKey: ["menu"],
    queryFn: async () => {
      const menu = await getMenu();

      await menu.setAsAppMenu();
      // Success
      return true;
    },
  }));

  _logErrors = effect(() => {
    const menuError = this.menuQuery.error();
    if (menuError) {
      console.error(menuError);
    }
  });

  // Allows us to get the position of the music player. Probably not the best way to do this.
  _getPos = effect((onCleanup) => {
    if (this.tauri().paused) return;
    if (this.tauri().currentlyPlayingPath === null) return;

    let unlisten = () => {};

    listen("PLAYBACK_POSITION_UPDATE", (event: Event<Duration>) => {
      this.dispatch(setPosition(event.payload));
    }).then((listener) => {
      unlisten = listener;
    });

    onCleanup(() => {
      unlisten();
    });
  });
}
