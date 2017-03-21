use std::io::{stderr, Write};
use std::net::UdpSocket;
use std::str;
use std::sync::mpsc::SyncSender;

pub trait Input {
    fn accept(&self, tx: SyncSender<Vec<u8>>);
}

pub struct UdpInput {
    listen: String,
}

impl UdpInput {
    pub fn new() -> UdpInput {
        let listen = "0.0.0.0:12345".to_owned();
        UdpInput { listen: listen }
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
            let line = &buf[..length];
            if let Err(e) = handle_record(&line, &tx) {
                let _ = writeln!(stderr(), "{}", e);
            }
        }
    }
}

fn handle_record(line: &[u8], tx: &SyncSender<Vec<u8>>) -> Result<(), &'static str> {
    match tx.send(line.to_vec()) {
        Err(e) => println!("{}", e),
        Ok(re) => println!("OK{:?}", re),
    };
    Ok(())
}
