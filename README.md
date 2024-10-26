## A real time chat client/server
# Description
<Author: Marco Antonio Raya Garcia>
<Mail: tonyrayagarcia@gmail.com>
Implementation real time chat aplication Client/Server. This project is interactive, using concepts of concurrency and threads this could be possible.
The implementation consists in two big parts and one most important. The server, the object server has two threads for manage multiple clients that connect 
with it through <sockets> and request-response at the same time.
# Protocol
The stream protocol it was worked with JSON,it should be noted that a lot of lenguages work with libraries to serialize and deserialize data in a JSON format.
Rust it does not the exception, this project was implemented with 100% Rust lenguage with this tecnologies: Tokio, serde and serde_json, uuid.
# How to run
'''Clone this repository:
git clone https://github.com/MarcoRG345/TalkNet.git
cargo run --bin server or cargo --bin client
./target/debug/server ./target/debug/client
# How the client interact
if you decided to clone all, and ./target/debug/client it will ask you your name if the client detect a connection with the server, you can follow this to interact
  *(>) Send a general message to the chat PUBLIC_TEXT.
  *(all/) Gives you a list of the users in the chat USERS.
  *(txfrom/username/... ordinary text) send a private message for someone in the chat TEXT.
  *(room/roomname) create a new room with the name specifed NEW_ROOM.
  *(INVITE/ <users> <roomname>) send an invitation for multiple users with the room name INVITE.
  *(JOIN/roomname) join to the room that your were invited JOIN_ROOM.
  *(ROOM_CONTENT/roomname) gives you a user list in the current room specifed.
  *(TX_ROOM/roomname/..ordinary text) send a room message ROOM_TEXT. 
