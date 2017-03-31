use std::sync::mpsc::{sync_channel, SyncSender, Receiver};
use std::sync::{Arc, Mutex};
use std::error::Error;
mod config;
mod input;
mod output;
mod metric;
use std::thread;

use self::config::Config;
use self::input::Input;
use self::output::Output;
use self::input::{UdpInput, TcpInput};
use self::output::ClickHouseOutput;


pub fn start(config_path: &str) {
    let config = match Config::from_path(&config_path) {
        Ok(config) => config,
        Err(e) => {
            println!("Fail to read config {}. {}", config_path, e.description());
            ::std::process::exit(1)
        }
    };
    let input_udp = UdpInput::new(&config);
    let input_tcp = TcpInput::new(&config);
    let output_transport = ClickHouseOutput::new(&config);
    let output_transport1 = ClickHouseOutput::new(&config);
    let queue_size = 10_000_000;

    let (tx2, rx2): (SyncSender<Vec<u8>>, Receiver<Vec<u8>>) = sync_channel(queue_size);
    let arx2 = Arc::new(Mutex::new(rx2));
    output_transport1.start(arx2);
        thread::spawn(move || { input_tcp.accept(tx2); });
        let (tx1, rx1): (SyncSender<Vec<u8>>, Receiver<Vec<u8>>) = sync_channel(queue_size);
    let arx1 = Arc::new(Mutex::new(rx1));
    output_transport.start(arx1);
    input_udp.accept(tx1);
}
