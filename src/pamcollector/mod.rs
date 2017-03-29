use std::sync::mpsc::{sync_channel, SyncSender, Receiver};
use std::sync::{Arc, Mutex};
use std::error::Error;
mod config;
mod input;
mod output;
mod metric;

use self::config::Config;
use self::input::Input;
use self::output::Output;
use self::input::UdpInput;
use self::output::ConsoleOutput;


pub fn start(config_path: &str) {
    let config = match Config::from_path(&config_path) {
        Ok(config) => config,
        Err(e) => {println!("Fail to read config {}. {}", config_path, e.description());::std::process::exit(1)}
    };
    let input_transport = UdpInput::new(&config);
    let output_transport = ConsoleOutput::new(&config);
    let queue_size = 10_000_000;
    let (tx, rx): (SyncSender<Vec<u8>>, Receiver<Vec<u8>>) = sync_channel(queue_size);
    let arx = Arc::new(Mutex::new(rx));
    output_transport.start(arx);
    input_transport.accept(tx);
}
