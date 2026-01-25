import { Component, effect } from "@angular/core";
import { LightGlassOutlined } from "../../components/light-glass-outlined/light-glass-outlined";
import { setBodyBg } from "../../utils/styling";

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
}
