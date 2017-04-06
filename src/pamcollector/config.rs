use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::io::{Error, ErrorKind};
use std::convert::AsRef;
use toml;

const DEFAULT_TCP_LISTEN: &'static str = "0.0.0.0:9091";
const DEFAULT_QUEUE_SIZE: usize = 20_000;
const DEFAULT_CH_ADDRESS: &'static str = "http://0.0.0.0:8123/";
const DEFAULT_OUT_QUEUE_SIZE: u16 = 100;

#[derive(Deserialize, Clone)]
pub struct Config {
    input: Option<Input>,
    output: Option<Output>,
}

#[derive(Deserialize, Clone)]
struct Input {
    udp_listen: Option<String>,
    tcp_listen: Option<String>,
    queue_size: Option<usize>,
}

#[derive(Deserialize, Clone)]
struct Output {
    ch_address: Option<String>,
    queue_size: Option<u16>,
}

impl Config {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Config, Error> {
        let mut fd = File::open(path)?;
        let mut toml_str = String::new();
        fd.read_to_string(&mut toml_str)?;
        Config::from_string(&toml_str)
    }

    pub fn from_string(toml_str: &str) -> Result<Config, Error> {
        let config: Config = match toml::from_str(toml_str) {
            Ok(config) => config,
            Err(e) => {
                error!("{}", e);
                return Err(Error::new(ErrorKind::InvalidData, "Config file is not valid TOML. {}"));
            }
        };
        Ok(config)
    }

    pub fn get_tcp_input(&self) -> &str {
        match self.input {
            None => DEFAULT_TCP_LISTEN,
            Some(ref input) => {
                input
                    .tcp_listen
                    .as_ref()
                    .map(AsRef::as_ref)
                    .unwrap_or(DEFAULT_TCP_LISTEN)
            }
        }
    }

    pub fn get_ch_address(&self) -> &str {
        match self.output {
            None => DEFAULT_CH_ADDRESS,
            Some(ref output) => {
                output
                    .ch_address
                    .as_ref()
                    .map(AsRef::as_ref)
                    .unwrap_or(DEFAULT_CH_ADDRESS)
            }
        }
    }

    pub fn get_queue_size(&self) -> usize {
        match self.input {
            Some(ref input) => input.queue_size.unwrap_or(DEFAULT_QUEUE_SIZE),
            None => DEFAULT_QUEUE_SIZE,
        }
    }
    pub fn get_output_queue_size(&self) -> u16 {
        match self.output {
            Some(ref output) => output.queue_size.unwrap_or(DEFAULT_OUT_QUEUE_SIZE),
            None => DEFAULT_OUT_QUEUE_SIZE,
        }
    }
}

