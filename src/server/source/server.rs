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
	current_users: Arc<Mutex<HashMap<String, String>>>,
}

impl Server {	
    pub fn new() -> Self {
        let mut users_name_satus = Arc::new(Mutex::new(HashMap::new()));
        Server {
            users: users_name_satus,
			suscribers: Arc::new(Mutex::new(HashMap::new())),
			current_users: Arc::new(Mutex::new(HashMap::new())),
        }
    }
	
	pub async fn suscribe(&self, request_protc: String, sender: UnboundedSender<String>){
		let mut channels_key = self.suscribers.lock().await;
		let mut users_key =  self.users.lock().await;
		let key_username = Self::get_id(request_protc.clone()).trim().replace("\n","");
		if !channels_key.contains_key(&key_username){
			channels_key.insert(key_username.clone(), sender.clone());
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
	pub async fn suscribe_auth(&self, auth: String, username: String){
		let mut suscribe_auth = self.current_users.lock().await;
		let only_user = Self::get_id(username);
		suscribe_auth.insert(auth, only_user);	
	}
	//Entry point.
	pub async fn publish(&self, current_auth:String, general_protocol: String, current_sender: UnboundedSender<String>){
		let json_data = general_protocol.clone();
		
		if json_data.contains("USERS"){
			let users_list = self.response_users().await;
			current_sender.send(users_list);
			
		}else if json_data.contains("TEXT") && json_data.contains("username"){
			let priv_text =  self.response_text(current_auth.clone(), json_data.clone()).await;
			let mut channels_key = self.suscribers.lock().await;
			//respuesta positiva
			let json_serialize: types_msg::Types_msg = serde_json::from_str(&json_data).unwrap();
			if priv_text.contains("username"){				
				if let types_msg::Types_msg::TEXT{username, text} = json_serialize{
					let user = username.to_string();
					let sender_value = channels_key.get(&user).unwrap();
					sender_value.send(priv_text);
				}
			}else{
				current_sender.send(priv_text);
			}
		}
		
		else if json_data.contains("PUBLIC_TEXT") && json_data.contains("text"){
			let mut channels_key = self.suscribers.lock().await;
			let pub_value = self.response_pub_text(current_auth, general_protocol).await;  
			for (_key, senders) in channels_key.iter_mut(){
				
				senders.send(pub_value.clone());
			}
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
			username: id.clone()
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
	
	async fn response_pub_text(&self, current_auth: String, general_protocol: String) -> String{
		let current_users_key = self.current_users.lock().await;
		let json_request: types_msg::Types_msg = serde_json::from_str(&general_protocol).unwrap();
		let mut text_content = String::new();
		if let types_msg::Types_msg::PUBLIC_TEXT{text} = json_request{
			text_content = text.to_string(); 
		}
		let user_id = current_users_key.get(&current_auth).unwrap();
		let json_response = type_protocol::Type_protocol::PUBLIC_TEXT_FROM{
			username: user_id.to_string(),
			text: text_content.to_string()
		};
		serde_json::to_string(&json_response).unwrap()
	}
	
	async fn response_text(&self, current_auth: String, priv_protocol: String) -> String{
		let users_key = self.users.lock().await;
		let mut channels_key = self.suscribers.lock().await;
		let current_users_key = self.current_users.lock().await;
		let mut id_name = String::new();
		let mut texting = String::new();
		let json_deserialize: types_msg::Types_msg = serde_json::from_str(&priv_protocol).unwrap();
		if let types_msg::Types_msg::TEXT {username, text} = json_deserialize{
			id_name = username;
			texting = text;
		}
		let key_id = id_name.clone();
		let mut json_str = String::new();
		let text_from = current_users_key.get(&current_auth).unwrap();
		if let Some(tuple_values) = users_key.get_key_value(key_id.as_str()){
			let tuple_id = tuple_values.0;
			let json_data = type_protocol::Type_protocol::TEXT_FROM{
				username: text_from.to_string(),
				text: texting.to_string()
			};
			json_str = serde_json::to_string(&json_data).unwrap();
		}else{
			//None user, or username incorrect.
			let json_data = type_protocol::Type_protocol::RESPONSE{
				request: "TEXT".to_string(),
				result: type_protocol::ResultType::NO_SUCH_USER,
				extra: id_name.to_string()
			};
			json_str = serde_json::to_string(&json_data).unwrap();
		}
		json_str
	}
}
