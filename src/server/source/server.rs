use common::connection::Connection;
use common::user::User;
use serde::{Deserialize, Serialize};
use serde_json::{json, map, Result, Value};
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use tokio::sync::{mpsc, Mutex, mpsc::Sender, mpsc::UnboundedSender};
use std::{collections::HashMap, sync::Arc};

pub struct Server {
    users: HashMap<User, String>,
	//Sender de los clientes son estos.
	suscribers: Arc<Mutex<HashMap<String, UnboundedSender<String>>>>,
}

impl Server {
    pub fn new() -> Self {
        let mut users_name_satus: HashMap<User, String> = HashMap::new();
        Server {
            users: users_name_satus,
			suscribers: Arc::new(Mutex::new(HashMap::new())),
        }
    }
	
	pub async fn suscribe(&self, username: String, sender: UnboundedSender<String>){
		let mut channels_key = self.suscribers.lock().await;
		if !channels_key.contains_key(&username){
			channels_key.insert(username.trim().replace("\n", ""), sender);
		}else {
			println!("Same cliente try to joined");
		}
	}
	pub async fn publish(&self, message: String){
		let mut channels_key = self.suscribers.lock().await;
		for (_key, senders) in channels_key.iter_mut(){
			senders.send(message.clone().trim().replace("\n", "n"));
		}
	}
}
