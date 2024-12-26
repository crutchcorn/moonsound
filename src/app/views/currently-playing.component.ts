import {Component, computed, inject, input} from '@angular/core';
import {injectSelector} from '@reduxjs/angular-redux';
import {RootState} from '../store';
import {pause, play, resume, seek} from "../services/music";
import {pickSong} from "../services/fs";
import {Metadata} from "../injectables/metadata";
import {injectMutation} from "@tanstack/angular-query-experimental";

function zeroPad(num: number, places: number) {
  return String(num).padStart(places, '0');
}

@Component({
  selector: "currently-playing-material",
  template: `
      <div class="coverImgBg" style="background-image: {{stateMetadata.urlCoverImage()}}"></div>
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

              @if (openFileMutation.data() && stateMetadata.metadata.isSuccess() && openFileMutation.isSuccess()) {
                  <album-art [src]="stateMetadata.urlCoverImage()"></album-art>
                  <div class="infoContainer">
                      <div class="textContainer">
                          <h1 class="title">{{ stateMetadata.metadata.data()?.tags?.TrackTitle ?? "Unknown Title" }}</h1>
                          <p class="artist">{{ stateMetadata.metadata.data()?.tags?.AlbumArtist ?? "Unknown Artist" }}</p>
                          <p class="album">{{ stateMetadata.metadata.data()?.tags?.Album ?? "Unknown Album" }}</p>
                      </div>

                      <div class="progressContainer">
                          <div class="progressTextContainer">
                              <p class="progressText">{{ currentTimeFormatted() }}</p>
                              <p class="progressText">{{ totalTimeFormatted() }}</p>
                          </div>
                          <progress class="progressBar" (click)="seekFromProgressBar($event)" value="{{currentSecs()}}"
                                    max="{{totalSecs()}}"></progress>
                      </div>

                      <div>
                          <div class="playPauseBtnContainer">
                              @if (playingMetadata().paused) {
                                  <button class="playPauseBtn" (click)="resume()">
                                  </button>
                                  <img class="playPauseIcon" src="/assets/play_icon.svg" alt="Play"/>
                              } @else {
                                  <button class="playPauseBtn" (click)="pause()">
                                  </button>
                                  <img class="playPauseIcon" src="/assets/pause_icon.svg" alt="Pause"/>
                              }
                          </div>
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
          padding: 8px 0;
          flex-direction: column;
          gap: 8px;
      }

      .progressTextContainer {
          display: flex;
          justify-content: space-between;
      }
      
      .progressText {
          font-size: 14px;
          color: white;
          opacity: 0.5;
          margin: 0;
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

      .playPauseBtnContainer {
          position: relative;
          width: fit-content;
          margin: -12px 0;
      }

      .playPauseBtn {
          height: 72px;
          width: 72px;
          border-radius: 2.125rem;
          border: 1px solid var(--Angled-Stroke, rgba(255, 255, 255, 0.20));
          aspect-ratio: 1;
          background: #FFF;
          mix-blend-mode: overlay;

          /* Blur + Shadow Big */
          box-shadow: 0px 7px 10px 0px rgba(0, 0, 0, 0.05), 0px 40px 80px 0px rgba(0, 0, 0, 0.08);
          backdrop-filter: blur(50px);
      }

      .playPauseIcon {
          pointer-events: none;
          position: absolute;
          top: 50%;
          left: 50%;
          transform: translate(-50%, -50%);
          width: 32px;
          height: 32px;
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

  totalSecs = computed(() => {
    return this.playingMetadata().duration?.secs;
  })

  currentSecs = computed(() => {
    return this.playingMetadata().position?.secs;
  });

  totalTime = computed(() => {
    // Format as `hh:mm:ss` but only show minutes and hours if they are non-zero
    const duration = this.playingMetadata().duration?.secs;
    if (!duration) {
      return {
        hours: 0,
        minutes: 0,
        seconds: 0
      };
    }
    // Format
    const hours = Math.floor(duration / 3600);
    const minutes = Math.floor(duration / 60) % 60;
    const seconds = duration % 60;
    return {
      hours,
      minutes,
      seconds
    }
  })

  totalTimeFormatted = computed(() => {
    const duration = this.totalTime();
    return `${duration.hours ? duration.hours + ':' : ''}${zeroPad(duration.minutes, 2)}:${zeroPad(duration.seconds, 2)}`;
  });

  currentTimeFormatted = computed(() => {
    // Format as `hh:mm:ss` but only if `totalTime` has an hour or minute value
    const totalDuration = this.totalTime();
    const duration = this.playingMetadata().position?.secs;
    if (!duration) {
      return '0:00';
    }
    // Format
    const hours = Math.floor(duration / 3600);
    const minutes = Math.floor(duration / 60) % 60;
    const seconds = duration % 60;
    const showHours = totalDuration.hours > 0 || hours > 0;
    const showMinutes = totalDuration.minutes > 0 || minutes > 0;
    return `${showHours ? hours + ':' : ''}${showMinutes ? zeroPad(minutes, 2) : ''}:${zeroPad(seconds, 2)}`;
  })

  resume = resume;
  pause = pause;

  seekFromProgressBar(event: MouseEvent) {
    const progressBar = event.target as HTMLProgressElement;
    const value = event.offsetX / progressBar.offsetWidth * parseInt(progressBar.getAttribute('max') || '0');
    seek(value);
  }
}
