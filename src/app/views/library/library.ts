import { Component, effect, inject } from "@angular/core";
import { LightGlassOutlined } from "../../components/light-glass-outlined/light-glass-outlined";
import { setBodyBg } from "../../utils/styling";
import { Metadata } from "../../injectables/metadata";
import { SidebarItem } from "./components/sidebar-item/sidebar-item";
import { injectSelector } from "@reduxjs/angular-redux";
import { RootState } from "../../store";
import { AlbumArt } from "../../components/album-art/album-art";
import { RouterLink } from "@angular/router";

@Component({
  selector: "library-view",
  templateUrl: "./library.html",
  styleUrl: "./library.scss",
  imports: [LightGlassOutlined, SidebarItem, AlbumArt, RouterLink],
})
export class Library {
  _ = effect(() => {
    setBodyBg("#060606");
  });

  stateMetadata = inject(Metadata);
  playingMetadata = injectSelector((state: RootState) => state.tauri);
}
