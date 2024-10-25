// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::AppHandle;
mod client;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![get_app_version])
    .invoke_handler(tauri::generate_handler![client::get_label])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

// Command to get the version from package info
#[tauri::command]
fn get_app_version(app_handle: AppHandle) -> Result<String, String> {
    let version = app_handle.package_info().version.to_string();
    Ok(version)
}
