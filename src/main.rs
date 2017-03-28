extern crate clap;
extern crate serde_json;
extern crate hyper;
extern crate chrono;
extern crate toml;

#[macro_use]
extern crate serde_derive;
mod pamcollector;

use std::io::{stderr, Write};
use clap::App;

const VERSION: &'static str = "0.0.1";
const DEFAULT_CONF_FILE: &'static str = "pamcollector.toml";


fn main() {
    let _ = App::new("PaMCollector")
        .version(VERSION)
        .about("PushAMP Metric Collector")
        .get_matches();
    let _ = writeln!(stderr(), "PaMCollector {}", VERSION);
    let config_path = DEFAULT_CONF_FILE;
    pamcollector::start(&config_path)
}
