import { Component, inject, input, output } from "@angular/core";
import { Metadata } from "../../../injectables/metadata";

@Component({
  selector: "play-pause-btn",
  templateUrl: "./play-pause-btn.html",
  styleUrl: "./play-pause-btn.scss",
})
export class PlayPauseBtn {
  metadata = inject(Metadata);
  resumeit = output();
  pauseit = output();
  isPaused = input.required<boolean>();
}
