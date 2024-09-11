use tokio::io::{self,BufReader,AsyncBufReadExt,AsyncReadExt, AsyncWriteExt, stdout};
use tokio::net::TcpStream;
use tokio::sync::{mpsc, Mutex};
mod client;

#[tokio::main]
async fn main() {
	let mut stream = TcpStream::connect("127.0.0.1:8001").await.unwrap();
	let (mut rd, mut writer) = io::split(stream);
	let (sender_tx, mut receiver_rx) = mpsc::unbounded_channel::<String>();
	let (server_tx, mut server_rx) = mpsc::unbounded_channel::<String>();
	let mut  register = false;
	//while receiver_rx en mi propio canal se lo envio
	//simula que llego del servidor.
	tokio::spawn(async move {
		loop{
			if let Some(msg) = receiver_rx.recv().await{
				writer.write_all(msg.as_bytes()).await;
			}
		}
	});
	//thread to incoming messages
	tokio::spawn(async move {
		loop {
			let mut buffer = [0; 1024];
			match rd.read(&mut buffer).await {
				Ok(n) => {
					let response = String::from_utf8_lossy(&buffer[..n]);
					//println!("{}", response);
					if !response.is_empty(){
						println!("> {}", response);
						tokio::io::stdout().flush().await;
					}		
				},
				Ok(0) => return,
				Err(err) => return,
			}
		}
	});
	print!("ENTER username: ");
	tokio::io::stdout().flush().await;
	let mut stdin = io::stdin();
	let mut reader = BufReader::new(stdin);
	let mut buffer = String::new();

	reader.read_line(&mut buffer).await;
	let username = buffer.trim().to_string();
	let mut client = client::Client::new(username);
	sender_tx.send(client.send_identify());
	
	loop{
		buffer.clear();
		tokio::io::stdout().flush().await;
		reader.read_line(&mut buffer).await;
		let mut type_protocol = buffer.trim();
		let mut proccess_protocol = String::new();
		//Checa las diferentes formas de comunicarte en el chat.
		if type_protocol.starts_with(">"){
			proccess_protocol = type_protocol.replace(">", "").clone();
			sender_tx.send(client.send_pub_text(&mut proccess_protocol));
		}else if type_protocol.starts_with("all/"){
			sender_tx.send(client.request_users());
		}
	}
}
