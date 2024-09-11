use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::collections::HashMap;


#[derive(Serialize, Deserialize)]
pub enum ResultType{
	SUCCESS,
	USER_ALREADY_EXISTS,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Type_protocol{
	RESPONSE {request: String, result: ResultType, extra: String},
	NEW_USER {username: String},
	USER_LIST{users: HashMap<String, String>},	
}
