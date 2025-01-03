import {Component, effect} from '@angular/core';
import { CommonModule } from '@angular/common';
import { RouterOutlet } from '@angular/router';
import { injectDispatch, injectSelector } from "@reduxjs/angular-redux";
import {Event, listen} from '@tauri-apps/api/event';
import {setPosition, musicSync, Duration} from './store/tauri';
import { AppDispatch, RootState } from './store';
import { UnderTitlebar } from './components/under-titlebar.component';
import {Metadata} from "./injectables/metadata";

@Component({
  selector: 'app-root',
  imports: [CommonModule, RouterOutlet, UnderTitlebar],
  providers: [Metadata],
  template: `
    <app-under-titlebar/>
    <router-outlet></router-outlet>
  `,
})
export class App {
  dispatch = injectDispatch<AppDispatch>();

  _musicSync = effect((onCleanup) => {
    this.dispatch(musicSync());

    // Listen for the server to tell us to sync the music player, like when the song changes or song is paused
    let unlisten = () => { };

    listen('SERVER_SYNC_EVENT', (event) => {
      this.dispatch(musicSync())
    }).then((listener) => {
      unlisten = listener;
    });

    onCleanup(() => {
      unlisten();
    });
  })

  tauri = injectSelector((state: RootState) => state.tauri);

  // Allows us to get the position of the music player. Probably not the best way to do this.
  _getPos = effect((onCleanup) => {
    if (this.tauri().paused) return;
    if (this.tauri().currentlyPlayingPath === null) return;

    let unlisten = () => { };

    listen('PLAYBACK_POSITION_UPDATE', (event: Event<Duration>) => {
      this.dispatch(setPosition(event.payload));
    }).then((listener) => {
      unlisten = listener;
    });

    onCleanup(() => {
        unlisten();
    });
  })
}
