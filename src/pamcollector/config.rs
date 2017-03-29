use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::io::{Error, ErrorKind};
use std::convert::AsRef;
use toml;
const DEFAULT_UDP_LISTEN: &'static str = "0.0.0.0:12345";
const DEFAULT_CH_ADDRESS: &'static str = "http://0.0.0.0:8123/";

#[derive(Deserialize, Clone)]
pub struct Config {
    input: Option<Input>,
    output: Option<Output>,
}

#[derive(Deserialize, Clone)]
struct Input {
    udp_listen: Option<String>,
}

#[derive(Deserialize, Clone)]
struct Output {
    ch_address: Option<String>,
}

impl Config {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Config, Error> {
        let mut fd = try!(File::open(path));
        let mut toml_str = String::new();
        try!(fd.read_to_string(&mut toml_str));
        Config::from_string(&toml_str)
    }

    pub fn from_string(toml_str: &str) -> Result<Config, Error> {
        let config: Config = match toml::from_str(toml_str) {
            Ok(config) => config,
            Err(e) => {
                println!("{}", e);
                return Err(Error::new(ErrorKind::InvalidData, "Config file is not valid TOML. {}"));
            }
        };
        Ok(config)
    }

    pub fn get_udp_input(&self) -> &str {
        match self.input {
            None => DEFAULT_UDP_LISTEN,
            Some(ref input) => {
                input.udp_listen
                    .as_ref()
                    .map(AsRef::as_ref)
                    .unwrap_or(DEFAULT_UDP_LISTEN)
            }
        }
    }

    pub fn get_ch_address(&self) -> &str {
        match self.output {
            None => DEFAULT_CH_ADDRESS,
            Some(ref output) => {
                output.ch_address
                    .as_ref()
                    .map(AsRef::as_ref)
                    .unwrap_or(DEFAULT_CH_ADDRESS)
            }
        }
    }
}

