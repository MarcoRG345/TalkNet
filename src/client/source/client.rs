use common::types_msg;
use std::net::TcpStream;
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
	pub fn get_id(&self) -> &String{
		&self.username
	}
}
