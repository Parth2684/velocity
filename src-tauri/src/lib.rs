// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use std::{ffi::OsString, sync::Mutex};

use gethostname::gethostname;
use mdns_sd::ResolvedService;
use tauri::Manager;

mod commands;

struct AppState {
    device_name: OsString,
    connected_with: Option<ResolvedService>
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app|{
            let device_name = gethostname();
            app.manage(Mutex::new(AppState{ 
                device_name,
                connected_with: None
            }));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
