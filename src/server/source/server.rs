use common::user::User;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct Server {
    users: HashMap<User, String>,
}

impl Server {
    fn new(&self) -> Self {
        let mut users_name_satus: HashMap<User, String> = HashMap::new();
        Server {
            users: users_name_satus,
        }
    }
    fn start(&self) {}
}
