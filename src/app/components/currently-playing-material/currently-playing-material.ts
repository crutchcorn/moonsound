import { Component, inject } from "@angular/core";
import { Metadata } from "../../injectables/metadata";

@Component({
  selector: "currently-playing-material",
  templateUrl: "./currently-playing-material.html",
  styleUrl: "./currently-playing-material.scss",
})
export class CurrentlyPlayingMaterial {
  stateMetadata = inject(Metadata);
}
