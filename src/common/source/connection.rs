use serde_json::Value;
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::io::AsyncReadExt;


/**
* Para generar una comuniacion entre cliente y servidor.
* La clase es instanciada por un cliente que abre su comunicacion con el servidor.
 */
#[derive(Clone)]
pub struct Connection {
    stream: Arc<Mutex<TcpStream>>,
}
impl Connection {
    pub async fn new() -> io::Result<Self> {
        let stream = TcpStream::connect("127.0.0.1:8001").await?;
        Ok(Connection {
            stream: Arc::new(Mutex::new(stream)),
        })
    }

    pub async fn request(&self, request: String) -> io::Result<()> {
        let mut stream = self.stream.lock().await;
        stream.write_all(request.as_bytes()).await?;
        stream.flush().await?;
        Ok(())
    }

    pub async fn response(&self) -> io::Result<String> {
        let mut stream = self.stream.lock().await;
        let mut buffer = vec![0; 1024];
        let n = stream.read(&mut buffer).await?;
        let response = String::from_utf8_lossy(&buffer[..n]);
        Ok(response.to_string())
    }
}
