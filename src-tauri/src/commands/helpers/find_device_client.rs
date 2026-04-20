use std::{collections::HashMap, sync::Mutex};

use mdns_sd::{ResolvedService, ServiceDaemon, ServiceEvent};
use serde::Deserialize;
use tauri::Manager;
use std::sync::mpsc;

use crate::AppState;


#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
enum Discovery {
    On,
    Off,
}



pub fn recv_connect (app: &tauri::AppHandle) -> Result<(), String> {
    let state = app.state::<Mutex<AppState>>();
    let mut state = match state.lock() {
        Err(err) => {
            eprintln!("error getting mutable state: {:?}", err);
            return Err(String::from("error getting mutable state"));
        }
        Ok(state) => state
    };
    
    // let (tx, mut rx) = mpsc::channel::<Discovery>();
    let mdns = match ServiceDaemon::new() {
        Err(err) => return Err(err.to_string()),
        Ok(service) => {
            service
        }
    };
    
    let receiver = mdns.browse("_velocity._udp.local.");
    match receiver {
        Err(err) => return Err(err.to_string()),
        Ok(recv) => {
            while let Ok(event) = recv.recv() {                
                match event {
                    ServiceEvent::ServiceResolved(resolved) => {
                        state.available_devices.insert(resolved.fullname.clone(), *resolved);
                    }
                    ServiceEvent::ServiceRemoved(_, full_name) => {
                        state.available_devices.remove(&full_name);
                    }
                    _=> {
                        println!("other event: {:?}", event);
                    }
                };
            }
            Ok(())
        }
    }
}