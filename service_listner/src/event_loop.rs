use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use common_lib::model::Zipcode;
use crate::command::Command;

pub struct EventLoop {
    command_receiver: mpsc::Receiver<Command>,
    shared_zips_db: Arc<Mutex<HashMap<String, Zipcode>>>,
}

impl EventLoop {
    pub fn new(command_receiver: mpsc::Receiver<Command>, shared_zips_db: Arc<Mutex<HashMap<String, Zipcode>>>) -> Self {
        Self {
            command_receiver,
            shared_zips_db,
        }
    }

    pub async fn run(mut self) {
        loop {
            tokio::select! {
                option_command = self.command_receiver.recv() => match option_command {
                    Some(command) => {
                        self.handle_command(command).await;
                    },
                    // Command channel closed, thus shutting down the network event loop.
                    None => { },
                },
            }
        }
    }

    async fn handle_command(&mut self, command: Command) {
        match command {
            Command::GetZipDetails {
                key,
                resp_tx: callback_tx,
            } => {
                let zips_db = self.shared_zips_db.lock().unwrap();
                match zips_db.get(key.as_str()) {
                    None => {
                        println!("zipcode not found");
                    }
                    Some(zipcode) => {
                        callback_tx.send(Ok(zipcode.clone())).expect("TODO: panic message");
                        let zipcode_json = serde_json::to_string(&zipcode).unwrap();
                        println!("{}", zipcode_json);
                    }
                }
            }
            Command::UpdateZipDetails {
                key: _,
                zipcode: _,
                resp_tx: _,
            } => {}
        }
    }
}