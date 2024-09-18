use tauri::{App, Manager};
use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial};
use std::ffi::c_void;

pub fn init(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(target_os = "macos")]
    {
        let win = app.get_window("main").unwrap();
        // 获取 ns_view 作为原始窗口句柄
        let raw_handle = win.ns_view() as *mut c_void;

        apply_vibrancy(raw_handle, NSVisualEffectMaterial::FullScreenUI, None, None)
            .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");
    }

    Ok(())
}
