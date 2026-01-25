import { Menu, MenuItem, Submenu } from "@tauri-apps/api/menu";
import { exit } from "@tauri-apps/plugin-process";
import { launchSettingsWindow } from "../services/windows";

export async function getMenu() {
  // Will become the application submenu on MacOS
  const aboutSubmenu = await Submenu.new({
    text: "About",
    items: [
      await MenuItem.new({
        id: "settings",
        text: "Settings",
        action: () => {
          void launchSettingsWindow().catch((err) => console.error(err));
        },
      }),
      // Can make these `IconMenuItem`s later to add icons
      await MenuItem.new({
        id: "quit",
        text: "Quit",
        action: () => {
          void exit(0).catch((err) => console.error(err));
        },
      }),
    ],
  });

  const menu = await Menu.new({
    items: [aboutSubmenu],
  });

  return menu;
}
