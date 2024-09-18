
  use tauri::Manager;
  use window_vibrancy::*;
  
  fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap();
  
            #[cfg(target_os = "macos")]
            apply_vibrancy(&window, NSVisualEffectMaterial::Sheet, Some(NSVisualEffectState::Active), Some(10.0))
                .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");
  
            #[cfg(target_os = "windows")]
            apply_blur(&window, Some((18, 18, 18, 125)))
                .expect("Unsupported platform! 'apply_blur' is only supported on Windows");
  
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
  }
  