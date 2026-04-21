use std::{sync::Mutex};

use mdns_sd::{ServiceDaemon, ServiceEvent};
use tauri::Manager;

use crate::{AppState, Discovery};



pub fn recv_connect (app: &tauri::AppHandle) -> Result<(), String> {
    let state = app.state::<Mutex<AppState>>();
    let mut state = match state.lock() {
        Err(err) => {
            eprintln!("error getting mutable state: {:?}", err);
            return Err(String::from("error getting mutable state"));
        }
        Ok(state) => state
    };
    
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
            loop {
                let should_continue = {
                    match state.discovery {
                        Discovery::Off => false,
                        Discovery::On => true
                    }
                };
                if !should_continue {
                    println!("stopped searching");
                    std::thread::sleep(std::time::Duration::from_secs(1));
                    mdns.shutdown().ok();
                    break;
                }
                if let Ok(event) = recv.recv() {                
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
            }
            
            Ok(())
        }
    }
}