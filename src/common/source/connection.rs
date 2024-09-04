use std::net::{Ipv4Addr, SocketAddrV4};

pub struct Connection {
    socket: SocketAddrV4,
    reader: Vec<u32>,
    writer: Vec<u32>,
}
impl Connection {
    fn new() -> Self {
        let socket = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8000);
        let mut v: Vec<u32> = Vec::new();
        for i in 0..1024 {
            v.push(i);
        }
        Connection {
            socket,
            reader: v.clone(),
            writer: v,
        }
    }
}
