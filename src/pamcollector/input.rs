use std::io::{stderr, Write, ErrorKind, BufRead, BufReader};
use std::net::UdpSocket;
use std::str;
use std::sync::mpsc::SyncSender;
use serde_json;
use pamcollector::metric::Metric;
use pamcollector::config::Config;
use std::net::{TcpListener, TcpStream};
use std::thread;

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
        let socket = UdpSocket::bind(&self.listen as &str)
            .expect(&format!("Unable to listen to {}", self.listen));
        let tx = tx.clone();

        let mut buf = [0; 65527];
        loop {
            let (length, _src) = match socket.recv_from(&mut buf) {
                Ok(res) => res,
                Err(_) => continue,
            };
            let line = String::from_utf8_lossy(&buf[..length]);
            if let Err(e) = handle_record(&line.into_owned(), &tx) {
                let _ = writeln!(stderr(), "{}", e);
            }
        }
    }
}

pub struct TcpInput {
    listen: String,
}

impl TcpInput {
    pub fn new(config: &Config) -> TcpInput {
        TcpInput { listen: config.get_tcp_input().to_string() }
    }
}

impl Input for TcpInput {
    fn accept(&self, tx: SyncSender<Vec<u8>>) {
        let listener = TcpListener::bind(&self.listen as &str).unwrap();
        for client in listener.incoming() {
            match client {
                Ok(client) => {
                    let tx = tx.clone();
                    thread::spawn(move || { handle_client(client, tx); });
                }
                Err(_) => {}
            }
        }
    }
}


fn handle_client(client: TcpStream, tx: SyncSender<Vec<u8>>) {
    if let Ok(peer_addr) = client.peer_addr() {
        println!("Connection over TCP from [{}]", peer_addr);
    }
    let buf_reader = BufReader::new(client);
    for line in buf_reader.lines() {
        let line = match line {
            Ok(line) => line,
            Err(e) => {
                match e.kind() {
                    ErrorKind::Interrupted => continue,
                    ErrorKind::InvalidInput | ErrorKind::InvalidData => {
                        let _ = writeln!(stderr(), "Invalid UTF-8 input");
                        continue;
                    }
                    ErrorKind::WouldBlock => {
                        let _ = writeln!(stderr(),
                                         "Client hasn't sent any data for a while - Closing idle \
                                          connection");
                        return;
                    }
                    _ => return,
                }
            }
        };
        if let Err(e) = handle_record(&line, &tx) {
            let _ = writeln!(stderr(), "{}: [{}]", e, line.trim());
        }
    }
}

fn handle_record(line: &String, tx: &SyncSender<Vec<u8>>) -> Result<(), String> {
    let m: Metric =
        serde_json::from_str(&line).or(Err("Invalid input, unable to parse as a JSON object"))?;
    let rencoded =
        serde_json::to_vec(&m).or(Err("Invalid input, unable to reencoded JSON to vec"))?;
    tx.send(rencoded)
        .or(Err("Invalid input, unable to send to tx"))?;
    Ok(())
}
