use log::{debug, error, info, LevelFilter};
use simple_logger::SimpleLogger;

use crate::cli::Cli;

pub(crate) struct EventLogging(SimpleLogger);

impl EventLogging {
    pub(crate) fn start() {
        let log_mode = match Cli::mode() {
            true => LevelFilter::Debug,
            false => LevelFilter::Info,
        };

        SimpleLogger::new().with_level(log_mode).init().unwrap();
        info!("logging started");

        if log_mode == LevelFilter::Debug {
            debug!("app started in debug mode");
        }
    }

    pub(crate) fn finish() {
        info!("logging finished");
    }

    pub(crate) fn panic(msg: &str) -> ! {
        error!("{msg}");
        panic!("{msg}");
    }
}
