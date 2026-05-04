use std::{
    collections::HashMap,
    io::{Read, Write},
    net::{SocketAddr, TcpStream},
    sync::{Arc, Mutex},
};

use quinn::{ClientConfig, Endpoint};
use rustls::pki_types::CertificateDer;
use tauri::{AppHandle, Emitter, Manager};
use tokio::io::AsyncReadExt;

use crate::AppState;

#[tauri::command]
pub async fn receive_cert_and_connect_quic(
    app: AppHandle,
    txt_properties: HashMap<String, String>,
    otp: String,
) -> Result<(), String> {
    let properties: HashMap<String, String> = txt_properties;
    let tcp_listner = match properties.get("tcp_listner") {
        None => return Err(String::from("could not get tcp listner")),
        Some(addr) => addr,
    };
    let quinn_addr = match properties.get("quinn_addr") {
        None => return Err(String::from("Could not get Quinn Address")),
        Some(addr) => addr,
    };

    let quinn_addr: SocketAddr = match quinn_addr.parse() {
        Err(err) => {
            eprintln!("error parsing quinn address: {}", err);
            return Err(String::from("error parsing connection socket address"));
        }
        Ok(addr) => addr,
    };

    let mut stream = match TcpStream::connect(tcp_listner) {
        Err(err) => {
            eprintln!("error connecting to tcp: {}", err);
            return Err(String::from("error connecting to tcp"));
        }
        Ok(stream) => stream,
    };
    stream
        .write_all(otp.as_bytes())
        .expect("failed to send otp");

    let mut header = [0u8; 3];

    let n = match stream.read(&mut header) {
        Err(err) => {
            eprintln!("error reading response: {}", err);
            return Err(String::from("error reading response"));
        }
        Ok(n) => n,
    };

    let header_str = String::from_utf8_lossy(&header[..n]);

    if header_str == "ERR" {
        return Err(String::from("Invalid OTP"));
    }

    let mut cert_bytes = Vec::new();
    match stream.read_to_end(&mut cert_bytes) {
        Err(err) => {
            eprintln!("error getting cert through tcp: {}", err);
            return Err(String::from("error getting cert from sender"));
        }
        Ok(_) => (),
    };

    let mut cert = rustls::RootCertStore::empty();

    match cert.add(CertificateDer::from(cert_bytes)) {
        Err(err) => {
            eprintln!("error adding cert to cert store: {}", err);
            return Err(String::from("error adding cert to cert store"));
        }
        Ok(_) => (),
    };

    let client_config = match ClientConfig::with_root_certificates(Arc::new(cert)) {
        Err(err) => {
            eprintln!("error making client config: {}", err);
            return Err(String::from("error making client config"));
        }
        Ok(conf) => conf,
    };

    let mut endpoint = match Endpoint::client("0.0.0.0:0".parse().unwrap()) {
        Err(err) => {
            eprintln!("error making client quinn endpoint: {}", err);
            return Err(String::from("Error making client for quinn conn"));
        }
        Ok(point) => point,
    };
    endpoint.set_default_client_config(client_config);
    match endpoint.connect(quinn_addr, &String::from("velocity")) {
        Err(err) => {
            eprintln!("error connecting with sender with quinn: {}", err);
            return Err(String::from("error connecting with sender"));
        }
        Ok(conn) => match conn.await {
            Err(err) => {
                eprintln!(
                    "error establishing stable quinn connection with the sender: {}",
                    err
                );
                return Err(String::from(
                    "Error establishing stable connection with the sender",
                ));
            }
            Ok(con) => {
                let state = app.state::<Mutex<AppState>>();
                let device_name = {
                    let mut state = match state.lock() {
                        Err(err) => {
                            eprintln!("error getting mutable state while establishing connection with sender: {}", err);
                            return Err(String::from("Error getting mutable state while establishing connection with sender"));
                        }
                        Ok(state) => state,
                    };
                    state.connected_to = Some(con.clone());
                    state.device_name.clone()
                };
                let (mut send, mut recv) =
                    con.accept_bi().await.expect("error accepting bi stream");
                let mut sender_name = String::new();
                recv.read_to_string(&mut sender_name)
                    .await
                    .expect("error getting sender's name");
                send.write_all(device_name.as_encoded_bytes())
                    .await
                    .expect("error sending your name to sender");
                send.finish().ok();
                con.close(0u8.into(), b"done");
                app.emit("connection_success", &sender_name).ok();
                Ok(())
            }
        },
    }
}
