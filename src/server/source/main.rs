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
		let (mut reader, mut writer) = io::split(stream);
		let (sender_tx, mut reciver_rx) = mpsc::unbounded_channel::<String>();
		let share_server = Arc::clone(&server);
		let share_server_conn = Arc::clone(&server);
	
		//Escucha la respuesta del cliente.
		tokio::spawn(async move {
			loop {
				if let Some(message) = reciver_rx.recv().await{
					println!("envio respuesta");
					writer.write_all(message.as_bytes()).await;
				}
			}
		});

		//Publica tus respuestas a tus suscriptores.
		tokio::spawn(async move{
			let mut buffer = [0; 1024];
			let mut suscribe = false;
			loop{				
				match reader.read(&mut buffer).await{
					Ok(0) => return,
					Ok(n) => {
						if !suscribe {
							let request_id = String::from_utf8_lossy(&buffer[..n]);
							println!("{}", request_id.clone());
							let unlocked_server = share_server.lock().await;
							unlocked_server.suscribe(request_id.clone().to_string(), sender_tx.clone()).await;
							unlocked_server.response_indentify(request_id.clone().to_string()).await;
							println!("suscribed");
							suscribe = true;
						}else{
							let input = String::from_utf8_lossy(&buffer[..n]);
							share_server_conn.lock().await.publish(input.to_string(), sender_tx.clone()).await;
						}
					},
					Err(e) => return,
				}
			}
		});
		
	}
}
