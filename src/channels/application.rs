use loco_rs::socketioxide::{
    extract::{AckSender, Bin, Data, SocketRef},
    // SocketIo,
};

use serde_json::Value;
use tracing::info;
use uuid::Uuid;
pub fn on_connect(socket: SocketRef, Data(data): Data<Value>) {
    info!("Socket.IO connected: {:?} {:?}", socket.ns(), socket.id);

    // info!("Sending auth data? heres the contents: {}", &data);
    socket.emit("auth", data).ok();

    socket.on(
        "message",
        |socket: SocketRef, Data::<Value>(data), Bin(bin)| {
            info!("Received event: {:?} {:?}", data, bin);
            socket.bin(bin).emit("message-back", data).ok();
        },
    );

    socket.on(
        "room",
        |socket: SocketRef, Data::<Value>(data), Bin(bin)| {
            info!("Received event: {:?} {:?}", data, bin);

            // Create a new room
            if data == "create" {
                
                socket.bin(bin).emit("message-back", Uuid::new_v4()).ok();
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