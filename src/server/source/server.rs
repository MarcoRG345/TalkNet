use common::connection::Connection;
use common::user::User;
use serde::{Deserialize, Serialize};
use serde_json::{json, map, Result, Value};
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc;
use std::{collections::HashMap, sync::Arc, sync::Mutex, thread};

pub struct Server {
    users: HashMap<User, String>,
    vec: Vec<String>,
}

impl Server {
    pub fn new() -> Self {
        let mut users_name_satus: HashMap<User, String> = HashMap::new();
        let mut arr = Vec::new();
        Server {
            users: users_name_satus,
            vec: arr,
        }
    }

    fn handle_client(&mut self, mut socket: TcpStream) {
        //Aqui se envian los jsons.
        let mut buffer = Vec::new();
        let mut reader = BufReader::new(socket.try_clone().unwrap());
        loop {
            buffer.clear();
            match reader.read_until(b'\n', &mut buffer) {
                Ok(0) => {
                    println!("Client disconnected.");
                    break;
                }
                Ok(_) => {
                    let message = String::from_utf8_lossy(&buffer);
                    let trimmed_message = message.trim_end();
                }
                Err(e) => {
                    println!("Failed to read from stream: {}", e);
                    break;
                }
            }
        }
        let data_json = serde_json::to_value(&buffer).unwrap();
        let user: User = User::new(
            &data_json["username"],
            serde_json::to_value(String::from("AWAY")).unwrap(),
        );
        let mut write_json;
        if self.users.contains_key(&user) {
            write_json = json![{
                "type":"RESPONSE",
                "request":"IDENTIFY",
                "result":"USER_ALREADY_EXISTS",
                "extra": user.get_id().clone()
            }];
        } else {
            write_json = json![{
                "type":"RESPONSE",
                "request":"IDENTIFY",
                "result":"SUCCESS",
                "extra": user.get_id().clone()
            }];

            self.users.insert(user, String::from("AWAY"));
        }
        let binding = write_json.to_string();
        let data_bytes = binding.as_bytes();
        socket.write(&data_bytes);
    }

    pub fn start_server(self) {
        let listener = TcpListener::bind("127.0.0.1:8001").unwrap();
        let mut share_server = Arc::new(Mutex::new(self));
        loop {
            match listener.accept() {
                Ok((socket, addr)) => {
                    let mut share_server_thr = Arc::clone(&share_server);
                    //Accept new client
                    thread::spawn(move || {
                        share_server_thr.lock().unwrap().handle_client(socket);
                    });
                }
                Err(e) => println!("any client detected: {e:?}"),
            }
        }
    }
    pub fn get_all_users(&self) -> &HashMap<User, String> {
        &self.users
    }
}
