use std::{sync::Mutex};

use mdns_sd::{ServiceDaemon, ServiceEvent};
use tauri::{AppHandle, Manager};

use crate::AppState;



pub async fn connect (app: AppHandle) -> Result<(), String> {
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
                        let mut state_handle = app.state::<Mutex<AppState>>();
                        if let Ok(state) = state_handle.lock() {
                            state.connected_with(*resolved);
                        };
                        
                    }
                    _ => {
                        println!("Other Receiver Service Event")
                    }
                }
            }
        }
    }
    
    Ok(())
}