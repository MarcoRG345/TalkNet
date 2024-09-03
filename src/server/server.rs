use crate::common::user::User;
use serde::{Deserialize, Serialize};
//use serde_json::Result;
use std::collections::HashMap;

pub struct Server {
    users: HashMap<User, String>,
    //optional String instead of Room instance.
    rooms: HashMap<User, String>,
}
