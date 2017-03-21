use std::error::Error;
use std::sync::mpsc::{sync_channel, SyncSender, Receiver};
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use std::sync::RwLock;
use self::input::UdpInput;
mod input;
use pamcollector::input::Input;
pub fn start() {
    let input_transport = UdpInput::new();
    let queue_size = 10_000_000;
    let (tx, rx): (SyncSender<Vec<u8>>, Receiver<Vec<u8>>) = sync_channel(queue_size);
    let arx = Arc::new(Mutex::new(rx));
    input_transport.accept(tx);
}
