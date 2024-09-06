use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::hash::Hash;

#[derive(Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct User {
    id: Value,
    status: Value,
}
impl User {
    pub fn new(id: &Value, status: Value) -> Self {
        User {
            id: id.clone(),
            status,
        }
    }

    pub fn get_id(&self) -> &Value {
        &self.id
    }
    pub fn get_status(&self) -> &Value {
        &self.status
    }
    pub fn set_status(&mut self, status: &String) {
        self.status = serde_json::to_value(status.to_string()).unwrap();
    }
    pub fn parse_me(&self, data: &str) -> User {
        let u: User = serde_json::from_str(data).unwrap();
        u
    }
}
