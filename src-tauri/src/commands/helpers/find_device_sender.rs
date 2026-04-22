use std::collections::HashMap;
use std::{sync::Mutex};

use mdns_sd::{ServiceInfo};
use tauri::{AppHandle, Manager};
use uuid::Uuid;

use crate::{AppState, Discovery};

fn to_host_name(name: &str) -> String {
    let cleaned: String = name
        .to_lowercase()
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() {
                c
            } else {
                '-' // replace everything else
            }
        })
        .collect();
    let uuid = &Uuid::new_v4().to_string()[..4];

    format!("{}-{}.local.", cleaned.trim_matches('-'), uuid)
}

pub fn send_publish(app: &AppHandle, discovery: Discovery) -> Result<(), String> {
    let state = app.state::<Mutex<AppState>>();
    let mut state = match state.lock() {
        Err(err) => {
            eprintln!("error getting mutable state, {}", err);
            return Err(String::from("error getting mutable state"));
        }
        Ok(state) => state
    };

    match discovery {
        Discovery::On => {
            let device_name = match state.device_name.clone().into_string() {
                Err(err) => {
                    eprintln!("error getting string from: {:?} ", err);
                    return Err(String::from("Error getting device name"));
                }
                Ok(str) => str
            };
            let host_name = to_host_name(&device_name);
           
            let ip = state.socket_addr.ip();
            let port = state.socket_addr.port();

            let mut properties = HashMap::new();
            properties.insert("version".to_string(), "1.0".to_string());
            properties.insert("service".to_string(), "file-transfer".to_string());
            
            state.discovery = Discovery::On;

            let my_service = match ServiceInfo::new("_velocity._udp.local.", &device_name, &host_name, ip, port, properties) {
                Err(err) => {
                    eprintln!("error making service: {}", err);
                    return Err(String::from("error making serice"));
                }
                Ok(service) => service
            };
            dbg!(&my_service);
            state.mdns.register(my_service).unwrap();
        }
        Discovery::Off => {
            state.discovery = Discovery::Off;
            std::thread::sleep(std::time::Duration::from_secs(1));
            state.mdns.shutdown().unwrap();
        }
    }

    Ok(())
}
