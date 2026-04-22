use std::collections::HashMap;
use std::{net::UdpSocket, sync::Mutex};

use mdns_sd::{ServiceInfo};
use tauri::{AppHandle, Manager};

use crate::{AppState, Discovery};


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
            let host_name = format!("{}.local.", &device_name);
            let socket = match UdpSocket::bind("0.0.0.0") {
                Err(err) => {
                    eprintln!("error getting socket: {}", err);
                    return Err(String::from("error getting local ip"));
                }
                Ok(socket) => socket
            };
            match socket.connect("8.8.8.8:80") {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Failed to determine IP: {}", e);
                    return Err("Please turn on the internet: Failed to determine ip".into());
                }
            }
            let socket = match socket.local_addr() {
                Err(err) => {
                    eprintln!("error getting device ip: {}", err);
                    return Err(String::from("error getting device ip"));
                }
                Ok(local) => local
            };

            let ip = socket.ip();
            let port = socket.port();

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
