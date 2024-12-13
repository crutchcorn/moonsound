import { Component, computed, signal } from '@angular/core';
import { invoke } from "@tauri-apps/api/core";
import { open } from '@tauri-apps/plugin-dialog';
import { injectMutation, injectQuery } from '@tanstack/angular-query-experimental';
import { JsonPipe } from '@angular/common';
import { injectSelector } from '@reduxjs/angular-redux';
import { RootState } from '../store';

@Component({
  selector: 'app-home',
  imports: [JsonPipe],
  template: `
  @if (!openFileMutataion.data()) {
   <button (click)="openFileMutataion.mutate()">Open File</button>
  } @else {
    @if (openFileMutataion.isError()) {
      <div>Error: {{openFileMutataion.error()}}</div>
    } @else if (openFileMutataion.isPending() || mp3Metadata.isPending()) {
      <div>Loading...</div>
    } @else {
      <div>Selected File: {{mp3Metadata.data() | json}}</div>
        <button (click)="openFileMutataion.mutate()">Open File</button>
        <button (click)="play()">Play</button>
        <button (click)="stop()">Stop</button>
    }
  }

  <code>{{stateMetadata() | json}}</code>
  `,
})
export class Home {
  stateMetadata = injectSelector((state: RootState) => state.tauri);

  openFileMutataion = injectMutation(() => ({
    mutationKey: ['openFile'],
    mutationFn: async () => {
      const result = await open({
        multiple: false,
        directory: false,
        filters: [{ name: 'MP3', extensions: ['mp3'] }]
      });
      return result;
    }
  }));

  mp3Metadata = injectQuery(() => ({
    queryKey: ['mp3Metadata', this.openFileMutataion.data()],
    queryFn: async () => {
      const path = this.openFileMutataion.data();
      if (!path) return null;
      return invoke<string>("read_mp3_metadata", { path });
    }
  }));

  play() {
    invoke("play_sound", {
      path: this.openFileMutataion.data()
    });
  }

  stop() {
    invoke("stop_sound");
  }
}
