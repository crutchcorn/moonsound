import { Component, input } from "@angular/core";

@Component({
  selector: "album-art",
  templateUrl: "./album-art.html",
  styleUrl: "./album-art.scss",
})
export class AlbumArt {
  src = input<string | null>();
}
