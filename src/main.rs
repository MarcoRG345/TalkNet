mod common;
use crate::common::user::User;
fn main() {
    println!("Hello, world!");
    let cadena = String::from("Marco");
    let estado = String::from("AWAY");
    let mut u = User::new(&cadena, &estado);
    println!("{}", u.get_status());
    let nuevo_estado = String::from("BUSY");
    u.set_status(&nuevo_estado);
    println!("{}", u.get_status());
}
