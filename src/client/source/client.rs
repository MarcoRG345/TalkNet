use common::types_msg;
use std::net::TcpStream;
use common::type_protocol;
pub struct Client {
	username: String,
	status: String,
}
impl Client {
    pub fn new(username: String) -> Self {
		let mut status = String::from("AWAY"); 
        Client { username, status }
    }
	pub fn send_identify(&self) -> String{
		let id = self.username.clone();
		let json_data = types_msg::Types_msg::IDENTIFY {
			username: id,
		};
		let json_str = serde_json::to_string(&json_data).unwrap();
		json_str
	}
	pub fn send_pub_text(&self, pub_text: &mut String) -> String{
		let json_data = types_msg::Types_msg::PUBLIC_TEXT {
			text: pub_text.to_string(),
		};
		let json_str = serde_json::to_string(&json_data).unwrap();
		json_str
	}

	pub fn request_users(&self) -> String{
		let json_data = types_msg::Types_msg::USERS;
		let json_str = serde_json::to_string(&json_data).unwrap();
		json_str
	}
	
	pub fn send_priv_text(&self, priv_text: String, id_name: String) -> String{
		let json_data = types_msg::Types_msg::TEXT{
			username: id_name,
			text: priv_text.to_string()
		};
		let json_str = serde_json::to_string(&json_data).unwrap();
		json_str
	}
	
	pub fn get_id(&self) -> &String{
		&self.username
	}
	pub fn request_new_room(&self, room_name: String) -> String{
		let json_data = types_msg::Types_msg::NEW_ROOM{
			roomname: room_name
		};
		serde_json::to_string(&json_data).unwrap()
	}
	pub fn send_room_bid(&self, room_name: String, user_names: Vec<String>) -> String{
		let json_data = types_msg::Types_msg::INVITE{
			roomname: room_name,
			usernames: user_names
		};
		serde_json::to_string(&json_data).unwrap()
	}
	
	pub fn join_room(&self, room_name: String) -> String{
		let json_data = types_msg::Types_msg::JOIN_ROOM{
			roomname: room_name
		};
		serde_json::to_string(&json_data).unwrap()
	}

	pub fn room_users(&self, room_name: String) ->String{
		let json_data = types_msg::Types_msg::ROOM_USERS{
			roomname: room_name
		};
		serde_json::to_string(&json_data).unwrap()
	}
	pub fn room_text(&self,room_name: String, text_: String) -> String{
		let json_data = types_msg::Types_msg::ROOM_TEXT{
			roomname: room_name,
			text: text_
		};
		serde_json::to_string(&json_data).unwrap()
	}
	
}

