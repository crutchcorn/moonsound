import { Component, inject, input, output } from "@angular/core";
import { Metadata } from "../../../injectables/metadata";
import { setVolume } from "../../../services/music";

@Component({
  selector: "volume-slider",
  templateUrl: "./volume-slider.html",
  styleUrl: "./volume-slider.scss",
})
export class VolumeSlider {
  metadata = inject(Metadata);
  volume = input.required<number>();
  backClick = output();

  onVolumeChange(event: Event) {
    const target = event.target as HTMLInputElement;
    const value = parseFloat(target.value);
    void setVolume(value);
  }
}
