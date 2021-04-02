use tokio::sync::mpsc::unbounded_channel;

mod eval_command;
mod pulse_worker;
mod socket_worker;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    println!("[{}] {}", "main", "starting...");
    let args: Vec<String> = std::env::args().collect();
    println!("{:?}", args);

    let (tx, _rx) = unbounded_channel::<String>();

    // tokio::spawn(async move {
    //     println!("[{}] {}", "bus.rx", "init");

    //     while let Some(signal) = rx.recv().await {
    //         println!("[{}] {} {}", "bus.rx", "signal", signal);
    //     }
    // });

    let tx1 = tx.clone();
    let tx2 = tx.clone();
    let host = args[1].clone();

    pulse_worker::init(tx1, host.clone());
    socket_worker::init(tx2, host.clone());

    // wait until exit
    println!("[{}] {}", "main", "running");
    std::io::stdin().read_line(&mut String::new()).unwrap();
    drop(tx);
    println!("[{}] {}", "main", "stoppped.");
}
