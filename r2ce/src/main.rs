use tokio::sync::mpsc::unbounded_channel;

mod pulse_worker;
mod socket_worker;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let (tx, mut rx) = unbounded_channel::<String>();

    tokio::spawn(async move {
        println!("rx.receive");

        while let Some(signal) = rx.recv().await {
            println!("tx.signal = {}", signal);
        }
    });

    println!("starting...");

    let tx1 = tx.clone();
    let tx2 = tx.clone();

    pulse_worker::init(tx1);
    socket_worker::init(tx2);

    // wait until exit
    println!("running. awaiting for input to exit.");
    std::io::stdin().read_line(&mut String::new()).unwrap();
}
