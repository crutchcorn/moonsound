import { Component, computed, input } from "@angular/core";
import { defaultCover } from "../../injectables/metadata";

@Component({
  selector: "album-art",
  templateUrl: "./album-art.html",
  styleUrl: "./album-art.scss",
})
export class AlbumArt {
  src = input<string | null>();

  srcOrDefault = computed(() => {
    const srcData = this.src();
    if (!srcData) {
      return `url(${defaultCover})`;
    }
    return srcData;
  });
}
