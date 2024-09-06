use common::connection::Connection;
use common::user::User;
use serde_json::{json, Value};
use std::io::*;
use std::net::TcpStream;
mod client;
fn main() {
    println!("ENTER msg: ");
    let mut msg: String = String::new();
    stdin().read_line(&mut msg);
    let connection = Connection::new();
    let client = client::Client::new(connection);
    /*let user = User::new(
        &serde_json::to_value(&msg).unwrap(),
        serde_json::to_value(String::from("AWAY")).unwrap(),
    );*/
    let serialize = json![{
        "type": "IDENTIFY",
        "username": msg.trim()
    }];
    let mut msg = serialize.to_string();
    let request = format!("{}\n", msg.to_string());
    client.send_to_connection(&request);
    let response = client.recv_from_connection();
    println!("{}", response.to_string());
}
