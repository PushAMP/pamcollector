#![feature(retain_hash_collection)]

extern crate clap;
extern crate serde_json;
extern crate hyper;
extern crate chrono;
extern crate toml;
#[macro_use]
extern crate serde_derive;
mod pamcollector;

use std::io::{stderr, Write};
use clap::{App, Arg};

const VERSION: &'static str = "0.0.1";
const DEFAULT_CONF_FILE: &'static str = "pamcollector.toml";


fn main() {
    let matches = App::new("PaMCollector")
        .version(VERSION)
        .about("PushAMP Metric Collector")
        .arg(Arg::with_name("config_file")
            .short("c")
            .long("config")
            .help("Configuration file")
            .value_name("FILE"))
        .get_matches();
    let _ = writeln!(stderr(), "PaMCollector {}", VERSION);
    let config_path = matches.value_of("config_file").unwrap_or(DEFAULT_CONF_FILE);
    pamcollector::start(&config_path)
}
