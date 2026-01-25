import { computed, Injectable } from "@angular/core";
import { injectSelector } from "@reduxjs/angular-redux";
import { RootState } from "../store";
import { injectQuery } from "@tanstack/angular-query-experimental";
import { invoke } from "@tauri-apps/api/core";
import { SongMetadata } from "../types/song-metadata";
import { Vibrant } from "node-vibrant/browser";

export const defaultCover = "/assets/default_album_art.svg";

// TODO: Remove this in favor of using the `@reduxjs/toolkit` package and RTK Query (not out yet for Angular)
@Injectable({ providedIn: null })
export class Metadata {
  path = injectSelector((state: RootState) => state.tauri.currentlyPlayingPath);

  metadata = injectQuery(() => ({
    queryKey: ["metadata", this.path()],
    queryFn: async () => {
      const path = this.path();
      if (!path) return null;
      return await invoke<SongMetadata>("read_metadata", { path });
    },
  }));

  coverImage = computed(() => {
    const metadata = this.metadata.data();
    if (!metadata) return defaultCover;
    const cover =
      metadata.visuals[
        Object.keys(metadata.visuals)[0] as
          | keyof typeof metadata.visuals
          | never
      ]?.data;
    if (!cover) return defaultCover;
    // url(base64)
    return cover;
  });

  urlCoverImage = computed(() => {
    const cover = this.coverImage();
    if (!cover) return null;
    return `url(${cover})`;
  });

  coverImagePalette = injectQuery(() => ({
    queryKey: ["metadata", this.path(), this.coverImage()],
    queryFn: async () => {
      const cover = this.coverImage();
      if (!cover) return null;
      const el = document.createElement("img");
      el.src = cover;
      const res = await Vibrant.from(el).getPalette();
      return res;
    },
  }));
}
