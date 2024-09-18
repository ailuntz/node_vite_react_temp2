use tauri::{App, Manager};
use window_vibrancy::{apply_blur, apply_vibrancy, NSVisualEffectMaterial};

/// setup
pub fn init(app: &mut App) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let win = app.get_window("main").unwrap();

    // 仅在 macOS 下执行
    #[cfg(target_os = "macos")]
    {
        // 使用 ns_view 获取原始窗口句柄
        let raw_handle = win.ns_view() as *mut std::ffi::c_void;
        apply_vibrancy(raw_handle, NSVisualEffectMaterial::FullScreenUI, None, None)
            .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");
    }

    // 仅在 Windows 下执行
    #[cfg(target_os = "windows")]
    {
        let raw_handle = win.hwnd() as *mut std::ffi::c_void;
        apply_blur(&raw_handle, Some((18, 18, 18, 125)))
            .expect("Unsupported platform! 'apply_blur' is only supported on Windows");
    }

    Ok(())
}
