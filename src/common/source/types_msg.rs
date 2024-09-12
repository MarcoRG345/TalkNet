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
    NEW_ROOM,
    INVITE,
    JOIN_ROOM,
    ROOM_USERS,
    ROOM_TEXT,
    LEAVE_ROOM,
    DISCONNECT,
}
