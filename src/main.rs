mod cli;
mod event;
mod logging;
mod parse;

use crate::event::Events;
use crate::logging::EventLogging;
use crate::parse::Config;

fn main() {
    EventLogging::start();

    let cfg = Config::parse();

    let data = cfg.data();
    let mut events = Events::parse(data);
    let events = events.events();

    events.iter().for_each(|event| {
        event.notify();
    });

    EventLogging::finish();
}
