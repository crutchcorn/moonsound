import { Component, input } from "@angular/core";

@Component({
  selector: "under-titlebar-macos",
  templateUrl: "./under-titlebar-macos.html",
  styleUrl: "./under-titlebar-macos.scss",
})
export class UnderTitlebarMacos {
  theme = input<"light" | "dark">();
}
