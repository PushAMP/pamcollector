use std::sync::mpsc::{sync_channel, Receiver, channel, Sender};
use std::sync::{Arc, Mutex};
use std::error::Error;
mod config;
// mod input;
mod output;
mod metric;
mod codec;
mod service;
mod protocol;
// use std::thread;

use self::config::Config;
// use self::input::Input;
use self::output::Output;
// use self::input::{UdpInput, TcpInput};
use self::output::ClickHouseOutput;
use self::codec::LineCodec;
use self::protocol::LineProto;
use self::service::Echo;
use tokio_proto::TcpServer;


pub fn start(config_path: &str) {
    let config = match Config::from_path(&config_path) {
        Ok(config) => config,
        Err(e) => {
            println!("Fail to read config {}. {}", config_path, e.description());
            ::std::process::exit(1)
        }
    };
    // let input_udp = UdpInput::new(&config);
    // let input_tcp = TcpInput::new(&config);
    let output_transport = ClickHouseOutput::new(&config);
    // let output_transport1 = ClickHouseOutput::new(&config);
    // let queue_size = 10_000_000;
    let addr = "0.0.0.0:12345".parse().unwrap();
    let server = TcpServer::new(LineProto, addr);
    let (tx, rx): (Sender<Vec<u8>>, Receiver<Vec<u8>>) = channel();
    let atx = Arc::new(Mutex::new(tx));
    let arx = Arc::new(Mutex::new(rx));
    output_transport.start(arx);
    server.serve(move || Ok(Echo {tx: atx.clone()}));
    // let (tx2, rx2): (SyncSender<Vec<u8>>, Receiver<Vec<u8>>) = sync_channel(queue_size);
    
    // thread::spawn(move || { input_tcp.accept(tx2); });
    // let (tx1, rx1): (SyncSender<Vec<u8>>, Receiver<Vec<u8>>) = sync_channel(queue_size);
    // let arx1 = Arc::new(Mutex::new(rx1));
    // output_transport.start(arx1);
    // input_udp.accept(tx1);
}
