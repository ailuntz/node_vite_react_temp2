use tauri::{App, Manager};
use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial};
use raw_window_handle::HasWindowHandle;
use std::ffi::c_void;

pub fn init(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(target_os = "macos")]
    {
        let win = app.get_window("main").unwrap();
        let raw_handle = win.raw_window_handle();
        if let raw_window_handle::RawWindowHandle::AppKit(ns_view_handle) = raw_handle {
            let ns_view = ns_view_handle.ns_view as *mut c_void;
            apply_vibrancy(ns_view, NSVisualEffectMaterial::FullScreenUI, None, None)
                .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");
        }
    }

    Ok(())
}
