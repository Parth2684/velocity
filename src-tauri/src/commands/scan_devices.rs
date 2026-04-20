use std::sync::Mutex;

use serde::{Deserialize};
use tauri::Manager;

use crate::{AppState, Discovery, commands::helpers::find_device_receiver};

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum UserType {
    Sender,
    Receiver,
}


#[tauri::command]
pub fn scan(app: tauri::AppHandle, user_type: UserType) -> Result<(), String> {
    match user_type {
        UserType::Sender => todo!(),
        UserType::Receiver => {
            let state_handle = app.state::<Mutex<AppState>>();
            let mut state = match state_handle.lock() {
                Ok(state) => state,
                Err(err) => {
                    eprintln!("error updating state pf connection: {:?}", err);
                    return Err(String::from("error updating connection state"));
                }
            };
            
            state.discovery = Discovery::On;
            match find_device_receiver::recv_connect(&app) {
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
        },
    }
}
