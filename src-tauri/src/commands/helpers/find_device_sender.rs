use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener};
use std::thread;
use std::{sync::Mutex};

use mdns_sd::{ServiceInfo};
use tauri::{AppHandle, Emitter, Manager};
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

pub fn send_publish(app: &AppHandle, discovery: Discovery, quinn_addr: SocketAddr) -> Result<(), String> {
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
            let tcp_ip = state.socket_addr.ip();
            let port = state.socket_addr.port();
            
            let cert = state.certificate.clone();
            
            let addr = SocketAddr::new(tcp_ip, 0000);
            let otp = Uuid::new_v4().to_string()[0..4].to_string();
            // let otp_clone = otp.clone();
            match app.emit("connect_otp", otp.clone()) {
                Ok(_) => (),
                Err(err) => {
                    eprintln!("error showing otp to user: {}", err);
                    panic!("error showing otp to you");
                }
            };
            let handle = thread::spawn(move || {
                let listner = TcpListener::bind(addr).expect("listner failed: could not share certificate");
                for stream in listner.incoming() {
                    let mut stream = stream.expect("error sending certificate");
                    let mut buffer = [0u8; 16];
                    let n = match stream.read(&mut buffer) {
                        Err(err) => {
                            eprintln!("error getting otp: {}", err);
                            continue;
                        }
                        Ok(size) => size
                    };
                    let incoming_otp = String::from_utf8_lossy(&buffer[..n]).trim().to_string();
                    if incoming_otp == otp {
                        stream.write_all(cert.as_ref()).expect("could not send certificate");
                        return;
                    }else {
                        continue;
                    }
                }
            });

            let mut properties = HashMap::new();
            properties.insert(String::from("tcp_listner"), addr.to_string());
            properties.insert(String::from("quinn_addr"), quinn_addr.to_string());
            
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
            handle.join().expect("error starting certificate sharing server");
        }
        Discovery::Off => {
            state.discovery = Discovery::Off;
            std::thread::sleep(std::time::Duration::from_secs(1));
            state.mdns.shutdown().unwrap();
        }
    }

    Ok(())
}
