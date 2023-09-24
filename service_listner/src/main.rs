mod server;
pub mod zips_db;
mod command;
mod event_loop;
mod app_error;

use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::ptr::null;
use std::sync::{Arc, Mutex};
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::sync::{mpsc, oneshot};
use common_lib::model::{Zipcode, Place};
use crate::command::Command;
use crate::event_loop::EventLoop;
use crate::app_error::AppError;
// use crate::zips_db::ZipsDbManager;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.expect("Failed to bind");

    println!("Server listening on 127.0.0.1:8080");

    let mut zips_db = HashMap::new();
    load_data(&mut zips_db).await;

    // let db_manager = ZipsDbManager::new(&mut zips_db);

    let shared_zips_db = Arc::new(Mutex::new(zips_db));

    let (command_sender, command_receiver) = mpsc::channel::<Command>(32);

    let event_loop = EventLoop::new(command_receiver, shared_zips_db);

    // event loop for the receiver
    tokio::spawn(event_loop.run());

    while let Ok((mut stream, socket_addr)) = listener.accept().await {
        println!("listening from: {}", socket_addr);
        let command_sender_cloned = command_sender.clone();

        tokio::spawn(async move {
            let mut buffer = [0u8; 1024];
            loop {
                let mut reader = BufReader::new(&mut stream);
                let mut line = String::new();
                match reader.read_line(&mut line).await {
                    Ok(0) => {
                        println!("Connection closed");
                        break;
                    }
                    Ok(i) => {
                        line = line.trim_end_matches('\n').to_string();
                        line = line.trim_end_matches('\r').to_string();
                    }
                    Err(_) => {
                        println!("Connection closed");
                        break;
                    }
                }

                /*let n = stream.read(&mut buffer).await.expect("Failed to read from socket");
                if n == 0 {
                    println!("Connection closed");
                    break;
                }*/
                let (resp_tx, resp_rx) = oneshot::channel::<Result<Zipcode, AppError>>();
                command_sender_cloned.send(Command::GetZipDetails {
                    key: line,
                    resp_tx,
                }).await.expect("TODO: panic message");

                if let Ok(resp_result) = resp_rx.await {
                    match resp_result {
                        Ok(zipcode) => {
                            let js = serde_json::to_string(&zipcode).unwrap();
                            stream.write_all(js.as_bytes()).await.expect("Failed to write to socket");
                        }
                        Err(error) => {
                            eprintln!("error {}", error)
                        }
                    }
                }
                // let response = &buffer[..n];
                // socket.write_all(response).await.expect("Failed to write to socket");
            }
        });
    }
}

async fn load_data(zips_db: &mut HashMap<String, Zipcode>) {
    let file_path = "zipcode.json";
    let mut file = File::open(file_path).expect("File open error");
    let mut json_data = String::new();
    file.read_to_string(&mut json_data).expect("read file into String error");
    let parsed_data = serde_json::from_str::<Vec<Zipcode>>(&json_data).expect("De-serialization error");
    parsed_data.iter().for_each(|z| {
        zips_db.insert(z.post_code.clone(), z.clone());
    });
}
