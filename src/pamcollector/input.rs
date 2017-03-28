use std::io::{stderr, Write};
use std::net::UdpSocket;
use std::str;
use std::sync::mpsc::SyncSender;
use serde_json;
use pamcollector::metric::Metric;
use pamcollector::config::Config;

pub trait Input {
    fn accept(&self, tx: SyncSender<Vec<u8>>);
}

pub struct UdpInput {
    listen: String,
}

impl UdpInput {
    pub fn new(config: &Config) -> UdpInput {
        let listen = config.get_udp_input();
        UdpInput { listen: listen.to_string() }
    }
}

impl Input for UdpInput {
    fn accept(&self, tx: SyncSender<Vec<u8>>) {
        let socket =
            UdpSocket::bind(&self.listen as &str).expect(&format!("Unable to listen to {}",
                                                                  self.listen));
        let tx = tx.clone();

        let mut buf = [0; 65527];
        loop {
            let (length, _src) = match socket.recv_from(&mut buf) {
                Ok(res) => res,
                Err(_) => continue,
            };
            let line = &buf[..length];
            if let Err(e) = handle_record(&line, &tx) {
                let _ = writeln!(stderr(), "{}", e);
            }
        }
    }
}

fn handle_record(line: &[u8], tx: &SyncSender<Vec<u8>>) -> Result<(), String> {
    let out = String::from_utf8_lossy(&line);
    let m: Metric = try!(serde_json::from_str(&out)
        .or(Err("Invalid input, unable to parse as a JSON object")));
    let rencoded = try!(serde_json::to_vec(&m)
        .or(Err("Invalid input, unable to reencoded JSON to vec")));
    try!(tx.send(rencoded).or(Err("Invalid input, unable to send to tx")));
    Ok(())
}

