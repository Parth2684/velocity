use std::{net::SocketAddr, sync::{Arc, Mutex}};

use quinn::{Endpoint, ServerConfig};
use tauri::{AppHandle, Manager};

use crate::AppState;

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
    transport_config.max_concurrent_bidi_streams(100_u32.into());
    let ip = state.socket_addr.ip();
    let socket_addr = SocketAddr::new(ip, 0000);
    let endpoint = match Endpoint::server(server_config, socket_addr) {
        Err(err) => {
            eprintln!("error making endpoint for quic: {}", err);
            return Err(String::from("error making endpoint"));
        }
        Ok(endpont) => endpont
    };
    
    let app = app.clone();
    
    tokio::spawn(async move {
        if let Some(incoming_conn) = endpoint.accept().await {
            match incoming_conn.await {
                Err(err) => {
                    eprintln!("error accepting connection: {}", err);
                    return Err(String::from("error accepting connection"));
                }
                Ok(con) => {
                    let state = app.state::<Mutex<AppState>>();
                    let mut state = state.lock().expect("Error connecting to device");
                    state.connected_to = Some(con);
                }
            };
        }
        Ok(())
    });
    Ok(())
}
