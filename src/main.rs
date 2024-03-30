mod cli;
mod event;
mod logging;
mod parse;

use crate::event::Event;
use crate::logging::EventLogging;
use crate::parse::Config;

fn main() {
    EventLogging::start();

    let cfg = Config::parse();

    let data = cfg.data();
    let events = Event::parse(data);

    EventLogging::finish();
}
