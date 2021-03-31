pub const BUS_SIGNAL_1: i16 = 1001;
pub const BUS_SIGNAL_2: i16 = 1002;
pub const BUS_SIGNAL_3: i16 = 1003;
pub const BUS_SIGNAL_4: i16 = 1004;

pub struct Bus {
    pub on_signal: fn(source: String, signal: i16),
}

impl Bus {
    pub fn send(&mut self, source: &str, signal: i16) {
        let _on_signal = self.on_signal;
        _on_signal(source.to_string(), signal);
    }

    pub fn send_async(&mut self, source: &str, signal: i16) {
        let _on_signal = self.on_signal;
        let _source = source.to_string();
        tokio::spawn(async move {
            _on_signal(_source, signal);
        });
    }
}
