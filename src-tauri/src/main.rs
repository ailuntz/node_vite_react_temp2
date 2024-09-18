// Copyright 2019-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::{App, Manager};
use window_vibrancy::*;

fn main() {
  tauri::Builder::default()
      .setup(|app| {
        //   let window = app.get_webview_window("main").unwrap();
          use raw_window_handle::HasRawWindowHandle;
          let raw_handle = window.raw_window_handle();
          let window = app.get_window("main").unwrap();


          #[cfg(target_os = "macos")]
          apply_vibrancy(&raw_handle, NSVisualEffectMaterial::HudWindow, None, None)
              .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");

          #[cfg(target_os = "windows")]
          apply_blur(&raw_handle, Some((18, 18, 18, 125)))
              .expect("Unsupported platform! 'apply_blur' is only supported on Windows");

          Ok(())
      })
      .run(tauri::generate_context!())
      .expect("error while running tauri application");
}