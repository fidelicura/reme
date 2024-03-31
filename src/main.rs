mod cli;
mod event;
mod logging;
mod parse;

use std::time::Duration;

use crate::event::Events;
use crate::logging::EventLogging;
use crate::parse::Config;

fn main() {
    EventLogging::start();

    let cfg = Config::parse();

    let data = cfg.data();
    let mut events = Events::parse(data);

    let sleep_time = Duration::from_secs(1);
    events.start(sleep_time);

    EventLogging::finish();
}
