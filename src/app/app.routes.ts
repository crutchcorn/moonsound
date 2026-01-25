import { Routes } from "@angular/router";
import { AppLayout } from "./layouts/app-layout";
import { CurrentlyPlaying } from "./views/currently-playing/currently-playing";
import { Settings } from "./views/settings/settings";
import { Library } from "./views/library/library";

export const routes: Routes = [
  {
    path: "",
    component: AppLayout,
    children: [
      { path: "", component: Library },
      { path: "playing", component: CurrentlyPlaying },
      {
        path: "settings",
        component: Settings,
      },
    ],
  },
];
