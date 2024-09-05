mod server;
use std::sync::{Arc, Mutex};
use std::thread;
fn main() {
    println!("Hello! from the server");
    let server = server::Server::new();
    server.start_server();
}
