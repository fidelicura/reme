use chrono::Local;
use env_logger::Builder as LoggingBuilder;
use log::{debug, error, info, LevelFilter};
use std::io::Write;

use crate::cli::Cli;

pub(crate) struct EventLogging;

impl EventLogging {
    pub(crate) fn start() {
        let log_mode = match Cli::mode() {
            true => LevelFilter::Debug,
            false => LevelFilter::Info,
        };

        LoggingBuilder::new()
            .filter_level(log_mode)
            .format(move |buf, record| {
                let record_time = Local::now().format("%Y-%m-%d %H:%M:%S");
                let record_level = record.level();
                let record_args = record.args();

                writeln!(buf, "[{} {}]: {}", record_time, record_level, record_args)
            })
            .init();

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
