import {Component, computed, inject, input} from '@angular/core';
import {injectSelector} from '@reduxjs/angular-redux';
import {RootState} from '../store';
import {pause, play, resume, seek} from "../services/music";
import {pickSong} from "../services/fs";
import {Metadata} from "../injectables/metadata";
import {injectMutation} from "@tanstack/angular-query-experimental";

@Component({
  selector: "currently-playing-material",
  template: `
      <div class="coverImgBg" style="background-image: {{stateMetadata.mp3CoverImage()}}"></div>
      <div class="scrim"></div>
      <ng-content></ng-content>
  `,
  styles: [`
      .coverImgBg {
          position: fixed;
          z-index: -2;
          top: 0;
          left: 0;
          width: 100%;
          height: 100%;
          background-size: cover;
          filter: blur(90px);
      }

      .scrim {
          position: fixed;
          z-index: -1;
          top: 0;
          left: 0;
          width: 100%;
          height: 100%;
          background-size: 50px 50px;
          background-repeat: repeat;
          background-position: 0% 0%;
          background-color: #222222;
          mix-blend-mode: overlay;
          backdrop-filter: blur(27px);
      }
  `]
})
class CurrentlyPlayingMaterial {
  stateMetadata = inject(Metadata);
}

@Component({
  selector: "album-art",
  template: `
      <div class="album-art-container">
          <div class="album-art-bg" [style.background-image]="src()"></div>
          <div class="album-art-border"></div>
          <div class="album-art-fg" [style.background-image]="src()"></div>
      </div>
  `,
  styles: [`
      .album-art-container {
          position: relative;
          width: fit-content;
          --border-size: 10px;
          --art-size: 212px;
          --border-radius: 16px;
          height: var(--art-size);
          width: var(--art-size);
          border-radius: var(--border-radius);
          overflow: hidden;
          box-shadow: 0px -0.5px 1px 0px rgba(255, 255, 255, 0.30) inset, 0px -0.5px 1px 0px rgba(255, 255, 255, 0.25) inset, 0px 1.5px 4px 0px rgba(0, 0, 0, 0.08) inset, 0px 1.5px 4px 0px rgba(0, 0, 0, 0.10) inset;
      }

      .album-art-bg {
          background-size: var(--art-size);
          background-repeat: no-repeat;
          background-position: center;
          position: absolute;
          top: 0;
          left: 0;
          right: 0;
          bottom: 0;
      }

      .album-art-border {
          position: absolute;
          top: 0;
          left: 0;
          right: 0;
          bottom: 0;
          background-image: linear-gradient(180deg, rgba(255, 255, 255, 0.2) 0%, rgba(255, 255, 255, 0) 41%, rgba(255, 255, 255, 0) 57%, rgba(255, 255, 255, 0.1) 100%);
          /*background-color: lightblue;*/
      }

      .album-art-fg {
          background-size: var(--art-size);
          background-repeat: no-repeat;
          background-position: center;
          position: absolute;
          top: var(--border-size);
          left: var(--border-size);
          right: var(--border-size);
          bottom: var(--border-size);
          border-radius: calc(var(--border-radius) - var(--border-size));
      }
  `]
})
export class AlbumArt {
  src = input<string | null>();
}

@Component({
  // TODO: Rename from "home" to "now-playing"
  selector: 'app-home',
  imports: [CurrentlyPlayingMaterial, AlbumArt],
  template: `
      <currently-playing-material>
          <div class="container">
              @if (!openFileMutation.data()) {
                  <button (click)="openFileMutation.mutate()">Play</button>
              }

              @if (openFileMutation.data() && stateMetadata.mp3Metadata.isSuccess() && openFileMutation.isSuccess()) {
                  <album-art [src]="stateMetadata.mp3CoverImage()"></album-art>
                  <div class="infoContainer">
                      <div class="textContainer">
                          <h1 class="title">{{ stateMetadata.mp3Metadata.data()?.tags?.TrackTitle ?? "Unknown Title" }}</h1>
                          <p class="artist">{{ stateMetadata.mp3Metadata.data()?.tags?.AlbumArtist ?? "Unknown Artist" }}</p>
                          <p class="album">{{ stateMetadata.mp3Metadata.data()?.tags?.Album ?? "Unknown Album" }}</p>
                      </div>

                      <div class="progressContainer">
                          <progress class="progressBar" (click)="seekFromProgressBar($event)" value="{{currentTime()}}"
                                    max="{{totalTime()}}"></progress>
                      </div>

                      <div>
                          @if (playingMetadata().paused) {
                              <button (click)="resume()">Resume</button>
                          } @else {
                              <button (click)="pause()">Pause</button>
                          }
                      </div>
                  </div>
              }
          </div>
      </currently-playing-material>
  `,
  styles: [`
      .container {
          padding: 24px;
          display: flex;
          flex-direction: row;
          overflow: hidden;
          justify-content: center;
          align-items: center;
          /* 32 because of top area */
          height: calc(100vh - 32px);
          box-sizing: border-box;
      }

      .infoContainer {
          display: flex;
          flex-direction: column;
          gap: 20px;
          justify-content: center;
          align-items: center;
          padding: 0 32px;
          text-align: center;
          width: 1px;
          flex-grow: 1;
      }

      .textContainer {
          display: flex;
          flex-direction: column;
          gap: 8px;
          justify-content: center;
          align-items: center;
          width: 100%;
      }

      .title {
          font-size: 24px;
          font-weight: bold;
          margin: 0;
          color: white;
          width: 100%;
          overflow: hidden;
          white-space: nowrap;
          text-overflow: ellipsis;
      }

      .artist {
          font-size: 16px;
          margin: 0;
          color: white;
          opacity: 0.8;
      }

      .album {
          font-size: 16px;
          margin: 0;
          color: white;
          opacity: 0.8;
      }

      .progressContainer {
          width: 100%;
      }

      .progressBar {
          border-radius: 1.5rem;
          width: 100%;
          box-shadow: 0px -0.5px 1px 0px rgba(255, 255, 255, 0.30) inset, 0px -0.5px 1px 0px rgba(255, 255, 255, 0.25) inset, 0px 1.5px 4px 0px rgba(0, 0, 0, 0.08) inset, 0px 1.5px 4px 0px rgba(0, 0, 0, 0.10) inset;
          background-blend-mode: plus-darker;
          height: 0.5rem;
          appearance: none;
      }

      .progressBar::-webkit-progress-bar {
          height: 100%;
          background: rgba(0, 0, 0, 0.12);
          padding: 0;
          margin: 0;
      }

      .progressBar::-webkit-progress-value {
          border-radius: 1.25rem;
          background: rgba(255, 255, 255, 0.80);
          mix-blend-mode: plus-lighter;
          height: 100%;
          padding: 0;
          margin: 0;
      }
  `]
})
export class CurrentlyPlaying {
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
