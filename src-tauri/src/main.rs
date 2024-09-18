// Prevents additional console window on Windows in release, DO NOT REMOVE!!
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// fn main() {
//   tauri::Builder::default()
//     .run(tauri::generate_context!())
//     .expect("error while running tauri application");
// }

mod setup;

fn main() {
    let context = tauri::generate_context!();
    tauri::Builder::default()
        .setup(setup::init)
        .run(context)
        .expect("error while running OhMyBox application");
}

