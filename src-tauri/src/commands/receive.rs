use std::{
    collections::HashMap,
    fs::{self, File},
    io::Write,
    path::PathBuf,
    sync::Mutex,
    time::Instant,
};

use bincode::config;
use tauri::{AppHandle, Emitter, Manager};

use crate::{commands::send::BUFFER_SIZE, AppState, Metadata};

#[tauri::command]
pub async fn receive_file(app: AppHandle) -> Result<(), String> {
    
    let (connection_clone, transfer_dir) = {
        let state = app.state::<Mutex<AppState>>();
        let state = state.lock().map_err(|err| {
            eprintln!("error getting state before receiving the files: {}", err);
            String::from("Error getting state before receiving files")
        })?;
        match state.connected_to.clone() {
            None => {
                eprintln!("No connection found in the app state");
                return Err(String::from("No connection found in app state"));
            }
            Some(conn) => (conn, state.transfer_dir.clone()),
        }
    };
    while let Ok(mut recv_stream) = connection_clone.accept_uni().await{    
        let mut len_buff = [0u8; 8];
        if let Err(err) = recv_stream.read_exact(&mut len_buff).await {
            eprintln!("error receiving size of the metadata:{}", err);
            return Err(String::from("Error receiving size of the metadata"));
        }
    
        let len = u64::from_be_bytes(len_buff);
        let mut metadata_bytes = vec![0u8; len as usize];
        recv_stream
            .read_exact(&mut metadata_bytes)
            .await
            .map_err(|err| {
                eprintln!("error readinf metadata:{}", err);
                String::from("Error reading metadata")
            })?;
    
        let (metadata, _): (HashMap<PathBuf, Metadata>, _) =
            bincode::decode_from_slice(&metadata_bytes, config::standard()).map_err(|err| {
                eprintln!("error decoding metadata: {}", err);
                String::from("error decoding metadata")
            })?;
        app.emit("to_receive", metadata.clone()).map_err(|err| {
            eprintln!("error sending metadata to frontend: {}", err);
            String::from("Error showing metadata")
        })?;
    
        for data in metadata {
            let start = Instant::now();
            let mut last_update = Instant::now();
            let mut len_buff = [0u8; 8];
            recv_stream.read_exact(&mut len_buff).await.map_err(|err| {
                eprintln!("error getting file size:{}", err);
                String::from("Error getting file size from the sender")
            })?;
    
            let file_type = &data.1.data_type;
    
            let file_dir = transfer_dir.join(file_type.to_string());
    
            fs::create_dir_all(&file_dir).map_err(|err| {
                eprintln!("error creating folder:{}", err);
                String::from("Error creating folder for receiving file")
            })?;
    
            let file_path = {
                let path = file_dir.join(&data.1.name);
                if !path.exists() {
                    path
                } else {
                    let mut new_path = path.clone();
                    let parent = &file_dir;
                    let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
                    let extension = path.extension();
                    let mut counter = 1;
                    while new_path.exists() {
                        let new_file_name = {
                            match extension {
                                None => PathBuf::from(parent.join(&format!("{}({})", stem, counter))),
                                Some(ext) => {
                                    let ext = ext.to_str().unwrap_or("");
                                    PathBuf::from(
                                        parent.join(&format!("{}({}).{}", stem, counter, ext)),
                                    )
                                }
                            }
                        };
                        new_path = new_file_name;
                        counter += 1;
                    }
                    path
                }
            };
            let mut file = File::create(file_path).map_err(|err| {
                eprintln!("error creating file: {}", err);
                String::from("Error creating file")
            })?;
    
            let mut received = 0u64;
            let file_size = data.1.file_size;
            let mut remaining = u64::from_be_bytes(len_buff);
            let mut buffer = vec![0u8; BUFFER_SIZE];
            let mut last_bytes = 0u64;
    
            while remaining > 0 {
                let mut size_buf = [0u8; 4];
                recv_stream.read_exact(&mut size_buf).await.map_err(|err| {
                    eprintln!("error getting confirmattion from sender: {}", err);
                    String::from("error getting confirmation from sender")
                })?;
                let to_continue = u32::from_be_bytes(size_buf);
    
                if to_continue == 0 {
                    app.emit("receive_stop", data.0.clone()).ok();
                    break;
                }
    
                let n = recv_stream.read(&mut buffer).await.map_err(|err| {
                    eprintln!("error receiving data from the sender: {}", err);
                    String::from("Error receiving data from the sender")
                })?;
                match n {
                    None => break,
                    Some(num) => {
                        if num == 0 {
                            break;
                        }
                        file.write_all(&buffer[..num]).map_err(|err| {
                            eprintln!("error writing file to disk: {}", err);
                            String::from("Error writing file to disk")
                        })?;
                        remaining -= num as u64;
                        received += num as u64;
                        if last_update.elapsed().as_secs() > 1 {
                            let elapsed = last_update.elapsed().as_secs_f64();
                            let bytes_diff = received - last_bytes;
    
                            let speed = bytes_diff as f64 / elapsed;
    
                            let speed_mbps: f64 = ((speed / (1024.0 * 1024.0)) * 100.0).round() / 100.0;
    
                            let progress = (received as f64 / file_size as f64) * 100.0;
    
                            app.emit(
                                "receive_progress",
                                serde_json::json!({
                                    "path": data.0,
                                    "transferred": received,
                                    "progress": progress,
                                    "speed": speed_mbps
                                }),
                            )
                            .ok();
    
                            last_update = Instant::now();
                            last_bytes = received;
                        }
                    }
                }
            }
            let completed_in = start.elapsed().as_secs_f32();
            app.emit(
                "file_received",
                serde_json::json!({
                    "path": data.0,
                    "completed_in": completed_in
                }),
            )
            .ok();
        }        
    };
    Ok(())
}
