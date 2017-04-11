use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};
use std::thread;
use pamcollector::config::Config;
use hyper::client::Client;
use hyper;
use std::io::Read;

pub struct ClickHouseOutput {
    conf: Config,
}

pub trait Output {
    fn start(&self, arx: Arc<Mutex<Receiver<Vec<u8>>>>);
}

impl ClickHouseOutput {
    pub fn new(_config: &Config) -> ClickHouseOutput {
        ClickHouseOutput { conf: _config.clone() }
    }
}

fn to_ch_sql(res_vec: &Vec<String>, conf: &Config) -> String {
    let strings = res_vec.iter()
        .map(|x| format!("({})", x))
        .collect::<Vec<_>>()
        .join(", ");
    let client = Client::new();


    let sql = format!("INSERT INTO timers (metric_name, VALUE, ts, app_name, app_layer, \
                       operation, tags, val_tags) VALUES {}",
                      strings);
    let mut res_text = String::new();
    let mut res = client.post(&format!("{}?", conf.get_ch_address()))
        .body(&sql)
        .send()
        .unwrap();
    res.read_to_string(&mut res_text).unwrap();
    if hyper::Ok != res.status {
        warn!("Failed push to CH {}", res_text);
    };

    info!("Resp fronm CH {}", res_text);
    sql
}

fn output_spawn(bytes: &Vec<u8>, res_vec: &mut Vec<String>, conf: &Config) -> Result<(), String> {
    let out: String = String::from_utf8_lossy(&bytes).into_owned();
    res_vec.push(out);
    if res_vec.len() > conf.get_output_queue_size() as usize {
        warn!("FULL");
        debug!("{}", to_ch_sql(&res_vec, &conf));
        res_vec.clear();
    };
    Ok(())
}

impl Output for ClickHouseOutput {
    fn start(&self, arx: Arc<Mutex<Receiver<Vec<u8>>>>) {
        let mut res_vec: Vec<String> = Vec::new();
        let _conf = self.conf.clone();
        thread::spawn(move || loop {
            let bytes = match arx.lock().unwrap().recv() {
                Ok(line) => line,
                Err(_) => return,
            };
            let _ = output_spawn(&bytes, &mut res_vec, &_conf);
        });
    }
}
