#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//! Tauri 应用入口 - 简化版

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}