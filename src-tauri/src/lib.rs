// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use std::{
    collections::{HashMap},
    ffi::OsString,
    fs,
    net::{SocketAddr, UdpSocket},
    sync::Mutex,
};

use gethostname::gethostname;
use mdns_sd::{ServiceDaemon};
use quinn::Connection;
use rcgen::generate_simple_self_signed;
use rustls::pki_types::{CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer};
use serde::{self, Deserialize, Serialize};
use tauri::Manager;

mod commands;

use commands::{
    connect_client::receive_cert_and_connect_quic, connect_server::serve_and_connect_quic,
    scan_devices::scan,
};

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
enum Discovery {
    On,
    Off,
}

#[derive(Serialize, Deserialize, Clone)]
struct AvailableDevice {
    ty_domain: String,
    sub_ty_domain: Option<String>,
    fullname: String,
    host: String,
    port: u16,
    txt_properties: HashMap<String, String>,
}

struct AppState {
    device_name: OsString,
    available_devices: HashMap<String, AvailableDevice>,
    discovery: Discovery,
    mdns: ServiceDaemon,
    socket_addr: SocketAddr,
    certificate: CertificateDer<'static>,
    key: PrivateKeyDer<'static>,
    connected_to: Option<Connection>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let device_name = gethostname();
            let mdns = ServiceDaemon::new().expect("error getting mdns daemon");
            let socket = match UdpSocket::bind("0.0.0.0") {
                Err(err) => {
                    eprintln!("error getting socket: {}", err);
                    panic!("error getting local ip");
                }
                Ok(socket) => socket,
            };
            match socket.connect("8.8.8.8:80") {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Failed to determine IP: {}", e);
                    // panic!("Please connect to internet: Failed to determine ip");
                }
            };
            let socket = match socket.local_addr() {
                Err(err) => {
                    eprintln!("error getting device ip: {}", err);
                    panic!("error getting device ip");
                }
                Ok(local) => local,
            };
            let local_data_dir = app
                .path()
                .local_data_dir()
                .expect("local data dir not created");
            let cert_path = local_data_dir.join("cert.der");
            let key_path = local_data_dir.join("key.der");

            if !cert_path.exists() || !key_path.exists() {
                let cert = generate_simple_self_signed(vec![String::from("velocity")])
                    .expect("could not create certificate");
                let cert_der = CertificateDer::from(cert.cert);
                let key_der = PrivateKeyDer::from(cert.signing_key);
                fs::write(&cert_path, cert_der).expect("could not store certificate");
                fs::write(&key_path, key_der.secret_der()).expect("could not store key");
            }

            let certificate =
                CertificateDer::from(fs::read(cert_path).expect("could not read certificate"));
            let key = PrivateKeyDer::from(PrivatePkcs8KeyDer::from(
                fs::read(key_path).expect("could not read key"),
            ));

            app.manage(Mutex::new(AppState {
                device_name,
                available_devices: HashMap::new(),
                discovery: Discovery::Off,
                mdns,
                socket_addr: socket,
                certificate,
                key,
                connected_to: None,
            }));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            scan,
            serve_and_connect_quic,
            receive_cert_and_connect_quic
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
