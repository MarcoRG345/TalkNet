use serde_json::Value;
use std::io::{BufReader, Read, Write};
use std::net::{Ipv4Addr, SocketAddrV4, TcpStream};
pub struct Connection {
    stream: TcpStream,
}
impl Connection {
    pub fn new() -> Self {
        let mut stream = TcpStream::connect("127.0.0.1:8001").unwrap();
        Connection { stream }
    }
    /**
    	* This method will recieve from other stream and returns the data
    	* in a JSON representation.
    	*/
    pub fn recv_from(&self, mut stream: &TcpStream) -> String {
        let mut buffer: String = String::new();
        let mut reader = BufReader::new(&mut stream);
        reader.read_to_string(&mut buffer);
        buffer
    }
    pub fn send_to(&self, data: &String, mut stream: &TcpStream) {
        let mut data_bytes = data.as_bytes();
        stream.write_all(&data_bytes);
    }
    pub fn get_stream(&self) -> &TcpStream {
        &self.stream
    }
}
