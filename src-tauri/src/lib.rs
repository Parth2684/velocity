// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use std::{collections::HashMap, ffi::OsString, sync::{Mutex, mpsc}};

use gethostname::gethostname;
use mdns_sd::ResolvedService;
use serde::{self, Deserialize};
use tauri::Manager;

mod commands;

use commands::connect::connect;

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
enum Discovery {
    On,
    Off,
}

struct AppState {
    device_name: OsString,
    connected_with: Option<ResolvedService>,
    available_devices: HashMap<String, ResolvedService>,
    // discovery: Option<mpsc>
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let device_name = gethostname();
            app.manage(Mutex::new(AppState {
                device_name,
                connected_with: None,
                available_devices: HashMap::new(),
                // discovery: None
            }));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![connect])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
