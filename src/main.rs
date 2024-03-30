mod cli;
mod event;
mod logging;
mod parse;

use crate::logging::EventLogging;
use crate::parse::Config;

fn main() {
    EventLogging::start();
    let cfg = Config::parse();
    EventLogging::finish();
}
