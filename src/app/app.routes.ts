import { Routes } from "@angular/router";
import { AppLayout } from "./layouts/app-layout";
import { CurrentlyPlaying } from "./views/currently-playing/currently-playing";
import { Settings } from "./views/settings/settings";

export const routes: Routes = [
  {
    path: "",
    component: AppLayout,
    children: [
      { path: "", component: CurrentlyPlaying },
      {
        path: "settings",
        component: Settings,
      },
    ],
  },
];
