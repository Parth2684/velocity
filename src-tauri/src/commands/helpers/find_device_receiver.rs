use std::sync::Mutex;

use mdns_sd::ServiceEvent;
use tauri::{Emitter, Manager};

use crate::{AppState, AvailableDevice, Discovery};

pub fn recv_search(app: &tauri::AppHandle) -> Result<(), String> {
    let state = app.state::<Mutex<AppState>>();
    let mut state = match state.lock() {
        Err(err) => {
            eprintln!("error getting mutable state: {:?}", err);
            return Err(String::from("error getting mutable state"));
        }
        Ok(state) => state,
    };

    let receiver = state.mdns.browse("_velocity._udp.local.");
    match receiver {
        Err(err) => return Err(err.to_string()),
        Ok(recv) => {
            loop {
                let should_continue = {
                    match state.discovery {
                        Discovery::Off => false,
                        Discovery::On => true,
                    }
                };
                if !should_continue {
                    println!("stopped searching");
                    std::thread::sleep(std::time::Duration::from_secs(1));
                    state.mdns.shutdown().ok();
                    break;
                }
                if let Ok(event) = recv.recv() {
                    match event {
                        ServiceEvent::ServiceResolved(resolved) => {
                            let resolved_service = AvailableDevice {
                                fullname: resolved.fullname.clone(),
                                ty_domain: resolved.ty_domain,
                                sub_ty_domain: resolved.sub_ty_domain,
                                host: resolved.host,
                                port: resolved.port,
                                txt_properties: resolved
                                    .txt_properties
                                    .iter()
                                    .map(|prop| (prop.key().to_string(), prop.val_str().to_owned()))
                                    .collect(),
                            };
                            state
                                .available_devices
                                .insert(resolved.fullname, resolved_service.clone());
                            app.emit("add_available_device", resolved_service).ok();
                        }
                        ServiceEvent::ServiceRemoved(_, full_name) => {
                            state.available_devices.remove(&full_name);
                            app.emit("remove_available_device", full_name).ok();
                        }
                        _ => {
                            println!("other event: {:?}", event);
                        }
                    };
                }
            }

            Ok(())
        }
    }
}
