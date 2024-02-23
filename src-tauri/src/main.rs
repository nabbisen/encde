use base64::{prelude::{BASE64_STANDARD}, Engine};

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#[cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn encode_base64(str: &str) -> String {
    let x = BASE64_STANDARD.encode(str);
    String::from_utf8(x.into()).expect("todo")
}

#[tauri::command]
fn decode_base64(str: &str) -> String {
    let x = BASE64_STANDARD.decode(str);
    String::from_utf8(x.expect("todo")).expect("todo")
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![encode_base64, decode_base64])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
