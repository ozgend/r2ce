use std::{collections::HashMap, env, net::TcpStream};

use embedded_websocket::{
    framer::{Framer, FramerError},
    WebSocketClient, WebSocketOptions, WebSocketSendMessageType,
};
use serde_json::json;
use tokio::{sync::mpsc::UnboundedSender, time};

use crate::eval_command;

const PROTO: &str = "http";
const SOCKET_URL: &str = "127.0.0.1:6022";

pub(crate) fn init(tx: UnboundedSender<String>) {
    println!("[{}] {}", "socket", "init");
    tx.send("SOCKET.init".to_string()).unwrap();
    tokio::spawn(async move {
        let mut interval = time::interval(time::Duration::from_secs(5));
        loop {
            let socket_run_result = run_socket_async(tx.clone()).await;
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

async fn run_socket_async(tx: UnboundedSender<String>) -> Result<(), FramerError> {
    println!("[{}] {}", "socket", "run");

    let mut read_buf: [u8; 4000] = [0; 4000];
    let mut write_buf: [u8; 4000] = [0; 4000];
    let mut frame_buf: [u8; 4000] = [0; 4000];
    let mut socket_id = "non-id".to_string();

    let mut ws_client = WebSocketClient::new_client(rand::thread_rng());
    let origin = format!("{}://{}", PROTO, SOCKET_URL).to_owned();

    let websocket_options = WebSocketOptions {
        path: "/r2ce",
        host: "localhost",
        origin: origin.as_str(),
        sub_protocols: None,
        additional_headers: None,
    };

    let mut stream = TcpStream::connect(SOCKET_URL)?;
    let mut websocket = Framer::new(&mut read_buf, &mut write_buf, &mut ws_client, &mut stream);
    websocket.connect(&websocket_options)?;

    tx.send("SOCKET.STARTED".to_string()).unwrap();

    // send join message
    let env_vars: HashMap<String, String> = env::vars().collect();
    let hostname = &env_vars["COMPUTERNAME"];
    let message = format!("join<<<server<<<{}@{}", hostname, std::process::id());
    websocket.write(WebSocketSendMessageType::Text, true, message.as_bytes())?;

    while let Some(data) = websocket.read_text(&mut frame_buf)? {
        // todo data packets <<<>>>
        println!("[{}] {} {}", "socket", "received: ", data);
        tx.send("SOCKET.RECEIVED".to_string()).unwrap();

        let args: Vec<&str> = data.split("<<<").collect();
        let action = args[0];
        let payload = args[2];

        match action {
            "id" => {
                socket_id = payload.to_string();
            }
            "forward" => {
                let result = eval_command::evaluate_command(payload);
                let response = json!(result).to_string();
                let message = format!("respond<<<{}<<<{}", socket_id, response).to_owned();
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
