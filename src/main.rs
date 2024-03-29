use log::{error, info, warn, LevelFilter};
use serde::Deserialize;
use simple_logger::SimpleLogger;
use std::time::SystemTime;
use xdg::BaseDirectories;

const RELATIVE_CONFIG_PATH: &'static str = "reme/config.json";

#[derive(Deserialize)]
struct EventTime {
    post: SystemTime,
    beforehand: Option<SystemTime>,
}

impl EventTime {
    fn new(post: SystemTime, beforehand: Option<SystemTime>) -> Self {
        Self { post, beforehand }
    }
}

#[derive(Deserialize)]
enum EventPriority {
    Low,
    Normal,
    Critical,
}

#[derive(Deserialize)]
struct EventMessage {
    main: String,
    additional: String,
}

impl EventMessage {
    fn new(main: String, additional: String) -> Self {
        Self { main, additional }
    }
}

#[derive(Deserialize)]
struct Event {
    time: EventTime,
    priority: EventPriority,
    message: EventMessage,
}

impl Event {
    fn new(time: EventTime, priority: EventPriority, message: EventMessage) -> Self {
        Self {
            time,
            priority,
            message,
        }
    }
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

    let cfg_json = match cfg_dir.find_config_file(RELATIVE_CONFIG_PATH) {
        Some(json) => json,
        None => EventLogging::panic("unable to find config file in directories!"),
    };

    // serialize json as `Event` struct

    EventLogging::finish();
}
