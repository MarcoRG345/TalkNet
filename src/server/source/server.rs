use common::{type_protocol, types_msg};
use serde::{Deserialize, Serialize};
use serde_json::{json, map, Result, Value};
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use tokio::sync::{mpsc, Mutex, mpsc::Sender, mpsc::UnboundedSender};
use std::{collections::HashMap, sync::Arc};

pub struct Server {
    users: Arc<Mutex<HashMap<String, String>>>,
	//Sender de los clientes son estos.
	suscribers: Arc<Mutex<HashMap<String, UnboundedSender<String>>>>,
}

impl Server {
    pub fn new() -> Self {
        let mut users_name_satus = Arc::new(Mutex::new(HashMap::new()));
        Server {
            users: users_name_satus,
			suscribers: Arc::new(Mutex::new(HashMap::new())),
        }
    }
	
	pub async fn suscribe(&self, request_protc: String, sender: UnboundedSender<String>){
		let mut channels_key = self.suscribers.lock().await;
		let mut users_key =  self.users.lock().await;
		let key_username = Self::get_id(request_protc.clone()).trim().replace("\n","");
		if !channels_key.contains_key(&key_username){
			channels_key.insert(key_username.clone(), sender);
			users_key.insert(key_username.clone(), "AWAY".to_string());
		}else {;
			let json_data = type_protocol::Type_protocol::RESPONSE {
				request: "IDENTIFY".to_string(),
				result: type_protocol::ResultType::USER_ALREADY_EXISTS,
				extra: key_username.clone()
			};
		}
	}
	 fn get_id(request_protc: String) -> String{
		let json_data: types_msg::Types_msg = serde_json::from_str(request_protc.as_str()).unwrap();
		let mut id = String::new();
		if let types_msg::Types_msg::IDENTIFY {username} = json_data{
			id = username;
		}
		id
		
	 }
	//Entry point.
	pub async fn publish(&self, general_protocol: String, current_sender: UnboundedSender<String>){
		let mut channels_key = self.suscribers.lock().await;
		let json_data = general_protocol.clone();
		if json_data.contains("USERS"){
			let users_list = self.response_users().await;
			current_sender.send(users_list);
		}
		for (_key, senders) in channels_key.iter_mut(){
			senders.send(general_protocol.clone().trim().replace("\n", ""));
		}
	}
	pub async fn response_indentify(&self, user_key: String){
		let id = Self::get_id(user_key.to_string());
		let json_data = type_protocol::Type_protocol::RESPONSE {
			request: "IDENTIFY".to_string(),
			result: type_protocol::ResultType::SUCCESS,
			extra: id.clone()
		};
		let response_all = type_protocol::Type_protocol::NEW_USER{
			username: id
		};
		let mut channels_key = self.suscribers.lock().await;
		let json_str_server = serde_json::to_string(&json_data).unwrap();
		let json_str_clients = serde_json::to_string(&response_all).unwrap();
		for (_key, senders) in channels_key.iter_mut(){
			senders.send(json_str_clients.clone().trim().replace("\n", "n"));
		}	
	}
	async fn response_users(&self) -> String{
		let mut users_key = self.users.lock().await;
		let json_data = type_protocol::Type_protocol::USER_LIST{
			users: users_key.clone() 	
		};
		let json_str = serde_json::to_string(&json_data).unwrap();
		json_str
	}
}
