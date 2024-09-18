use tauri::Window;
use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial};
use raw_window_handle::HasRawWindowHandle;
use std::ffi::c_void;

pub fn init(win: &Window) -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(target_os = "macos")]
    {
        // 获取原始窗口句柄
        let raw_handle = win.raw_window_handle().ns_view() as *mut c_void;
        apply_vibrancy(raw_handle, NSVisualEffectMaterial::FullScreenUI, None, None)
            .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");
    }

    #[cfg(target_os = "windows")]
    {
        // Windows 平台的处理方式
        // ...
    }

    Ok(())
}
