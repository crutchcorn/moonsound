import { Component, computed, inject, input, signal } from "@angular/core";
import { injectSelector } from "@reduxjs/angular-redux";
import { CurrentlyPlayingMaterial } from "../../components/currently-playing-material/currently-playing-material";
import { AlbumArt } from "../../components/album-art/album-art";
import { PlayPause } from "../../components/controls/play-pause/play-pause";
import { VolumeSlider } from "../../components/controls/volume-slider/volume-slider";
import { Metadata } from "../../injectables/metadata";
import { ControlType } from "../../types/currently-playing";
import { RootState } from "../../store";
import { injectMutation } from "@tanstack/angular-query-experimental";
import { pickSong } from "../../services/fs";
import { pause, play, resume, seek } from "../../services/music";
import { zeroPad } from "../../utils/strings";

@Component({
  selector: "currently-playing",
  imports: [CurrentlyPlayingMaterial, AlbumArt, PlayPause, VolumeSlider],
  templateUrl: "./currently-playing.html",
  styleUrl: "currently-playing.scss",
})
export class CurrentlyPlaying {
  playingMetadata = injectSelector((state: RootState) => state.tauri);
  stateMetadata = inject(Metadata);

  controls = signal<ControlType>("play-pause");

  openFileMutation = injectMutation(() => ({
    mutationKey: ["openFile"],
    mutationFn: async () => {
      const path = await pickSong();
      if (!path) {
        return false;
      }
      await play(path);
      return true;
    },
  }));

  totalSecs = computed(() => {
    return this.playingMetadata().duration?.secs;
  });

  currentSecs = computed(() => {
    return this.playingMetadata().position?.secs;
  });

  progressPercent = computed(() => {
    const current = this.currentSecs() ?? 0;
    const total = this.totalSecs() ?? 0;
    if (total === 0) return 0;
    return (current / total) * 100;
  });

  totalTime = computed(() => {
    // Format as `hh:mm:ss` but only show minutes and hours if they are non-zero
    const duration = this.playingMetadata().duration?.secs;
    if (!duration) {
      return {
        hours: 0,
        minutes: 0,
        seconds: 0,
      };
    }
    // Format
    const hours = Math.floor(duration / 3600);
    const minutes = Math.floor(duration / 60) % 60;
    const seconds = duration % 60;
    return {
      hours,
      minutes,
      seconds,
    };
  });

  totalTimeFormatted = computed(() => {
    const duration = this.totalTime();
    return `${duration.hours ? duration.hours + ":" : ""}${zeroPad(duration.minutes, 2)}:${zeroPad(duration.seconds, 2)}`;
  });

  currentTimeFormatted = computed(() => {
    // Format as `hh:mm:ss` but only if `totalTime` has an hour or minute value
    const totalDuration = this.totalTime();
    const duration = this.playingMetadata().position?.secs;
    if (!duration) {
      return "0:00";
    }
    // Format
    const hours = Math.floor(duration / 3600);
    const minutes = Math.floor(duration / 60) % 60;
    const seconds = duration % 60;
    const showHours = totalDuration.hours > 0 || hours > 0;
    const showMinutes = totalDuration.minutes > 0 || minutes > 0;
    return `${showHours ? hours + ":" : ""}${showMinutes ? zeroPad(minutes, 2) : ""}:${zeroPad(seconds, 2)}`;
  });

  isPlayingASong = computed(() => !!this.playingMetadata().currentlyPlayingPath);

  resume = () => {
    // Need to select a file to play
    if (!this.isPlayingASong()) {
      this.openFileMutation.mutate(undefined, {
        onSuccess: () => {
          void resume();
        },
      });
      return;
    }
    void resume();
  };
  pause = () => {
    void pause();
  };

  seekFromInput(event: Event) {
    const input = event.target as HTMLInputElement;
    const value = parseFloat(input.value);
    void seek(value);
  }
}
