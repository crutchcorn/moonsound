import { Component } from "@angular/core";
import { UnderTitlebar } from "../components/under-titlebar.component";
import { RouterOutlet } from "@angular/router";

@Component({
  imports: [UnderTitlebar, RouterOutlet],
  selector: "app-layout",
  template: `
    <app-under-titlebar />
    <router-outlet></router-outlet>
  `,
})
export class AppLayout {}
