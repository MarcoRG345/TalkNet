use serde_json::Value;
use tokio::io::{self, AsyncWrite, AsyncRead, AsyncBufReadExt, AsyncWriteExt, ReadHalf, WriteHalf};
use tokio::net::TcpStream;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::io::AsyncReadExt;
use crate::types_msg::Types_msg;
use serde_json::json;


/**
* Para generar una comuniacion entre cliente y servidor.
* La clase es instanciada por un cliente que abre su comunicacion con el servidor.
 */
#[derive(Clone)]
pub struct Connection {
	reader: Arc<Mutex<ReadHalf<TcpStream>>>,
	writer: Arc<Mutex<WriteHalf<TcpStream>>>,
}
impl Connection{
	
}

