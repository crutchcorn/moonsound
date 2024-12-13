import { Component } from '@angular/core';
import { invoke } from "@tauri-apps/api/core";
import { open } from '@tauri-apps/plugin-dialog';
import { injectMutation, injectQuery } from '@tanstack/angular-query-experimental';
import { JsonPipe } from '@angular/common';

@Component({
  selector: 'app-home',
  imports: [JsonPipe],
  template: `
  @if (!openFileMutataion.data()) {
   <button (click)="openFileMutataion.mutate()">Open File</button>
  } @else {
    @if (openFileMutataion.isError()) {
      <div>Error: {{openFileMutataion.error()}}</div>
    } @else if (openFileMutataion.isPending() || mp3Metadata.isPending()) {
      <div>Loading...</div>
    } @else {
      <div>Selected File: {{mp3Metadata.data() | json}}</div>
      <button (click)="openFileMutataion.mutate()">Open File</button>
    }
  }
  `,
})
export class Home {
  openFileMutataion = injectMutation(() => ({
    mutationKey: ['openFile'],
    mutationFn: async () => {
      const result = await open({
        multiple: false,
        directory: false,
        filters: [{ name: 'MP3', extensions: ['mp3'] }]
      });
      return result;
    }
  }));

  mp3Metadata = injectQuery(() => ({
    queryKey: ['mp3Metadata', this.openFileMutataion.data()],
    queryFn: async () => {
      const path = this.openFileMutataion.data();
      if (!path) return null;
      return invoke<string>("read_mp3_metadata", { path });
    }
  }));
}
