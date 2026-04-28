use std::{collections::{HashMap, HashSet}, fs::File, io::Read, path::PathBuf, sync::Mutex};

use bincode::{config};
use tauri::{AppHandle, Emitter, Manager};
use infer::{MatcherType};

use crate::{AppState, CustomMatcherType, Metadata};



fn matcher_to_string(matcher_type: MatcherType) -> CustomMatcherType {
    match matcher_type {
        MatcherType::App => CustomMatcherType::App,
        MatcherType::Archive => CustomMatcherType::Archive,
        MatcherType::Audio => CustomMatcherType::Audio,
        MatcherType::Book => CustomMatcherType::Book,
        MatcherType::Custom => CustomMatcherType::Custom,
        MatcherType::Doc => CustomMatcherType::Doc,
        MatcherType::Font => CustomMatcherType::Font,
        MatcherType::Image => CustomMatcherType::Image,
        MatcherType::Text => CustomMatcherType::Text,
        MatcherType::Video => CustomMatcherType::Video
    }
}

const BUFFER_SIZE: usize = 1024 * 1024 * 50;


#[tauri::command]
pub async fn send_file(app: AppHandle, paths: HashSet<String>) -> Result<(), String> {
    let paths_exists: HashSet<PathBuf> = paths.into_iter().filter_map(|map|{
        let path = PathBuf::from(map);
        if !path.exists() {
            None
        }else {
            Some(path)
        }
    }).collect();


    let metadata: HashMap<PathBuf, Metadata> = paths_exists.iter().filter_map(|path| {
        let file = match File::open(path) {
            Err(err) => {
                eprintln!("could not access file: {}", err);
                return None;
            }
            Ok(file) => file
        };
        let file_size: u64 = match file.metadata() {
            Err(err) => {
                eprintln!("error getting metadata of the file: {}", err);
                return None;
            }
            Ok(data) => {
                data.len()
            }
        };
        let matcher_type = match infer::get_from_path(&path) {
            Err(err) => {
                eprintln!("could not get file type, {}", err);
                matcher_to_string(MatcherType::Custom)
            }
            Ok(some_type) => match some_type {
                None => matcher_to_string(MatcherType::Custom),
                Some(matcher_type) => matcher_to_string(matcher_type.matcher_type())
            }
        };
        let name = match path.file_name() {
            None => String::from("Unnamed-file"),
            Some(name) => name.to_string_lossy().to_string()
        };
        Some((path.to_owned(), Metadata { path: path.to_owned(), name, data_type: matcher_type, file_size }))
    }).collect();
    
    app.emit("to_send", metadata.clone()).map_err(|err| {
        eprintln!("error sending metadata to client: {}", err);
        String::from("Error sending metadata to client")
    })?;
    let connection_clone = {
        let state = app.state::<Mutex<AppState>>();
        let mut state = match state.lock() {
            Err(err) => {
                eprintln!("error getting state: {}", err);
                return Err(String::from("error getting connection info"));
            }
            Ok(state) => state
        };
        state.to_send = metadata.clone();
        state.connected_to.clone()
    }; 
    let connection = match connection_clone {
        None => return Err(String::from("Please connect to a device first")),
        Some(conn) => conn
    };

    let uni_con = connection.open_uni().await;

    let mut send_stream = match uni_con {
        Err(err) => {
            eprintln!("error connecting to receiver: {}", err);
            return Err(String::from("Connection Error"));
        }
        Ok(stream) => stream
    };

    let metadata_bytes = match bincode::encode_to_vec(&metadata, config::standard()) {
        Err(err) => {
            eprintln!("error getting metadata bytes: {}", err);
            String::from("Error getting metadata bytes").as_bytes().to_vec()
        }
        Ok(bytes) => bytes
    };

    let len = metadata_bytes.len() as u64;
    if let Err(err) = send_stream.write_all(&len.to_be_bytes()).await {
        eprintln!("error sendinf metadata len: {}", err);
        return Err(String::from("error sending metadata length to receiver"));
    }

    match send_stream.write_all(&metadata_bytes).await{
        Err(err) => {
            eprintln!("error sending metadata: {}", err);
            return Err(String::from("Error sending data to client"));
        }
        Ok(_) => {}
    }
    
    for data in metadata {
        if let Err(err) = send_stream.write_all(&data.1.file_size.to_be_bytes()).await{
            eprintln!("error sending file size to receiver: {}", err);
            continue;
        }
        let mut file = match File::open(&data.1.path) {
            Err(err) => {
                eprintln!("error opening file: {}", err);
                continue;
            }
            Ok(file) => file
        };
        
        let mut buffer = [0u8; BUFFER_SIZE];
        let total = data.1.file_size;
        let mut sent: u64 = 0;
        loop {
            let state = app.state::<Mutex<AppState>>();
            let to_cancel = match state.lock() {
                Err(err) => {
                    eprintln!("error getting state: {}", err);
                    false
                }
                Ok(state) => {
                    !state.to_send.contains_key(&data.0)
                }
            };
            if to_cancel {
                send_stream.write_all(&0u64.to_be_bytes()).await.map_err(|err| {
                    eprintln!("error sending cancel signal: {}", err);
                    String::from("Error sending cancel signal")
                })?;
                break; 
            }
            send_stream.write_all(&data.1.file_size.to_be_bytes()).await.map_err(|err| {
                eprintln!("error sending file confirmation: {}", err);
                String::from("Error sending file")
            })?;
            let n = file.read(&mut buffer).map_err(|err| {
                eprintln!("error reading file to buffer: {}", err);
                String::from("Error reading file to buffer")
            })?;
            if n == 0 {
                break;
            }
            send_stream.write_all(&buffer[..n]).await.map_err(|err| {
                eprintln!("error sending file to receiver: {}", err);
                String::from("Error sending file to receiver")
            })?;
            sent += n as u64;
            
            let progress = (sent as f64 / total as f64) * 100.0;
            if let Err(err) = app.emit("progress", serde_json::json!({
                "path": data.0,
                "sent": sent,
                "progress": progress
            })) {
                eprintln!("error sending progress to client: {}", err);
            };
        }
    }
    
    Ok(())
}
