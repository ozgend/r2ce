mod bus;
mod pulse_worker;
mod socket_worker;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    println!("starting...");

    let bus: &bus::Bus = &bus::Bus {
        on_signal: event_callback,
    };

    // bus.send("a1", bus::BUS_SIGNAL_1);
    // bus.send("a1", bus::BUS_SIGNAL_2);
    // bus.send("a1", bus::BUS_SIGNAL_3);
    // bus.send("a1", bus::BUS_SIGNAL_4);
    // bus.send_async("b1", bus::BUS_SIGNAL_1);
    // bus.send_async("b1", bus::BUS_SIGNAL_2);
    // bus.send_async("b1", bus::BUS_SIGNAL_3);
    // bus.send_async("b1", bus::BUS_SIGNAL_4);

    pulse_worker::start(&bus);
    socket_worker::start();

    // wait until exit
    println!("running. awaiting for input to exit.");
    std::io::stdin().read_line(&mut String::new()).unwrap();
}

fn event_callback(source: String, signal: i16) {
    println!("event_callback.signal > {}: {}", source, signal);
}
