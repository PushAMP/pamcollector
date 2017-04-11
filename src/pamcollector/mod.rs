use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::sync::{Arc, Mutex};
use std::error::Error;
mod config;
mod output;
mod metric;
mod codec;
mod service;
mod protocol;

use self::config::Config;
use self::output::Output;
use self::output::ClickHouseOutput;
use self::protocol::LineProto;
use self::service::MetricInputService;
use tokio_proto::TcpServer;

pub fn start(config_path: &str) {
    let config = match Config::from_path(&config_path) {
        Ok(config) => config,
        Err(e) => {
            error!("Fail to read config {}. {}", config_path, e.description());
            ::std::process::exit(1)
        }
    };
    let output_transport = ClickHouseOutput::new(&config);
    let queue_size = config.get_queue_size();
    let addr = match config.get_tcp_input().parse() {
        Ok(addr) => addr,
        Err(e) => {
            error!("Fail to parse tcp input address {}. {:?}",
                   config.get_tcp_input(),
                   e);
            ::std::process::exit(1)
        }
    };
    let server = TcpServer::new(LineProto, addr);
    let (tx, rx): (SyncSender<Vec<u8>>, Receiver<Vec<u8>>) = sync_channel(queue_size);
    let atx = Arc::new(Mutex::new(tx));
    let arx = Arc::new(Mutex::new(rx));
    output_transport.start(arx);
    server.serve(move || Ok(MetricInputService { tx: atx.clone() }));
}
