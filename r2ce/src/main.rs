use tokio::runtime::Runtime;

mod pulse_worker;

fn main() {
    let rt = Runtime::new().unwrap();

    println!("starting rt.block_on");

    rt.block_on(async move {
        println!("starting async block");
        pulse_worker::start_pulse();
    });
    loop {}
}
