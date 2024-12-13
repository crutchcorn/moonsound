import { Component, computed, signal } from '@angular/core';
import { invoke } from "@tauri-apps/api/core";
import { open } from '@tauri-apps/plugin-dialog';
import { injectMutation, injectQuery } from '@tanstack/angular-query-experimental';
import { JsonPipe } from '@angular/common';
import { injectSelector } from '@reduxjs/angular-redux';
import { RootState } from '../store';
import { SongMetadata } from '../types/song-metadata';

@Component({
  selector: 'app-home',
  imports: [JsonPipe],
  template: `
    @if (!openFileMutataion.data()) {
      <button (click)="openFileMutataion.mutate()">Play</button>
    }
    @if (openFileMutataion.isError()) {
      <div>Error: {{ openFileMutataion.error | json }}</div>
    }
    @if (mp3Metadata.isError()) {
      <div>Error: {{ mp3Metadata.error | json }}</div>
    }
    @if (openFileMutataion.isPending()) {
      <div>Loading...</div>
    }
    @if (mp3Metadata.isLoading()) {
      <div>Loading song metadata...</div>
    }

    @if (openFileMutataion.data() && openFileMutataion.isSuccess() && mp3Metadata.isSuccess()) {

      <div style="height: 256px; width: 256px; background-size: cover; background-image: {{mp3CoverImage()}}"></div>
      <div>{{ mp3Metadata.data()?.tags?.TrackTitle }}</div>
      <div>{{ mp3Metadata.data()?.tags?.AlbumArtist }}</div>
      <div>{{ mp3Metadata.data()?.tags?.Album }}</div>

      <div>
        @if (stateMetadata().paused) {
          <button (click)="resume()">Resume</button>
        } @else {
          <button (click)="pause()">Pause</button>
        }

        @if (stateMetadata().currentlyPlaying === null) {
          <button (click)="play()">Play</button>
        } @else {
          <button (click)="stop()">Stop</button>
        }
      </div>
    }
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
      // TODO: Move invoke to a service
      return invoke<SongMetadata>("read_mp3_metadata", { path });
    }
  }));

  mp3CoverImage = computed(() => {
    const metadata = this.mp3Metadata.data();
    if (!metadata) return null;
    // uint8array to base64
    const cover = metadata.visuals[Object.keys(metadata.visuals)[0] as keyof typeof metadata.visuals].data;
    if (!cover) return null;
    const base64 = btoa(String.fromCharCode(...cover));
    return `url(data:image/jpeg;base64,${base64})`;
  });

  play() {
    invoke("play", {
      path: this.openFileMutataion.data()
    });
  }

  stop() {
    invoke("stop");
  }

  resume() {
    invoke("resume");
  }

  pause() {
    invoke("pause");
  }
}
