import { Component, inject, input, output } from "@angular/core";
import { Metadata } from "../../../injectables/metadata";
import { PlayPauseBtn } from "./play-pause-btn";

@Component({
  selector: "play-pause",
  imports: [PlayPauseBtn],
  templateUrl: "./play-pause.html",
  styleUrl: "./play-pause.scss",
})
export class PlayPause {
  metadata = inject(Metadata);
  resumeit = output();
  pauseit = output();
  volumeClick = output();
  isPaused = input.required<boolean>();
}
