pub struct User {
    id: String,
    status: String,
}
impl User {
    pub fn new(id: &String, status: &String) -> Self {
        User {
            id: id.to_string(),
            status: status.to_string(),
        }
    }

    pub fn get_id(&self) -> &String {
        &self.id
    }
    pub fn get_status(&self) -> &String {
        &self.status
    }
    pub fn set_status(&mut self, status: &String) {
        self.status = status.to_string();
    }
}
