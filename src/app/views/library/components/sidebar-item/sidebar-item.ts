import { Component, inject, input } from "@angular/core";
import { Metadata } from "../../../../injectables/metadata";
import { LightGlassOutlined } from "../../../../components/light-glass-outlined/light-glass-outlined";

@Component({
  selector: "sidebar-item",
  templateUrl: "./sidebar-item.html",
  styleUrl: "./sidebar-item.scss",
  imports: [LightGlassOutlined],
})
export class SidebarItem {
  stateMetadata = inject(Metadata);

  name = input.required<string>();
  iconPath = input.required<string>();
}
