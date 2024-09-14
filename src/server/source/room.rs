use tokio::sync::{mpsc, Mutex, mpsc::UnboundedSender};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use serde_json::{json, map, Result, Value};
use std::collections::HashMap;


pub struct Room{
	room_users: Arc<Mutex<HashMap<String, UnboundedSender<String>>>>,
	invitations: Arc<Mutex<HashMap<String, String>>>,
}
impl Room{

	pub fn new() -> Self{
		Room{
			room_users: Arc::new(Mutex::new(HashMap::new())),
			invitations: Arc::new(Mutex::new(HashMap::new())),
		}
	}
	
	
	pub async fn add_user(&self, username: String, sender: UnboundedSender<String>){
		let mut room_users_lock = self.room_users.lock().await;
		let mut json_str = String::new();
		if !room_users_lock.contains_key(&username){
			room_users_lock.insert(username.clone(), sender.clone());
		}
	}

	pub async fn suscribe_me(&self, username: String, room_name: String){
		let mut current_invitations = self.invitations.lock().await;
		current_invitations.insert(username.clone(), room_name.clone());
	}
	
	pub async fn is_invited(&self, username: String) -> bool{
		let mut current_invitations = self.invitations.lock().await;
		if  current_invitations.contains_key(&username){
			return true;
		}
		return false;
	}
	pub async fn is_invited_into(&self, username: String, room_name: String) -> bool{		
		let mut current_invitations = self.invitations.lock().await;
		println!("{:?}", current_invitations);
		if current_invitations.contains_key(&username) {
			let mut roomname = current_invitations.get(&username).unwrap();
			if *roomname == room_name { return true}
		}
		return false;
	}
	pub async fn publish_room(&self, general_protocol: String, without: String){
		let room_users = self.room_users.lock().await;
		for (user, channel) in room_users.iter(){
			if !(*user == without){
				channel.send(general_protocol.clone());
			}
		} 
	}
	
}
