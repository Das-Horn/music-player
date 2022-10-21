#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// KEEP Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

use music_player::Lib;
use std::env;

#[tauri::command]
fn get_os_dir() -> String {
    let ret: String;
    match dirs2::audio_dir() {
        Some(path) => ret = path.into_os_string().into_string().unwrap(),
        None => ret = String::from("Unsupported OS"),
    }
    return ret;
}

#[tauri::command]
fn scan_dir(path: String) -> String {
    Lib::find_music(path)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_os_dir, scan_dir])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
