import { Component, effect } from '@angular/core';
import { invoke } from "@tauri-apps/api/core";

@Component({
  selector: 'app-home',
  template: `
   <p>Home</p>
  `,
})
export class Home {
  _mp3 = effect(() => {
    const path = `/Users/crutchcorn/Documents/cute.mp3`;
    invoke<string>("read_mp3_metadata", { path }).then((data) => {
      console.log({data})
    }).catch(console.error);  
  })
}
