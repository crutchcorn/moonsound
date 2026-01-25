import {
  Menu,
  MenuItem,
  PredefinedMenuItem,
  Submenu,
} from "@tauri-apps/api/menu";
import { launchSettingsWindow } from "../services/windows";

export async function getMenu() {
  // Will become the application submenu on MacOS
  const aboutSubmenu = await Submenu.new({
    text: "About",
    items: [
      await PredefinedMenuItem.new({
        item: {
          About: {},
        },
      }),
      await MenuItem.new({
        id: "settings",
        text: "Settings",
        action: () => {
          void launchSettingsWindow().catch((err) => console.error(err));
        },
      }),
      await PredefinedMenuItem.new({
        item: "CloseWindow",
      }),
      await PredefinedMenuItem.new({
        item: "Quit",
      }),
    ],
  });

  const menu = await Menu.new({
    items: [aboutSubmenu],
  });

  return menu;
}
