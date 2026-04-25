use std::{collections::HashMap, io::{Read, Write}, net::{SocketAddr, TcpStream}, sync::{Arc, Mutex}};

use quinn::{ClientConfig, Endpoint};
use rustls::pki_types::CertificateDer;
use tauri::{AppHandle, Manager};

use crate::AppState;


#[tauri::command]
pub async fn receive_cert_and_connect_quic(app: AppHandle, txt_properties: HashMap<String, String>, otp: String) -> Result<(), String> {
    let properties: HashMap<String, String> = txt_properties;
    let tcp_listner = match properties.get("tcp_listner") {
        None => return Err(String::from("could not get tcp listner")),
        Some(addr) => addr
    };
    let quinn_addr = match properties.get("quinn_addr") {
        None => return Err(String::from("Could not get Quinn Address")),
        Some(addr) => addr
    };
    
    let quinn_addr: SocketAddr = match quinn_addr.parse() {
        Err(err) => {
            eprintln!("error parsing quinn address: {}", err);
            return Err(String::from("error parsing connection socket address"));
        }
        Ok(addr) => addr
    };
    
    let mut stream = match TcpStream::connect(tcp_listner) {
        Err(err) => {
            eprintln!("error connecting to tcp: {}", err);
            return Err(String::from("error connecting to tcp"))
        }
        Ok(stream) => stream
    };
    stream.write_all(otp.as_bytes()).expect("failed to send otp");
    
    let mut cert_bytes = Vec::new();
    match stream.read_to_end(&mut cert_bytes){
        Err(err) => {
            eprintln!("error getting cert through tcp: {}", err);
            return Err(String::from("error getting cert from sender"));
        }
        Ok(_) => ()
    };
    let mut cert = rustls::RootCertStore::empty();
    
    match cert.add(CertificateDer::from(cert_bytes)) {
        Err(err) => {
            eprintln!("error adding cert to cert store: {}", err);
            return Err(String::from("error adding cert to cert store"));
        }
        Ok(_) => ()
    };
    
    let client_config = match ClientConfig::with_root_certificates(Arc::new(cert)) {
        Err(err) => {
            eprintln!("error making client config: {}", err);
            return Err(String::from("error making client config"));
        }
        Ok(conf) => conf
    };
    
    let mut endpoint = match Endpoint::client("0.0.0.0:0".parse().unwrap()) {
        Err(err) => {
            eprintln!("error making client quinn endpoint: {}", err);
            return Err(String::from("Error making client for quinn conn"));
        }
        Ok(point) => point
    };
    endpoint.set_default_client_config(client_config);
    match endpoint.connect(quinn_addr, &String::from("velocity")) {
        Err(err) => {
            eprintln!("error connecting with sender with quinn: {}", err);
            return Err(String::from("error connecting with sender"));
        } 
        Ok(conn) => match conn.await {
            Err(err) => {
                eprintln!("error establishing stable quinn connection with the sender: {}", err);
                return Err(String::from("Error establishing stable connection with the sender"));
            }
            Ok(con) => {
                let state = app.state::<Mutex<AppState>>();
                let mut state = match state.lock() {
                    Err(err) => {
                        eprintln!("error getting mutable state while establishing connection with sender: {}", err);
                        return Err(String::from("Error getting mutable state while establishing connection with sender"));
                    }
                    Ok(state) => state
                };
                state.connected_to = Some(con);
                Ok(())
            }
        }
    }
}