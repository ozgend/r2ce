use std::io;

mod pulse_worker;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    run().await;
    io::stdin().read_line(&mut String::new()).unwrap();
}

async fn run() -> () {
    println!("running ...");
    pulse_worker::start_pulse();
}
