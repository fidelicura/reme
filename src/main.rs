use log::{error, info, LevelFilter};
use serde::Deserialize;
use simple_logger::SimpleLogger;
use std::fs::File;
use std::io::{BufReader, Read};
use std::time::SystemTime;
use xdg::BaseDirectories;

const RELATIVE_CONFIG_PATH: &'static str = "reme/config.json";

#[derive(Debug, Deserialize)]
struct EventTime {
    post: SystemTime,
    warn: Option<SystemTime>,
}

#[derive(Debug, Deserialize)]
enum EventPriority {
    Low,
    Normal,
    Critical,
}

#[derive(Debug, Deserialize)]
struct EventMessage {
    main: String,
    additional: String,
}

#[derive(Debug, Deserialize)]
struct Event {
    time: EventTime,
    priority: EventPriority,
    message: EventMessage,
}

struct EventLogging(SimpleLogger);

impl EventLogging {
    fn start(level: LevelFilter) {
        SimpleLogger::new().with_level(level).init().unwrap();
        info!("logging started");
    }

    fn finish() {
        info!("logging finished");
    }

    fn panic(msg: &str) -> ! {
        error!("{msg}");
        panic!("{msg}");
    }
}

fn main() {
    EventLogging::start(LevelFilter::Info);

    let cfg_dir =
        BaseDirectories::new().expect("unable to find XDG-compliant directory hierarchy!");

    let cfg_path = cfg_dir
        .find_config_file(RELATIVE_CONFIG_PATH)
        .unwrap_or_else(|| EventLogging::panic("unable to find config file in directories!"));
    info!("config path is {:?}", &cfg_path);

    // SAFETY: we've already checked this path for existence so
    // it is totally safe to unwrap it like so
    let cfg_file = unsafe { File::open(cfg_path).unwrap_unchecked() };
    info!("config file state is {:?}", &cfg_file);

    let mut cfg_str = String::new();
    let mut cfg_reader = BufReader::new(cfg_file);

    cfg_reader
        .read_to_string(&mut cfg_str)
        .unwrap_or_else(|_| EventLogging::panic("unable to read config file content"));

    // serialize json as `Event` struct
    let event = serde_json::from_str::<Event>(cfg_str.as_str())
        .unwrap_or_else(|_| EventLogging::panic("unable to deserialize config file content"));

    dbg!(event);

    EventLogging::finish();
}
