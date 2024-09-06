use common::connection::Connection;
use std::net::TcpStream;
pub struct Client {
    /* Connection client/server. The TcpStream here is from him.  */
    connection: Connection,
    /* New TcpStream to connect with the server. The TcpStream here is from the server. */
    stream: TcpStream,
}
impl Client {
    pub fn new(connection: Connection) -> Self {
        let stream = TcpStream::connect("127.0.0.1:8001").unwrap();
        Client { connection, stream }
    }
    pub fn send_to_connection(&self, request: &String) {
        self.connection.send_to(request, &self.stream)
    }
    pub fn recv_from_connection(&self) -> String {
        self.connection.recv_from(self.connection.get_stream())
    }
}
