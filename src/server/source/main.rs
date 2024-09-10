mod server;
use std::sync::{Arc};
use tokio::io::{self, AsyncWriteExt, AsyncReadExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{mpsc, Mutex};
#[tokio::main]
async fn main() {
    println!("Server started...");
	start_server().await;
}

async fn start_server(){
	let listener = TcpListener::bind("127.0.0.1:8001").await.unwrap();
	let server = Arc::new(Mutex::new(server::Server::new()));
	loop{
		let (mut stream, addr) = listener.accept().await.unwrap();
		println!("paso esta prueba");
		let (mut reader, mut writer) = io::split(stream);
		let (sender_tx, mut reciver_rx) = mpsc::unbounded_channel::<String>();
		let share_server = Arc::clone(&server);
		let share_server_conn = Arc::clone(&server);

		{
			let mut buf = [0;1024];
			match reader.read(&mut buf).await{
				Ok(n) => {
					let username = String::from_utf8_lossy(&buf[..n]);
					share_server.lock().await.suscribe(username.to_string(), sender_tx.clone()).await;
					println!("suscribed");		
				},
				Ok(0) => return,
				Err(err) => return,
			}
		}
		//Escucha la respuesta del cliente.
		tokio::spawn(async move {
			loop {
				if let Some(message) = reciver_rx.recv().await{
					println!("envio respuesta");
					writer.write_all(message.as_bytes()).await;
				}
			}
		});
		
		tokio::spawn(async move{
			let mut buffer = [0; 1024];
			loop{
				match reader.read(&mut buffer).await{
					Ok(0) => return,
					Ok(n) => {
						let input = String::from_utf8_lossy(&buffer[..n]);
						share_server_conn.lock().await.publish(input.to_string()).await;
						
					},
					Err(e) => return,
				}
			}
		});
		
	}
}
