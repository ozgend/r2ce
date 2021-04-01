use embedded_websocket::{
    framer::{Framer, FramerError},
    WebSocketClient, WebSocketOptions, WebSocketSendMessageType,
};

use std::net::TcpStream;
use tokio::{sync::mpsc, time};

const SOCKET_URL: &str = "127.0.0.1:6022";

pub(crate) fn init(tx: tokio::sync::mpsc::UnboundedSender<String>) {
    println!("+ socket_worker::init");
    tx.send("SOCKET.init".to_string()).unwrap();
}

async fn run_socket_async(tx: mpsc::UnboundedSender<String>) -> Result<(), FramerError> {
    println!("+  socket_worker::run_socket_async");

    let mut read_buf: [u8; 4000] = [0; 4000];
    let mut write_buf: [u8; 4000] = [0; 4000];
    let mut frame_buf: [u8; 4000] = [0; 4000];
    let mut ws_client = WebSocketClient::new_client(rand::thread_rng());

    let websocket_options = WebSocketOptions {
        path: "/r2ce",
        host: "localhost",
        origin: "http://127.0.0.1:6022",
        sub_protocols: None,
        additional_headers: None,
    };

    let mut stream = TcpStream::connect(SOCKET_URL)?;
    let mut websocket = Framer::new(&mut read_buf, &mut write_buf, &mut ws_client, &mut stream);
    websocket.connect(&websocket_options)?;

    tx.send("SOCKET.STARTED".to_string()).unwrap();

    let message = "iam-r2ce-client!";
    websocket.write(WebSocketSendMessageType::Text, true, message.as_bytes())?;

    while let Some(s) = websocket.read_text(&mut frame_buf)? {
        println!("Received: {}", s);
        tx.send("SOCKET.RECEIVED".to_string()).unwrap();
    }

    // handle_socket_tx(&mut websocket, &mut frame_buf)?;

    println!("Connection closed");
    Ok(())
}

// fn handle_socket_tx(
//     websocket: &mut Framer<rand::prelude::ThreadRng, embedded_websocket::Client, TcpStream>,
//     frame_buf: &mut [u8],
// ) -> Result<(), FramerError> {
//     while let Some(s) = websocket.read_text(frame_buf)? {
//         println!("Received: {}", s);

//         // close the websocket after receiving the first reply
//         // websocket.close(WebSocketCloseStatusCode::NormalClosure, None)?;
//         // println!("Sent close handshake");
//     }
//     Ok(())
// }
