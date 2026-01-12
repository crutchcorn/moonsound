import { Component, inject, input, output } from "@angular/core";
import { Metadata } from "../../injectables/metadata";

@Component({
  selector: "app-play-pause-btn",
  template: `
    <div
      class="playPauseBtnContainer"
      style="
          --bg-color: {{ metadata.coverImagePalette.data()?.Vibrant?.hex }};
          --color: {{
        metadata.coverImagePalette.data()?.Vibrant?.titleTextColor
      }};   
        "
    >
      @if (isPaused()) {
        <button class="playPauseBtn" (click)="resumeit.emit()">
          <span class="visually-hidden">Resume</span>
        </button>
        <img class="playPauseIcon" src="/assets/play_icon.svg" alt="Play" />
      } @else {
        <button class="playPauseBtn" (click)="pauseit.emit()">
          <span class="visually-hidden">Pause</span>
        </button>
        <img class="playPauseIcon" src="/assets/pause_icon.svg" alt="Pause" />
      }
    </div>
  `,
  styles: [
    `
      :host {
        display: inline-flex;
        height: 1px;
        overflow: visible;
        justify-content: center;
        align-items: center;
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
        border: 1px solid var(--Angled-Stroke, rgba(255, 255, 255, 0.2));
        aspect-ratio: 1;

        background-color: var(--bg-color);
        color: var(--color);

        /* Blur + Shadow Big */
        box-shadow:
          0px 7px 10px 0px rgba(0, 0, 0, 0.05),
          0px 40px 80px 0px rgba(0, 0, 0, 0.08);
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
    `,
  ],
})
export class PlayPauseBtn {
  metadata = inject(Metadata);
  resumeit = output();
  pauseit = output();
  isPaused = input.required<boolean>();
}

@Component({
  selector: "app-play-pause",
  imports: [PlayPauseBtn],
  template: `
    <div
      class="buttonsRow"
      style="background-color: {{
        metadata.coverImagePalette.data()?.DarkVibrant?.hex
      }}"
    >
      <button class="invisibleButton" (click)="volumeClick.emit()">
        <img height="24" width="24" src="/assets/volume_icon.svg" alt="Volume" />
      </button>
      <app-play-pause-btn
        [isPaused]="isPaused()"
        (resumeit)="resumeit.emit()"
        (pauseit)="pauseit.emit()"
      />
    </div>
  `,
  styles: [
    `
      .buttonsRow {
        display: flex;
        justify-content: center;
        align-items: center;
        gap: 12px;
        padding-left: 1rem;
        padding-right: 1rem;
        border-radius: 14px;
      }

      .invisibleButton {
        background: none;
        border: none;
        padding: 12px;
      }
    `,
  ],
})
export class PlayPause {
  metadata = inject(Metadata);
  resumeit = output();
  pauseit = output();
  volumeClick = output();
  isPaused = input.required<boolean>();
}
