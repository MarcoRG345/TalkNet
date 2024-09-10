use common::connection::Connection;
use std::net::TcpStream;
pub struct Client {
    /* Connection client/server. The TcpStream here is from him.  */
    connection: Connection,
}
impl Client {
    pub fn new(connection: Connection) -> Self {
        let stream = TcpStream::connect("127.0.0.1:8001").unwrap();
        Client { connection }
    }
}
