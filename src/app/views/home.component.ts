import { Component, computed } from '@angular/core';
import { invoke } from "@tauri-apps/api/core";
import { open } from '@tauri-apps/plugin-dialog';
import { injectMutation, injectQuery } from '@tanstack/angular-query-experimental';
import { JsonPipe } from '@angular/common';
import { injectSelector } from '@reduxjs/angular-redux';
import { RootState } from '../store';
import { SongMetadata } from '../types/song-metadata';

@Component({
  // TODO: Rename from "home" to "now-playing"
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

      @if (mp3CoverImage()) {
        <div style="height: 256px; width: 256px; background-size: cover; background-image: {{mp3CoverImage()}}"></div>
      }
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

        <div>
          <progress (click)="seekFromProgressBar($event)" value="{{currentTime()}}" max="{{totalTime()}}"></progress>
        </div>
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
        filters: [{ name: 'MP3', extensions: ['mp3'] }, { name: 'FLAC', extensions: ['flac'] }]
      });
      return result;
    }
  }));

  currentTime = computed(() => {
    return this.stateMetadata().position?.secs;
  })

  totalTime = computed(() => {
    return this.stateMetadata().duration?.secs;
  })

  mp3Metadata = injectQuery(() => ({
    queryKey: ['mp3Metadata', this.openFileMutataion.data()],
    queryFn: async () => {
      const path = this.openFileMutataion.data();
      if (!path) return null;
      // TODO: Move invoke to a service
      return invoke<SongMetadata>("read_metadata", { path });
    }
  }));

  mp3CoverImage = computed(() => {
    const metadata = this.mp3Metadata.data();
    if (!metadata) return null;
    // uint8array to base64
    const cover = metadata.visuals[Object.keys(metadata.visuals)[0] as keyof typeof metadata.visuals | never]?.data;
    if (!cover) return null;
    return cover;
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

  seekFromProgressBar(event: MouseEvent) {
    const progressBar = event.target as HTMLProgressElement;
    const value = event.offsetX / progressBar.offsetWidth * parseInt(progressBar.getAttribute('max') || '0');
    invoke("seek_to", {
      position: {
        secs: Math.floor(value),
        nanos: 0
      }
    });
  }
}
