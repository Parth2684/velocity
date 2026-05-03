use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
    thread,
};

use quinn::{Endpoint, ServerConfig};
use tauri::{AppHandle, Emitter, Manager};
use tokio::io::AsyncReadExt;

use crate::{commands::helpers::find_device_sender::send_publish, AppState, Discovery};

#[tauri::command]
pub fn serve_and_connect_quic(app: AppHandle) -> Result<(), String> {
    let state = app.state::<Mutex<AppState>>();
    let state = match state.lock() {
        Err(err) => {
            eprintln!("error getting state: {}", err);
            return Err(String::from("error getting application state"));
        }
        Ok(state) => state,
    };

    let mut server_config = match ServerConfig::with_single_cert(
        vec![state.certificate.clone()],
        state.key.clone_key(),
    ) {
        Err(err) => {
            eprintln!("error loading server config certificate: {}", err);
            return Err(String::from("error loading server config certificate"));
        }
        Ok(conf) => conf,
    };

    let transport_config = match Arc::get_mut(&mut server_config.transport) {
        None => {
            let error_message = "could not mutate transport service";
            eprintln!("{}", error_message);
            return Err(String::from(error_message));
        }
        Some(config) => config,
    };

    transport_config.max_concurrent_uni_streams(100_u32.into());
    // transport_config.max_concurrent_bidi_streams(100_u32.into());
    let ip = state.socket_addr.ip();
    let socket_addr = SocketAddr::new(ip, 0);
    let endpoint = match Endpoint::server(server_config, socket_addr) {
        Err(err) => {
            eprintln!("error making endpoint for quic: {}", err);
            return Err(String::from("error making endpoint"));
        }
        Ok(endpont) => endpont,
    };

    let app = app.clone();
    let app2 = app.clone();

    tokio::spawn(async move {
        while let Some(incoming_conn) = endpoint.accept().await {
            let app = app.clone();
    
            tokio::spawn(async move {
                match incoming_conn.await {
                    Err(err) => {
                        eprintln!("error accepting connection: {}", err);
                    }
                    Ok(con) => {
                        let device_name = {
                            let state = app.state::<Mutex<AppState>>();
                            let mut state = state.lock().expect("error getting state");
                            state.connected_to = Some(con.clone());
                            state.device_name.clone()
                        };
                        let (mut send_stream, mut recv_stream) = con.open_bi().await.expect("error opening bi con to share device info to receiver");
                        
                        send_stream.write_all(device_name.as_encoded_bytes()).await.expect("error sending device name to receiver");
                        send_stream.finish().ok();
                        
                        let mut receiver_name = String::new();
                        if let Err(err) = recv_stream.read_to_string(&mut receiver_name).await {
                            eprintln!("error receiving receriver's name: {}", err);
                        }
                        con.closed().await;
                        
                        app.emit("connection_success", &receiver_name).ok();
                    }
                }
            });
        }
    });

    thread::spawn(move || {
        let app2 = app2;
        send_publish(&app2, Discovery::On, socket_addr)
    });

    Ok(())
}
