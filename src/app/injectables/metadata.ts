import {computed, Injectable} from "@angular/core";
import {injectSelector} from "@reduxjs/angular-redux";
import {RootState} from "../store";
import {injectQuery} from "@tanstack/angular-query-experimental";
import {invoke} from "@tauri-apps/api/core";
import {SongMetadata} from "../types/song-metadata";

// TODO: Remove this in favor of using the `@reduxjs/toolkit` package and RTK Query (not out yet for Angular)
@Injectable({providedIn: null})
export class Metadata {
  path = injectSelector((state: RootState) => state.tauri.currentlyPlayingPath);

  mp3Metadata = injectQuery(() => ({
    queryKey: ['mp3Metadata', this.path()],
    queryFn: async () => {
      const path = this.path();
      if (!path) return null;
      return await invoke<SongMetadata>("read_metadata", { path });
    }
  }));

  mp3CoverImage = computed(() => {
    const metadata = this.mp3Metadata.data();
    if (!metadata) return null;
    const cover = metadata.visuals[Object.keys(metadata.visuals)[0] as keyof typeof metadata.visuals | never]?.data;
    if (!cover) return null;
    // url(base64)
    return cover;
  });
}
