mod server;
use std::sync::{Arc};
use tokio::io::{self, AsyncWriteExt, AsyncReadExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{mpsc, Mutex};
use uuid::Uuid;
use std::io::Write;
mod room; // Declarar el módulo `room`
//use room::Room; 
#[tokio::main]
async fn main() {
    println!("Server started...");
	start_server().await;
}

async fn start_server(){
	print!("ENTER the IPv4 address and PORT (example 127.0.0.1:8001): ");
	std::io::stdout().flush();
	let mut buffer = String::new();
	std::io::stdin().read_line(&mut buffer).unwrap();
	let ipv4addr = buffer.trim().to_string();
	let listener = TcpListener::bind(&ipv4addr).await.unwrap();
	let server = Arc::new(Mutex::new(server::Server::new()));
	loop{
		let (mut stream, addr) = listener.accept().await.unwrap();
		println!("cliente conectado");
		let (mut reader, mut writer) = io::split(stream);
		let (sender_tx, mut reciver_rx) = mpsc::unbounded_channel::<String>();
		let share_server = Arc::clone(&server);
		let share_server_conn = Arc::clone(&server);
	
		//Escucha la respuesta del cliente.
		tokio::spawn(async move {
			loop {
				if let Some(message) = reciver_rx.recv().await{
					writer.write_all(message.as_bytes()).await;
				}
			}
		});

		//Publica tus respuestas a tus suscriptores.
		tokio::spawn(async move{
			let mut buffer = [0; 1024];
			let mut suscribe = false;
			let auth = Uuid::new_v4();
			let mut has_auth = false;
			let mut  id = String::new();
			loop{				
				match reader.read(&mut buffer).await{
					Ok(0) => return,
					Ok(n) => {
						if !suscribe {
							let request_id = String::from_utf8_lossy(&buffer[..n]);
							let unlocked_server = share_server.lock().await;
							unlocked_server.suscribe(request_id.clone().to_string(), sender_tx.clone()).await;
							
							unlocked_server.response_indentify(request_id.clone().to_string()).await;
							suscribe = true;
							id = request_id.clone().to_string();
						}
						else{
							let input = String::from_utf8_lossy(&buffer[..n]);
							if !has_auth{
								let unlocked_server = share_server.lock().await;
								unlocked_server.suscribe_auth(auth.to_string(), id.to_string()).await;
								has_auth = true;
							}
							share_server_conn.lock().await.publish(auth.to_string(), input.to_string(), sender_tx.clone()).await;
						}
					},
					Err(e) => return,
				}
			}
		});
		
	}
}
