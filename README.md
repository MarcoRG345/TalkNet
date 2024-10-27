## A real time chat client/server
# Description
<Author: Marco Antonio Raya Garcia>
<Mail: tonyrayagarcia@gmail.com>
Implementation real time chat aplication Client/Server. This project is interactive, using concepts of concurrency and threads this could be possible.
The implementation consists in two big parts and one most important. The server, the object server has two threads for manage multiple clients that connect 
with it through <sockets> and request-response at the same time. A lot of computers can connect with a main computer that run the server connection, only ensure the computers,
these computers are the clients and they can chat with the rest of the clients, can make rooms, send private messages, get a room list or general list and more features that you 
will found in the `protocol.txt` file.
# Protocol
The stream protocol it was worked with JSON,it should be noted that a lot of lenguages work with libraries to serialize and deserialize data in a JSON format.
Rust it does not the exception, this project was implemented with 100% Rust lenguage with this tecnologies: Tokio, serde and serde_json, uuid.
# Use
Clone this repository:
Install `cargo` and rust to run directly from the main code
 * `git clone https://github.com/MarcoRG345/TalkNet.git` 
 * `cargo run --bin server` or `cargo --bin client`.
If you prefer no to do it, you can try download the `tar.gz` files, descompress it and execute ./server or ./client. In both cases they will require an IPv4 address, the case
of the client, ensure that you are in the same network connection and the IPv4 address is the correct server connection that decided to listening there.
# How the client interact
if you decided to clone all,it will ask you your name if the client detect a connection with the server, you can follow this to interact:<br>
  *(>) Send a general message to the chat PUBLIC_TEXT<br>.
  *(all/) Gives you a list of the users in the chat USERS<br>.
  *(txfrom/username/... ordinary text) send a private message for someone in the chat TEXT<br>.
  *(room/roomname) create a new room with the name specifed NEW_ROOM<br>.
  *(INVITE/ <users> <roomname>) send an invitation for multiple users with the room name INVITE<br>.
  *(JOIN/roomname) join to the room that your were invited JOIN_ROOM<br>.
  *(ROOM_CONTENT/roomname) gives you a user list in the current room specifed<br>.
  *(TX_ROOM/roomname/..ordinary text) send a room message ROOM_TEXT. 
