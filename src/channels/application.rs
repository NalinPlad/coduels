use loco_rs::socketioxide::{
    extract::{AckSender, Bin, Data, SocketRef, State},
    // SocketIo,
};

use serde_json::{Value};
use super::state;
use tracing::info;
use uuid::Uuid;

// use super::state::SessionStore;

pub fn on_connect(socket: SocketRef, Data(data): Data<Value>) {
    info!("Socket.IO connected: {:?} {:?}", socket.ns(), socket.id);

    // info!("Sending auth data? heres the contents: {}", &data);
    // socket.emit("auth", data).ok();

    socket.on(
        "message",
        |socket: SocketRef, Data::<Value>(data), Bin(bin)| {
            info!("Received event: {:?} {:?}", data, bin);
            socket.bin(bin).emit("message-back", data).ok();
        },
    );

    socket.on(
        "create",
        |socket: SocketRef, Data::<Value>(data)| {
            info!("Received event: {:?}", data);

            // let action = data["action"].;
            dbg!(&data);

            // Create a new room
            socket.emit("message-back", Uuid::new_v4()).ok();
            
        },
    );

    // socket.on(
    //     "join",
    //     |socket: SocketRef, Data::<String>(room)| async move{
    //         info!("Received event: {:?}", room);

    //         // let action = data["action"].;
    //         dbg!(&data);

    //         // Create a new room
    //         socket.emit("message-back", Uuid::new_v4()).ok();
            
    //     },
    // );

    socket.on(
        "join", 
        |socket: SocketRef, Data::<String>(room), store: State<state::SessionStore>| async move {
            info!("Received event: {:?}", room);
            // check if room exists
            if store.sessions.read().await.contains_key(&room) {
                socket.emit("message-back", "Yes, that room exists. Cool").ok();
            }
        },
    );


    socket.on(
        "message-with-ack",
        |Data::<Value>(data), ack: AckSender, Bin(bin)| {
            info!("Received event: {:?} {:?}", data, bin);
            ack.bin(bin).send(data).ok();
        },
    );
}