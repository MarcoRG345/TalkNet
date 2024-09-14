use tokio::io::{self,BufReader,AsyncBufReadExt,AsyncReadExt, AsyncWriteExt, stdout};
use tokio::net::TcpStream;
use std::io::Write;
use std::sync::Arc;
use regex::Regex;
use tokio::sync::{mpsc, Mutex};
mod client;
use common::{type_protocol, types_msg};
#[tokio::main]
async fn main() {
	let mut stream = TcpStream::connect("127.0.0.1:8001").await.unwrap();
	let (mut rd, mut writer) = io::split(stream);
	let (sender_tx, mut receiver_rx) = mpsc::unbounded_channel::<String>();
	tokio::spawn(async move {
		loop{
			if let Some(msg) = receiver_rx.recv().await{
				writer.write_all(msg.as_bytes()).await;
			}
		}
	});
	//thread to incoming messages
	let (request_tx,  mut response_rx)= mpsc::unbounded_channel::<String>();
	tokio::spawn(async move {
		loop {
			let mut buffer = [0; 1024];
			match rd.read(&mut buffer).await {
				Ok(n) => {
					let response = String::from_utf8_lossy(&buffer[..n]);
					if !response.is_empty(){
						responses::type_response(response.to_string());
						print!(" --> CHAT: ");
						std::io::stdout().flush().unwrap();
					}		
				},
				Ok(0) => return,
				Err(err) => return,
			}
		}
	});
	print!("ENTER username: ");
	std::io::stdout().flush();
	let mut buffer = String::new();

	std::io::stdin().read_line(&mut buffer).unwrap();
	let username = buffer.trim().to_string();
	let mut client = client::Client::new(username.clone());
	sender_tx.send(client.send_identify());
	loop{
		let mut buf = String::new();
		std::io::stdin().read_line(&mut buf).unwrap();
		let mut type_protocol = buf.trim();
		let mut proccess_protocol = String::new();
		//Checa las diferentes formas de comunicarte en el chat.
		if type_protocol.starts_with(">"){
			proccess_protocol = type_protocol.replace(">", "").clone();
			sender_tx.send(client.send_pub_text(&mut proccess_protocol));
			
		}else if type_protocol.starts_with("all/"){
			sender_tx.send(client.request_users());
			
		}else if type_protocol.starts_with("txfrom/"){
			proccess_protocol = type_protocol.replace("txfrom/", "").clone();
			let cut_protoc = proccess_protocol.find('/').unwrap_or(proccess_protocol.len());
			let mut id_name: String = proccess_protocol.drain(..cut_protoc).collect();
			id_name = id_name.trim().replace(" ", "");
			let mut priv_text = proccess_protocol.replace("/", "");
			sender_tx.send(client.send_priv_text(priv_text.to_string(), id_name));
			
		}else if type_protocol.starts_with("room/"){
			proccess_protocol = type_protocol.replace("room/", "").clone();
			proccess_protocol = proccess_protocol.trim().to_string();
			sender_tx.send(client.request_new_room(proccess_protocol.clone()));
			
		}else if type_protocol.starts_with("INVITE/"){
			proccess_protocol = type_protocol.replace("INVITE/", "").clone();
			let re = Regex::new(r"<([^>]+)> <([^>]+)>").unwrap();
			 if let Some(capturas) = re.captures(proccess_protocol.as_str()) {
       			 let users_names = &capturas[1]; // Primera subcadena capturada
       			 let mut room_name = &capturas[2];
				 let vec_users: Vec<String> = users_names.split_whitespace().map(|s| s.to_string()).collect();
				 room_name = room_name.trim();
				 sender_tx.send(client.send_room_bid(room_name.to_string(), vec_users));
			} 
		}else if type_protocol.starts_with("JOIN/"){
			proccess_protocol = type_protocol.replace("JOIN/", "").clone();
			proccess_protocol = proccess_protocol.trim().to_string();
			sender_tx.send(client.join_room(proccess_protocol.clone()));
			
		}else if type_protocol.starts_with("ROOM_CONTENT/"){
			proccess_protocol = type_protocol.replace("ROOM_CONTENT/", "").clone();
			proccess_protocol = proccess_protocol.trim().to_string();
			sender_tx.send(client.room_users(proccess_protocol.clone()));
		}else if type_protocol.starts_with("TX_ROOM/"){
			proccess_protocol = type_protocol.replace("TX_ROOM/", "").clone();
			let cut_protoc = proccess_protocol.find('/').unwrap_or(proccess_protocol.len());
			let mut room_name: String = proccess_protocol.drain(..cut_protoc).collect();
			room_name = room_name.trim().replace(" ", "");
			let mut room_text = proccess_protocol.replace("/", "");
			sender_tx.send(client.room_text(room_name.clone(), room_text.clone()));
		}
	}
}
pub mod responses{
	use super::*;
pub fn type_response(response: String){
	let json_response = serde_json::from_str(&response).unwrap();
	if let type_protocol::Type_protocol::RESPONSE{operation, result, extra} = json_response{
		if operation.to_string() == "IDENTFY".to_string(){
			println!(" -> was indentificate: {}", extra.to_string());
		}else if operation.to_string() == "ROOM_USERS".to_string() && result == type_protocol::ResultType::NOT_JOINED{
			eprintln!("-- you not have in this room --");
		}else if operation.to_string() == "NEW_ROOM".to_string() && result == type_protocol::ResultType::SUCCESS{
			println!("--------------------------------------------------------");
			println!("> Currently you created the -- {} -- room ", extra.to_string());
			println!("--------------------------------------------------------");
		}else if operation.to_string() == "JOIN_ROOM".to_string() && result == type_protocol::ResultType::SUCCESS{
			println!("--------------------------------------------------------");
			println!("> Currently you joined in the -- {} -- room ", extra.to_string());
			println!("--------------------------------------------------------");	
		}
	}
	else if let type_protocol::Type_protocol::NEW_USER{username} = json_response{
		println!("> {} was joined to the general chat", username.clone());
	}else if let type_protocol::Type_protocol::TEXT_FROM{ username, text} = json_response{
		println!("> FROM: {} --> {}", username.to_string(), text.to_string())
	}else if let type_protocol::Type_protocol::PUBLIC_TEXT_FROM{ username, text} = json_response{
		println!("> {}: {}", username.to_string(), text.to_string());
		
	}else if let type_protocol::Type_protocol::USER_LIST{ users} = json_response{
		println!("all theese clients ------> " );
		for (user, status) in users.iter(){
			println!("> {user} -> status: {status}");
		}
	}
	else if let type_protocol::Type_protocol::INVITATION{username, roomname} = json_response{
		println!("> You has an invitation from {} to join in the {} room ---- ", username.to_string(), roomname.to_string());
	}
	else if let type_protocol::Type_protocol::JOINED_ROOM{roomname, username} = json_response{
		println!("--------------------------------------------------------");
		println!("> Currently {} are in the -- {} -- room ", username,roomname);
		println!("--------------------------------------------------------");
	}
	else if let type_protocol::Type_protocol::ROOM_USER_LIST{ roomname, users} = json_response{
		println!("Current users in the -- {} -- room ->> ", roomname);
		for (user, status) in users{
			println!("ROOM -> username -- {} -- status: {}", user, status);
		}
	} else if let type_protocol::Type_protocol::ROOM_TEXT_FROM{roomname, username, text} = json_response{
		println!("> {}: {} -> {}", roomname, username, text);
	}
	}
}
