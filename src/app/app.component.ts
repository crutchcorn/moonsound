import { Component, effect } from '@angular/core';
import { CommonModule } from '@angular/common';
import { RouterOutlet } from '@angular/router';
import { injectDispatch } from "@reduxjs/angular-redux";
import { listen } from '@tauri-apps/api/event';
import { musicSync } from './store/tauri';
import { AppDispatch } from './store';

@Component({
  selector: 'app-root',
  imports: [CommonModule, RouterOutlet],
  template: `
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

    // // Allows us to get the position of the music player. Probably not the best way to do this.
    // const interval = setInterval(() => {
    //   this.dispatch(musicSync());
    // }, 1000);

    onCleanup(() => {
      // clearInterval(interval)
      unlisten();
    });
  })
}
