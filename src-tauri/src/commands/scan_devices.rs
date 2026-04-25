use std::{
    sync::Mutex,
};

use tauri::Manager;

use crate::{
    commands::helpers::{find_device_receiver},
    AppState, Discovery,
};


#[tauri::command]
pub fn scan(
    app: tauri::AppHandle,
    discovery: Discovery,
) -> Result<(), String> {
    let state_handle = app.state::<Mutex<AppState>>();
    let mut state = match state_handle.lock() {
        Err(err) => {
            eprintln!(
                "error getting mutable state for changing discovery mode: {}",
                err
            );
            return Err(String::from(
                "Error getting mutable state for changing discovery mode",
            ));
        }
        Ok(state) => state,
    };
    state.discovery = discovery;
    match find_device_receiver::recv_search(&app) {
        Err(err) => {
            let err_state = String::from("couldn't connect with the sender");
            eprintln!("{:?}: {:?}", &err_state, &err);
            return Err(err_state);
        }
        Ok(_) => {
            println!("scanning stopped");
            Ok(())
        }
    }
}
