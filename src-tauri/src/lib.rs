// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg(target_os = "macos")]
use cocoa::appkit::{NSColor, NSWindow};
#[cfg(target_os = "macos")]
use cocoa::base::{id, nil};

use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(target_os = "macos")]
            {
                let window = app.get_webview_window("main").unwrap();
                let ns_window = window.ns_window().unwrap() as id;

                unsafe {
                    ns_window.setOpaque_(false);
                    ns_window.setBackgroundColor_(NSColor::clearColor(nil));
                }
            }

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}