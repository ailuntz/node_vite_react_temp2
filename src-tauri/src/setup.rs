use tauri::{App, Manager};
use window_vibrancy::{apply_blur, apply_vibrancy, NSVisualEffectMaterial};

/// setup
pub fn init(app: &mut App) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let win = app.get_window("main").unwrap();

    // 仅在 macOS 下执行
    #[cfg(target_os = "macos")]
    unsafe {
        // 使用 ns_window() 获取 macOS 上的原始 NSWindow 对象
        let raw_handle = win.ns_window().unwrap() as *mut std::ffi::c_void;
        apply_vibrancy(raw_handle, NSVisualEffectMaterial::FullScreenUI, None, None)
            .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");
    }

    // 仅在 Windows 下执行
    #[cfg(target_os = "windows")]
    apply_blur(&win, Some((18, 18, 18, 125)))
        .expect("Unsupported platform! 'apply_blur' is only supported on Windows");

    Ok(())
}
