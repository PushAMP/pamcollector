use std::sync::mpsc::{sync_channel, SyncSender, Receiver};
use std::sync::{Arc, Mutex};

mod input;
mod output;
mod metric;

use self::input::Input;
use self::output::Output;
use self::input::UdpInput;
use self::output::ConsoleOutput;

pub fn start() {
    let input_transport = UdpInput::new();
    let output_transport = ConsoleOutput::new();
    let queue_size = 10_000_000;
    let (tx, rx): (SyncSender<Vec<u8>>, Receiver<Vec<u8>>) = sync_channel(queue_size);
    let arx = Arc::new(Mutex::new(rx));
    output_transport.start(arx);
    input_transport.accept(tx);
}
