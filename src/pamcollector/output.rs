use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};
use std::thread;

pub struct ConsoleOutput {}

pub trait Output {
    fn start(&self, arx: Arc<Mutex<Receiver<Vec<u8>>>>);
}

impl ConsoleOutput {
    pub fn new() -> ConsoleOutput {
        ConsoleOutput {}
    }
}

impl Output for ConsoleOutput {
    fn start(&self, arx: Arc<Mutex<Receiver<Vec<u8>>>>) {
        let mut res_vec = Vec::new();
        thread::spawn(move || loop {
            let bytes = match {
                arx.lock().unwrap().recv()
            } {
                Ok(line) => line,
                Err(_) => return,
            };
            let out = String::from_utf8_lossy(&bytes);
            let decoded = format!("{}", out);
            res_vec.push(decoded);
            if res_vec.len() > 2 {
                println!("FULL");
                println!("{:?}", res_vec);
                res_vec.clear();
            }
        });
    }
}
