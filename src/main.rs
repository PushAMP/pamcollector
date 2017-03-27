extern crate clap;
extern crate serde_json;
extern crate hyper;
extern crate chrono;
#[macro_use] extern crate serde_derive;
mod pamcollector;


use std::io::{stderr, Write};
use clap::App;

const VERSION: &'static str = "0.0.1";


fn main() {
    let _ = App::new("PaMCollector")
        .version(VERSION)
        .about("PushAMP Metric Collector")
        .get_matches();
    let _ = writeln!(stderr(), "PaMCollector {}", VERSION);
    pamcollector::start()
}
