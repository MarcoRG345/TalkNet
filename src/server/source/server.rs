use common::{type_protocol, types_msg};
use serde::{Deserialize, Serialize};
use serde_json::{json, map, Result, Value};
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use tokio::sync::{mpsc, Mutex, mpsc::Sender, mpsc::UnboundedSender};
use std::{collections::HashMap, sync::Arc};
//mod room; // Declarar el módulo `room`
use crate::room::Room; 
pub struct Server {
    users: Arc<Mutex<HashMap<String, String>>>,
	//HashMap of username to channel.
	suscribers: Arc<Mutex<HashMap<String, UnboundedSender<String>>>>,
	//HashMap of uuid_auth to username.
	current_users: Arc<Mutex<HashMap<String, String>>>,
	//
	current_rooms: Arc<Mutex<HashMap<String, Room>>>,
}

impl Server {	
    pub fn new() -> Self {
        let mut users_name_satus = Arc::new(Mutex::new(HashMap::new()));
        Server {
            users: users_name_satus,
			suscribers: Arc::new(Mutex::new(HashMap::new())),
			current_users: Arc::new(Mutex::new(HashMap::new())),
			current_rooms: Arc::new(Mutex::new(HashMap::new())),
        }
    }
	
	pub async fn suscribe(&self, request_protc: String, sender: UnboundedSender<String>){
		let mut channels_key = self.suscribers.lock().await;
		let mut users_key =  self.users.lock().await;
		let key_username = Self::get_id(request_protc.clone()).trim().replace("\n","");
		if !channels_key.contains_key(&key_username){
			channels_key.insert(key_username.clone(), sender.clone());
			users_key.insert(key_username.clone(), "AWAY".to_string());
		}else{
			self.deny_indentify(request_protc, sender);
		}
	}
	
	pub fn deny_indentify(&self, protocol: String, channel: UnboundedSender<String>){
		let id = Self::get_id(protocol.clone().to_string());
		let json_data = type_protocol::Type_protocol::RESPONSE {
				operation: "IDENTIFY".to_string(),
				result: type_protocol::ResultType::USER_ALREADY_EXISTS,
				extra: id.clone()
			};
			let json_str = serde_json::to_string(&json_data).unwrap();
			channel.send(json_str);
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
		
		if json_data.contains("USERS") && !json_data.contains("ROOM"){
			let users_list = self.response_users().await;
			current_sender.send(users_list);
			
		}else if json_data.contains("TEXT") && json_data.contains("username") && !json_data.contains("ROOM"){
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
		
		else if json_data.contains("PUBLIC_TEXT") && json_data.contains("text") && !json_data.contains("ROOM"){
			let mut channels_key = self.suscribers.lock().await;
			let pub_value = self.response_pub_text(current_auth, general_protocol).await;  
			for (_key, senders) in channels_key.iter_mut(){
				
				senders.send(pub_value.clone());
			}
		}
		else if json_data.contains("NEW_ROOM"){
			let mut json_data = self.response_room(current_auth, general_protocol, current_sender.clone()).await;
			current_sender.send(json_data.clone());
		}else {
			let json_data = serde_json::from_str(&general_protocol).unwrap();
			self.handle_room(json_data, current_auth).await;
		}
		
	}
	pub async fn response_indentify(&self, user_key: String){
		let id = Self::get_id(user_key.to_string());
		let json_data = type_protocol::Type_protocol::RESPONSE {
			operation: "IDENTIFY".to_string(),
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
			println!("{}", json_str.to_string());
		}else{
			//None user, or username incorrect.
			let json_data = type_protocol::Type_protocol::RESPONSE{
				operation: "TEXT".to_string(),
				result: type_protocol::ResultType::NO_SUCH_USER,
				extra: id_name.to_string()
			};
			json_str = serde_json::to_string(&json_data).unwrap();
		}
		json_str
	}
	async fn response_room(&self, auth: String, general_protocol: String, sender: UnboundedSender<String>) -> String{
		let new_room = Room::new();
		let channels_key = self.suscribers.lock().await;
		let current_users_key = self.current_users.lock().await;
		let mut current_rooms_key = self.current_rooms.lock().await;
		
		let username = current_users_key.get(&auth).unwrap();
		let json_serialize: types_msg::Types_msg = serde_json::from_str(&general_protocol).unwrap();
		let mut room_name = String::new();
		
		if let types_msg::Types_msg::NEW_ROOM {roomname } = json_serialize{
			room_name = roomname.to_string();
		}

		let mut json_str_data = String::new();
		if !current_rooms_key.contains_key(&room_name){
			new_room.add_user(username.to_string(), sender.clone()).await;
			new_room.suscribe_me(username.to_string(), room_name.clone()).await;
			current_rooms_key.insert(room_name.clone(), new_room);
			let json_serialize = type_protocol::Type_protocol::RESPONSE{
				operation: "NEW_ROOM".to_string(),
				result: type_protocol::ResultType::SUCCESS,
				extra: room_name
			};
			json_str_data = serde_json::to_string(&json_serialize).unwrap();
		}else{
			let json_serialize = type_protocol::Type_protocol::RESPONSE{
				operation: "NEW_ROOM".to_string(),
				result: type_protocol::ResultType::ROOM_ALREADY_EXISTS,
				extra: room_name
			};
			json_str_data = serde_json::to_string(&json_serialize).unwrap();
		}
		json_str_data
	}

	async fn handle_room(&self, type_room: types_msg::Types_msg, current_auth: String){
		let current_users_key = self.current_users.lock().await;
		let current_user = current_users_key.get(&current_auth).unwrap().to_string();
		if let types_msg::Types_msg::INVITE{roomname, usernames} = type_room{
			let room_name = roomname.to_string();
			self.manage_invite(room_name.clone(), current_user.clone(), usernames).await;	
		}else if let types_msg::Types_msg::JOIN_ROOM{roomname} = type_room{
			let room_name = roomname.to_string();
			self.manage_join(current_user.clone(), room_name.clone()).await;
		}else if let types_msg::Types_msg::ROOM_USERS{roomname} = type_room{
			let room_name = roomname.to_string();
			self.manage_room_users(current_user.clone(), room_name.clone()).await;
		}else if let types_msg::Types_msg::ROOM_TEXT{roomname, text} = type_room{
			let texting = text.to_string();
			let room_name = roomname.to_string();
			self.manage_room_text(current_user.clone(), room_name.clone(), texting.clone()).await
		}
	}

	async fn manage_invite(&self, room_name: String, current_user: String, usernames: Vec<String>){
		//let room_name = roomname.to_string();
		let mut json_str = String::new();
		let mut channels_key = self.suscribers.lock().await;
		let mut current_rooms_key = self.current_rooms.lock().await;
			let err_room_invited = type_protocol::Type_protocol::RESPONSE{
				operation: "INVITE".to_string(),
				result: type_protocol::ResultType::NO_SUCH_ROOM,
				extra: room_name.clone()
			};
			let current_sender = channels_key.get(&current_user).unwrap();
			if current_rooms_key.contains_key(&room_name){
				let room = current_rooms_key.get(&room_name).unwrap();
				
				if room.is_invited(current_user.clone()).await{}{
					let json_data = type_protocol::Type_protocol::INVITATION{
						username: current_user.clone(),
						roomname: room_name.clone()	
					};
					json_str = serde_json::to_string(&json_data).unwrap();
					
					for user in &usernames {
						if !channels_key.contains_key(&user.clone()){
							//userno exists
							let err_json = type_protocol::Type_protocol::RESPONSE{
								operation:"INVITE".to_string(),
								result: type_protocol::ResultType::NO_SUCH_USER,
								extra: user.clone()
							};
							json_str = serde_json::to_string(&err_json).unwrap();
							current_sender.send(json_str.clone());
						}else{
							room.suscribe_me(user.clone(), room_name.clone()).await;
							let sender = channels_key.get(user).unwrap();
							println!("{}", json_str.clone());
							sender.send(json_str.clone());
						}
					}
				}
			}else{
				json_str = serde_json::to_string(&err_room_invited).unwrap();
				println!("{}", json_str.clone());
				current_sender.send(json_str.clone());
			}
	}
	async fn manage_join(&self, current_user: String, room_name: String){
		let mut current_rooms_key = self.current_rooms.lock().await;
		let mut channels_key = self.suscribers.lock().await;
		//la sala existe o no existe.
		if current_rooms_key.contains_key(&room_name){
			let room = current_rooms_key.get(&room_name).unwrap();
			if room.is_invited_into(current_user.clone(), room_name.clone()).await{
				let sender = channels_key.get(&current_user).unwrap();
				let json_data = type_protocol::Type_protocol::RESPONSE{
					operation: "JOIN_ROOM".to_string(),
					result: type_protocol::ResultType::SUCCESS,
					extra: room_name.clone()
				};
				let mut  json_str = serde_json::to_string(&json_data).unwrap();
				sender.send(json_str.clone());
				let json_advice = type_protocol::Type_protocol::JOINED_ROOM{
					roomname: room_name.clone(),
					username: current_user.clone()
				};
				room.add_user(current_user.clone(), sender.clone()).await;
				json_str = serde_json::to_string(&json_advice).unwrap();
				room.publish_room(json_str.clone(), current_user.clone()).await;
			}else{
				let err_json = type_protocol::Type_protocol::RESPONSE{
					operation: "JOIN_ROOM".to_string(),
					result: type_protocol::ResultType::NOT_INVITED,
					extra: room_name.clone()
				};
				let err_json_str = serde_json::to_string(&err_json).unwrap();
				let sender = channels_key.get(&current_user).unwrap();
				sender.send(err_json_str.clone());
			}
		}else{
			let err_json = type_protocol::Type_protocol::RESPONSE{
				operation: "JOIN_ROOM".to_string(),
				result: type_protocol::ResultType::NO_SUCH_ROOM,
				extra: room_name.clone()
			};
			let err_json_str = serde_json::to_string(&err_json).unwrap();
			let sender = channels_key.get(&current_user).unwrap();
			sender.send(err_json_str.clone());
		}
	}
	async fn manage_room_users(&self, current_user: String, room_name: String){
		let current_rooms_key = self.current_rooms.lock().await;
		let mut channels_key = self.suscribers.lock().await;
		let users_status = self.users.lock().await;
		if !current_rooms_key.contains_key(&room_name){
			let json_data = type_protocol::Type_protocol::RESPONSE{
				operation: "ROOM_USERS".to_string(),
				result: type_protocol::ResultType::NO_SUCH_ROOM,
				extra: room_name.to_string()
			};
			let json_str = serde_json::to_string(&json_data).unwrap();
			let sender = channels_key.get(&current_user).unwrap();
			sender.send(json_str.clone());
		}else if current_rooms_key.contains_key(&room_name){
			let room = current_rooms_key.get(&room_name).unwrap();
			let mut  temp_HashMap: HashMap<String, String> = HashMap::new();
			for (user, status) in users_status.iter(){
				let us = user.to_string();
				if room.is_in_room(user.to_string()).await{
					temp_HashMap.insert(us, status.to_string());
				}
			}
			if room.is_in_room(current_user.clone()).await{
				let json_data = type_protocol::Type_protocol::ROOM_USER_LIST{
					roomname: room_name.clone(),
					users: temp_HashMap
				};
				let json_str = serde_json::to_string(&json_data).unwrap();
				let sender = channels_key.get(&current_user).unwrap();
				sender.send(json_str);	
			}
			else{
			let json_data = type_protocol::Type_protocol::RESPONSE{
				operation: "ROOM_USERS".to_string(),
				result: type_protocol::ResultType::NOT_JOINED,
				extra: room_name.clone()
			};
			let json_str = serde_json::to_string(&json_data).unwrap();
			println!("{}", json_str.clone());
			let sender = channels_key.get(&current_user).unwrap();
			sender.send(json_str);
		} 
		}
	}
	async fn manage_room_text(&self, current_user: String, room_name: String, texting: String){
		let current_rooms_key = self.current_rooms.lock().await;
		let channels_key = self.suscribers.lock().await;
		if current_rooms_key.contains_key(&room_name){
			let room = current_rooms_key.get(&room_name).unwrap();
			if room.is_in_room(current_user.clone()).await{
				let json_data = type_protocol::Type_protocol::ROOM_TEXT_FROM{
					roomname:room_name.clone(),
					username:current_user.clone(),
					text: texting.trim().to_string()
				};
				let json_str = serde_json::to_string(&json_data).unwrap();
				room.publish_room(json_str.clone(), current_user.clone()).await;
			}else{
				let json_data = type_protocol::Type_protocol::RESPONSE{
					operation: "ROOM_TEXT".to_string(),
					result: type_protocol::ResultType::NOT_JOINED,
					extra: room_name.clone()
				};
				let json_str = serde_json::to_string(&json_data).unwrap();
				let sender = channels_key.get(&current_user).unwrap();
				sender.send(json_str);
			}
		}else{
			let json_data = type_protocol::Type_protocol::RESPONSE{
				operation: "ROOM_TEXT".to_string(),
				result: type_protocol::ResultType::NO_SUCH_ROOM,
				extra: room_name.clone()
			};
			let json_str = serde_json::to_string(&json_data).unwrap();
			println!("{}", json_str);
			let sender = channels_key.get(&current_user).unwrap();
			sender.send(json_str);
		} 
	}
}
