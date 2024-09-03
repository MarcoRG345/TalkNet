pub mod common;
use crate::common::user::User;
fn main() {
    println!("Hello, world!");
    let id = String::from("Marco");
    let status = String::from("AWAY");
    let mut u = User::new(&id, &status);
    println!("{}", u.get_status());
    let new_status = String::from("BUSY");
    u.set_status(&new_status);
    println!("{}", u.get_status());
}
