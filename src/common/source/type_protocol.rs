use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::collections::HashMap;


#[derive(Serialize, Deserialize, PartialEq)]
pub enum ResultType{
	SUCCESS,
	USER_ALREADY_EXISTS,
	NO_SUCH_USER,
	ROOM_ALREADY_EXISTS,
	NO_SUCH_ROOM,
	NOT_INVITED,
}

#[derive(Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum Type_protocol{
	RESPONSE {operation: String, result: ResultType, extra: String},
	NEW_USER {username: String},
	USER_LIST{users: HashMap<String, String>},
	PUBLIC_TEXT_FROM {username: String, text: String},
	TEXT_FROM {username: String, text: String},
	INVITATION {username: String, roomname: String},
	JOINED_ROOM{roomname: String, username: String},
}
