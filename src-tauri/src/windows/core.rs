use tauri::{WebviewWindow};
use tauri_plugin_decorum::WebviewWindowExt;
use window_vibrancy::{apply_vibrancy, apply_mica, NSVisualEffectMaterial};

pub fn make_window_effects(window: WebviewWindow) {
    window.create_overlay_titlebar().unwrap();
    if cfg!(target_os = "macos") {
        #[cfg(target_os = "macos")]
        window.make_transparent().unwrap();

        #[cfg(target_os = "macos")]
        apply_vibrancy(&window, NSVisualEffectMaterial::HudWindow, None, None)
            .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");
    } else if cfg!(target_os = "windows") {
        #[cfg(target_os = "windows")]
        apply_mica(
            &window,
            if window.theme().unwrap() == Theme::Dark {
                Some(true)
            } else {
                None
            },
        )
            .expect("Unsupported platform! 'apply_mica' is only supported on Windows");
    }

}
