// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// use tauri::Manager; // todo: for devtools
use base64::{prelude::BASE64_STANDARD, Engine};
// use hashes::md5::hash;

#[tauri::command]
fn encode_base64(str: &str) -> String {
    let x = BASE64_STANDARD.encode(str);
    String::from_utf8(x.into()).expect("todo")
}

#[tauri::command]
fn decode_base64(str: &str) -> Option<String> {
    let decoded = BASE64_STANDARD.decode(str);
    if let Ok(decoded) = decoded {
        Some(String::from_utf8(decoded).expect("todo"))
    } else {
        None
    }
}

// #[tauri::command]
// fn hash_md5(str: &str) -> String {
//     let digest = hash(str.as_bytes());
// }

fn main() {
    tauri::Builder::default()
        // todo: for devtools
        // .setup(|app| {
        //     #[cfg(debug_assertions)] // only include this code on debug builds
        //     {
        //         let window = app.get_window("main").unwrap();
        //         window.open_devtools();
        //         window.close_devtools();
        //     }
        //     Ok(())
        // })
        .invoke_handler(tauri::generate_handler![
            encode_base64,
            decode_base64,
            // hash_md5,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
