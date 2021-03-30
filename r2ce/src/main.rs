use std::io;

mod pulse_worker;
mod socket_worker;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    println!("starting...");

    pulse_worker::start();
    socket_worker::start();

    // wait until exit
    println!("running. awaiting for input to exit.");
    io::stdin().read_line(&mut String::new()).unwrap();
}
