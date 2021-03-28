use minreq::{post};
use rust_socketio::{Payload, Socket, SocketBuilder};
use serde_json::json;
use std::{
    collections::HashMap,
    env,
    io::{Result},
};
use tokio::time;

const PULSE_INTERVAL: u64 = 5;
const PULSE_URL: &str = "http://localhost:6022/pulse";
const SOCKET_URL: &str = "http://localhost:6022/socket";

pub fn start_pulse() {
    println!("+ starting pulse");
    tokio::spawn(async { send_pulse_async(PULSE_INTERVAL).await });
}

async fn send_pulse_async(interval_seconds: u64) -> Result<()> {
    let mut interval = time::interval(time::Duration::from_secs(interval_seconds));

    loop {
        let payload_env: HashMap<String, String> = env::vars().collect();
        let payload = json!(payload_env).to_string();
        let send_request = post(PULSE_URL)
            .with_header("Content-Type", "application/json")
            .with_body(payload)
            .send();

        if send_request.is_err() {
            println!("pulse sent failed - will retry");
        } else {
            let response = send_request.unwrap();

            print!("pulse sent - status:{} ", response.status_code);

            match response.status_code {
                204 => {
                    println!("ok")
                }
                201 => {
                    let data = response.as_str().unwrap();
                    println!("with response: {}", data);
                    evaluate_pulse_command_async(data).await;
                }
                _ => {
                    println!("with error: {}", response.reason_phrase);
                }
            }
        }

        interval.tick().await;
    }
}

async fn evaluate_pulse_command_async(command: &str) -> () {
    match command {
        "connect" => start_socket(),
        _ => {
            println!("- unmatching pulse command: {}", command)
        }
    }
}

fn start_socket() {
    println!("+ starting socket");
    tokio::spawn(async { setup_socket_async().await });
}

async fn setup_socket_async() {
    println!("+ starting socket");

    let socket_callback = |payload: Payload, _socket: Socket| match payload {
        Payload::String(str) => println!("Received: {}", str),
        Payload::Binary(bin_data) => println!("Received bytes: {:#?}", bin_data),
    };

    let mut socket: Socket;

    socket = SocketBuilder::new(SOCKET_URL)
        // .set_namespace("/")
        // .expect("illegal namespace")
        .on("command", socket_callback)
        .on("error", |err, _| eprintln!("Error: {:#?}", err))
        .connect()
        .expect("Connection failed");

    socket.emit("setActive", "DOPC").expect("setActive failed");
}
