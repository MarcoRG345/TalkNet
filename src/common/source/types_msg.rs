use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Types_msg {
    IDENTIFY {username: String},
    STATUS { status: String},
    USERS,
    TEXT {username: String, text: String},
    PUBLIC_TEXT { text: String},
    NEW_ROOM {roomname: String},
    INVITE {roomname: String, usernames: Vec<String>},
    JOIN_ROOM{roomname: String},
    ROOM_USERS {roomname: String},
    ROOM_TEXT {roomname: String, text: String},
    LEAVE_ROOM,
    DISCONNECT,
}
