use tokio::sync::mpsc::unbounded_channel;

mod pulse_worker;
mod socket_worker;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    println!("[{}] {}", "main", "starting...");

    let (tx, mut rx) = unbounded_channel::<String>();

    tokio::spawn(async move {
        println!("[{}] {}", "bus.rx", "init");

        while let Some(signal) = rx.recv().await {
            println!("[{}] {} {}", "bus.rx", "signal", signal);
        }
    });

    let tx1 = tx.clone();
    let tx2 = tx.clone();

    pulse_worker::init(tx1);
    socket_worker::init(tx2);

    // wait until exit
    println!("[{}] {}", "main", "running");
    std::io::stdin().read_line(&mut String::new()).unwrap();
    drop(tx);
    println!("[{}] {}", "main", "stoppped.");
}
