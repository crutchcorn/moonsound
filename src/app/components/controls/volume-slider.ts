import { Component, inject, input, output } from "@angular/core";
import { Metadata } from "../../injectables/metadata";
import { setVolume } from "../../services/music";

@Component({
  selector: "app-volume-slider",
  template: `
    <div
      class="volumeRow"
      [style.background-color]="metadata.coverImagePalette.data()?.DarkVibrant?.hex"
    >
      <button class="iconButton" (click)="backClick.emit()">
        <span class="backArrow">‹</span>
      </button>
      <input
        type="range"
        class="volumeSlider"
        min="0"
        max="1"
        step="0.01"
        [value]="volume()"
        (input)="onVolumeChange($event)"
      />
      <button class="iconButton">
        <img height="24" width="24" src="/assets/volume_icon.svg" alt="Volume" />
      </button>
    </div>
  `,
  styles: [
    `
      :host {
        display: block;
        width: 100%;
      }

      .volumeRow {
        display: flex;
        justify-content: center;
        align-items: center;
        gap: 12px;
        padding: 12px 1rem;
        border-radius: 14px;
      }

      .iconButton {
        background: none;
        border: none;
        padding: 12px;
        cursor: pointer;
        display: flex;
        align-items: center;
        justify-content: center;
      }

      .backArrow {
        font-size: 28px;
        color: rgba(255, 255, 255, 0.8);
        line-height: 1;
      }

      .volumeSlider {
        flex: 1;
        height: 8px;
        border-radius: 1.5rem;
        appearance: none;
        background: rgba(0, 0, 0, 0.12);
        box-shadow:
          0px -0.5px 1px 0px rgba(255, 255, 255, 0.3) inset,
          0px -0.5px 1px 0px rgba(255, 255, 255, 0.25) inset,
          0px 1.5px 4px 0px rgba(0, 0, 0, 0.08) inset,
          0px 1.5px 4px 0px rgba(0, 0, 0, 0.1) inset;
      }

      .volumeSlider::-webkit-slider-thumb {
        appearance: none;
        width: 16px;
        height: 16px;
        border-radius: 50%;
        background: rgba(255, 255, 255, 0.9);
        cursor: pointer;
        box-shadow: 0px 2px 4px rgba(0, 0, 0, 0.2);
        margin-top: -4px;
      }

      .volumeSlider::-webkit-slider-runnable-track {
        height: 8px;
        border-radius: 1.5rem;
      }
    `,
  ],
})
export class VolumeSlider {
  metadata = inject(Metadata);
  volume = input.required<number>();
  backClick = output();

  onVolumeChange(event: Event) {
    const target = event.target as HTMLInputElement;
    const value = parseFloat(target.value);
    setVolume(value);
  }
}
