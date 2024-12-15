import {Component, computed, inject} from '@angular/core';
import {JsonPipe} from '@angular/common';
import {injectSelector} from '@reduxjs/angular-redux';
import {RootState} from '../store';
import {pause, play, resume, seek} from "../services/music";
import {pickSong} from "../services/fs";
import {Metadata} from "../injectables/metadata";
import {injectMutation} from "@tanstack/angular-query-experimental";

@Component({
  // TODO: Rename from "home" to "now-playing"
  template: `
      @if (!openFileMutation.data()) {
          <button (click)="openFileMutation.mutate()">Play</button>
      }
      @if (openFileMutation.isError()) {
          <div>Error: {{ openFileMutation.error | json }}</div>
      }
      @if (stateMetadata.mp3Metadata.isError()) {
          <div>Error: {{ stateMetadata.mp3Metadata.error | json }}</div>
      }
      @if (openFileMutation.isPending()) {
          <div>Loading...</div>
      }
      @if (stateMetadata.mp3Metadata.isLoading()) {
          <div>Loading song metadata...</div>
      }

      @if (openFileMutation.data() && stateMetadata.mp3Metadata.isSuccess() && openFileMutation.isSuccess()) {

          @if (stateMetadata.mp3CoverImage()) {
              <div style="height: 256px; width: 256px; background-size: cover; background-image: {{stateMetadata.mp3CoverImage()}}"></div>
          }
          <div>{{ stateMetadata.mp3Metadata.data()?.tags?.TrackTitle }}</div>
          <div>{{ stateMetadata.mp3Metadata.data()?.tags?.AlbumArtist }}</div>
          <div>{{ stateMetadata.mp3Metadata.data()?.tags?.Album }}</div>

          <div>
              @if (playingMetadata().paused) {
                  <button (click)="resume()">Resume</button>
              } @else {
                  <button (click)="pause()">Pause</button>
              }

              <div>
                  <progress (click)="seekFromProgressBar($event)" value="{{currentTime()}}"
                            max="{{totalTime()}}"></progress>
              </div>
          </div>
      }
  `,
  selector: 'app-home',
  imports: [JsonPipe],
})
export class Home {
  playingMetadata = injectSelector((state: RootState) => state.tauri);
  stateMetadata = inject(Metadata);

  openFileMutation = injectMutation(() => ({
    mutationKey: ['openFile'],
    mutationFn: async () => {
      const path = await pickSong();
      if (!path) {
        return;
      }
      await play(path);
      return true
    }
  }))

  currentTime = computed(() => {
    return this.playingMetadata().position?.secs;
  })

  totalTime = computed(() => {
    return this.playingMetadata().duration?.secs;
  })

  resume = resume;
  pause = pause;

  seekFromProgressBar(event: MouseEvent) {
    const progressBar = event.target as HTMLProgressElement;
    const value = event.offsetX / progressBar.offsetWidth * parseInt(progressBar.getAttribute('max') || '0');
    seek(value);
  }
}
