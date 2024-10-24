// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod client;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![client::get_label])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
