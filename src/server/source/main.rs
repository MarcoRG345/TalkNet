mod server;
use std::sync::{Arc};
use tokio::io::{self, AsyncWriteExt, AsyncReadExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{mpsc, Mutex};
use uuid::Uuid;
use std::io::Write;
mod room;  
#[tokio::main]
async fn main() {
	start_server().await;
}

/// Start the behaviors server. The function recieve a valid IPv4 address to bind through
/// the port. The sense to build a server is listening always the request from the clients
/// the reason to have a indeterminate loop.
///
/// The loop consists with two threads, one of them when a new connection was joined, new thread
/// is created to listen the client requests. But at the same time send the a response if the
/// server detect a request.

async fn start_server(){
	let mut listener_option: Option<TcpListener> = None;
	
	while let None = listener_option{
		print!("ENTER the IPv4 address and PORT (example 127.0.0.1:8001): ");
		std::io::stdout().flush();
		let mut buffer = String::new();
		std::io::stdin().read_line(&mut buffer).unwrap();
		let ipv4addr = buffer.trim().to_string();
	
		listener_option = match TcpListener::bind(&ipv4addr).await{
			Ok(bind) => Some(bind),
			Err(_) => {eprintln!("Try again with a valid IPv4 address"); None},
		};
	}
	
	println!("Server started...");
	let listener = listener_option.unwrap();
	let server = Arc::new(Mutex::new(server::Server::new()));
	
	loop{
		let (mut stream, addr) = listener.accept().await.unwrap();
		println!("New client connected");
		let (mut reader, mut writer) = io::split(stream);
		let (sender_tx, mut reciver_rx) = mpsc::unbounded_channel::<String>();
		let share_server = Arc::clone(&server);
		let share_server_conn = Arc::clone(&server);
	
		// Publish the client responses. write all messeage in the writer socket port.
		tokio::spawn(async move {
			loop {
				if let Some(message) = reciver_rx.recv().await{
					writer.write_all(message.as_bytes()).await;
				}
			}
		});

		// Listen the client answer.
		tokio::spawn(async move{
			let mut buffer = [0; 1024];
			let mut suscribe = false;
			let auth = Uuid::new_v4();
			let mut has_auth = false;
			let mut id = String::new();
			loop{				
				match reader.read(&mut buffer).await{
					Ok(0) => return,
					Ok(n) => {
						// Sucribe a new cient with the first word joined.
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
							// Detect a new client to sucribe him/her with a unique uuid.
							if !has_auth{
								let unlocked_server = share_server.lock().await;
								unlocked_server.suscribe_auth(auth.to_string(), id.to_string()).await;
								has_auth = true;
							}
							// publish through the sender a response message, send to the reciever channel to write through the socket.
							share_server_conn.lock().await.publish(auth.to_string(), input.to_string(), sender_tx.clone()).await;
						}
					},
					Err(e) => return,
				}
			}
		});
		
	}
}
