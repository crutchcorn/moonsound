import { Component, effect } from "@angular/core";
import { LightGlassOutlined } from "../../components/light-glass-outlined/light-glass-outlined";
import { setBodyBg } from "../../utils/styling";
import {injectMutation} from "@tanstack/angular-query-experimental";
import { addFolder } from "../../services/fs";

@Component({
  selector: "settings-view",
  templateUrl: "./settings.html",
  styleUrl: "./settings.scss",
  imports: [LightGlassOutlined],
})
export class Settings {
  _ = effect(() => {
    setBodyBg("#060606");
  });
  addFolderMutation = injectMutation(() => ({
    mutationKey: ["openFile"],
    mutationFn: async () => {
      const path = await addFolder();
      if (!path) {
        return;
      }
      return true;
    },
  }));
}
