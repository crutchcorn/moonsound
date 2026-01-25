import { Routes } from "@angular/router";
import { CurrentlyPlaying } from "./views/currently-playing.component";
import { Settings } from "./views/settings.component";
import { AppLayout } from "./layouts/app-layout";

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
