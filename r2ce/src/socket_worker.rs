use embedded_websocket::{
    framer::{Framer, FramerError},
    WebSocketClient, WebSocketOptions, WebSocketSendMessageType,
};
use std::net::TcpStream;

const SOCKET_URL: &str = "127.0.0.1:6022";

pub fn start() {
    println!("+ starting socket");
    tokio::spawn(async { run_socket_async().await });
}

async fn run_socket_async() -> Result<(), FramerError> {
    // open a TCP stream to localhost port 1337
    let mut stream = TcpStream::connect(SOCKET_URL)?;
    println!("+  socket connected.");

    let mut read_buf: [u8; 4000] = [0; 4000];
    let mut write_buf: [u8; 4000] = [0; 4000];
    let mut frame_buf: [u8; 4000] = [0; 4000];
    let mut ws_client = WebSocketClient::new_client(rand::thread_rng());

    // initiate a websocket opening handshake
    let websocket_options = WebSocketOptions {
        path: "/r2ce",
        host: "localhost",
        origin: "http://127.0.0.1:6022",
        sub_protocols: None,
        additional_headers: None,
    };

    let mut websocket = Framer::new(&mut read_buf, &mut write_buf, &mut ws_client, &mut stream);
    websocket.connect(&websocket_options)?;

    let message = "iam-r2ce-client!";
    websocket.write(WebSocketSendMessageType::Text, true, message.as_bytes())?;

    handle_socket_tx(&mut websocket, &mut frame_buf)?;

    println!("Connection closed");
    Ok(())
}

fn handle_socket_tx(
    websocket: &mut Framer<rand::prelude::ThreadRng, embedded_websocket::Client, TcpStream>,
    frame_buf: &mut [u8],
) -> Result<(), FramerError> {
    while let Some(s) = websocket.read_text(frame_buf)? {
        println!("Received: {}", s);

        // close the websocket after receiving the first reply
        // websocket.close(WebSocketCloseStatusCode::NormalClosure, None)?;
        // println!("Sent close handshake");
    }
    Ok(())
}
