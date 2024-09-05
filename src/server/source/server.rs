use common::connection::Connection;
use common::user::User;
use serde::{Deserialize, Serialize};
use serde_json::{json, Result, Value};
use std::net::TcpListener;
use std::{collections::HashMap, sync::Arc, sync::Mutex, thread};
pub struct Server {
    users: HashMap<User, String>,
}

impl Server {
    pub fn new() -> Self {
        let mut users_name_satus: HashMap<User, String> = HashMap::new();
        Server {
            users: users_name_satus,
        }
    }

    fn handle_client(connection: &Connection) {
        //Aqui se envian los jsons.
        let data_json = connection.recv_from(connection.get_stream());
    }

    pub fn start_server(&self) {
        let listener = TcpListener::bind("127.0.0.1:8001").unwrap();
        loop {
            match listener.accept() {
                Ok((socket, addr)) => {
                    //Accept new client
                    thread::spawn(move || {
                        let connection = Connection::new(socket);
                        Self::handle_client(&connection);
                    });
                }
                Err(e) => println!("any client detected: {e:?}"),
            }
        }
    }
}
