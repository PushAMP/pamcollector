extern crate serde;
extern crate clap;
extern crate serde_cbor;
extern crate hyper;
extern crate chrono;
extern crate toml;
extern crate bytes;
extern crate futures;
extern crate tokio_io;
extern crate tokio_proto;
extern crate tokio_service;
extern crate tokio_core;
extern crate log4rs;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

use clap::{App, Arg};
use log::LogLevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Config, Root};
mod pamcollector;


const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const DEFAULT_CONF_FILE: &'static str = "pamcollector.toml";

fn main() {
    let stdout = ConsoleAppender::builder().build();
    let log_config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(Root::builder().appender("stdout").build(
            LogLevelFilter::Info,
        ))
        .unwrap();

    let _ = log4rs::init_config(log_config).unwrap();
    let matches = App::new("PaMCollector")
        .version(VERSION)
        .about("PushAMP Metric Collector")
        .arg(
            Arg::with_name("config_file")
                .short("c")
                .long("config")
                .help("Configuration file")
                .value_name("FILE"),
        )
        .get_matches();
    info!("PaMCollector {}", VERSION);
    let config_path = matches.value_of("config_file").unwrap_or(DEFAULT_CONF_FILE);
    pamcollector::start(&config_path)
}
