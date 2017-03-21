mod pamcollector;
use clap::{Arg, App};
extern crate clap;
use std::io::{stderr, Write};

const VERSION: &'static str = "0.0.1";


fn main() {
    let matches = App::new("PaMCollector")
        .version(VERSION)
        .about("PushAMP Metric Collector")
        .get_matches();
    let _ = writeln!(stderr(), "PaMCollector {}", VERSION);
    pamcollector::start()
}
