use serde_json::Value;
use std::io::{BufReader, Read, Write};
use std::net::{Ipv4Addr, SocketAddrV4, TcpStream};
pub struct Connection {
    stream: TcpStream,
}
impl Connection {
    pub fn new(stream: TcpStream) -> Self {
        Connection { stream }
    }
    //recibir peticion en bytes y proporciona el json.
    pub fn recv_from(&self, mut stream: &TcpStream) {
        let mut reader: Vec<u8> = Vec::new();
        if let Ok(v) = stream.read_to_end(&mut reader) {
            let data: u8 = v as u8;
            let data_slice = std::slice::from_ref(&data);
            let data_json = std::str::from_utf8(data_slice).unwrap();
        }
    }
    pub fn get_stream(&self) -> &TcpStream {
        &self.stream
    }
}
