import { Component, ViewEncapsulation } from "@angular/core";
import { UnderTitlebar } from "../components/under-titlebar.component";
import { RouterOutlet } from "@angular/router";

@Component({
  imports: [UnderTitlebar, RouterOutlet],
  selector: "app-layout",
  template: `
    <app-under-titlebar />
    <div class="__app_layout___mainContents">
      <router-outlet></router-outlet>
    </div>
  `,
  encapsulation: ViewEncapsulation.None,
  styles: [
    `
      app-layout {
        min-height: 100vh;
        display: flex;
        flex-direction: column;
      }

      .__app_layout___mainContents {
        flex-grow: 1;
        height: 1px;
        display: flex;
        flex-direction: column;
      }

      .__app_layout___mainContents > *:not(router-outlet) {
        flex-grow: 1;
        height: 1px;
        display: flex;
        flex-direction: column;
      }
    `,
  ],
})
export class AppLayout {}
