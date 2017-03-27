use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};
use std::thread;
use serde_json;
use pamcollector::metric::Metric;
use hyper::client::Client;
use hyper;
use std::io::Read;
pub struct ConsoleOutput {}

pub trait Output {
    fn start(&self, arx: Arc<Mutex<Receiver<Vec<u8>>>>);
}

impl ConsoleOutput {
    pub fn new() -> ConsoleOutput {
        ConsoleOutput {}
    }
}

fn to_ch_sql(res_vec: &Vec<Metric>) -> String {
    let strings = res_vec.iter()
                 .map(|x| format!("({})", x.to_val().join(", ")))
                //  .join(", ")
                 .collect::<Vec<_>>().join(", ");
    //  .concat();
    let client = Client::new();


    let sql = format!("INSERT INTO timers (metric_name, VALUE, ts, app_name, app_layer, \
                       operation) VALUES {}",
                      strings);
    let mut res_text = String::new();
    let mut res = client.post("http://127.0.0.1:8123/?")
        .body(&sql)
        .send()
        .unwrap();
    res.read_to_string(&mut res_text).unwrap();
    if hyper::Ok != res.status {
        println!("Failed push to CH {}", res_text);
    };

    println!("{}", res_text);
    sql
}

fn output_spawn(bytes: &Vec<u8>, res_vec: &mut Vec<Metric>) -> Result<(), String> {
    let out = String::from_utf8_lossy(&bytes);
    let m: Metric = try!(serde_json::from_str(&out)
        .or(Err("Invalid input, unable to parse as a JSON object")));
    res_vec.push(m);
    if res_vec.len() > 2 {
        println!("FULL");
        println!("{}", to_ch_sql(&res_vec));
        res_vec.clear();
    };
    Ok(())
}


impl Output for ConsoleOutput {
    fn start(&self, arx: Arc<Mutex<Receiver<Vec<u8>>>>) {
        let mut res_vec: Vec<Metric> = Vec::new();
        thread::spawn(move || loop {
                          let bytes = match {
                                    arx.lock().unwrap().recv()
                                } {
                              Ok(line) => line,
                              Err(_) => return,
                          };
                          let _ = output_spawn(&bytes, &mut res_vec);
                      });
    }
}

