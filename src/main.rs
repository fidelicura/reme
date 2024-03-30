mod cli;
mod event;
mod logging;
mod parse;

use log::info;

use crate::event::Events;
use crate::logging::EventLogging;
use crate::parse::Config;

fn main() {
    EventLogging::start();

    let cfg = Config::parse();
    info!("config parsing has finished fine");

    let data = cfg.data();
    let events = Events::parse(data);
    info!("events deserialization has finished fine");

    EventLogging::finish();
}
