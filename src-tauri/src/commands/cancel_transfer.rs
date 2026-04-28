use std::{path::PathBuf, sync::Mutex};

use tauri::{AppHandle, Manager};

use crate::AppState;



#[tauri::command]
pub fn cancel_transfer_file(app: AppHandle, path: String) -> Result<(), String> {
    let path = PathBuf::from(path); 
    let state = app.state::<Mutex<AppState>>();
    let mut state = state.lock().map_err(|err| {
        eprintln!("error getting mutable state:{}", err);
        String::from("Error cancelling transfer: could not get access of app state")
    })?;
    match state.to_send.remove(&path) {
        None => Err(String::from("No such transfer found")),
        Some(_) => Ok(())
    }
}