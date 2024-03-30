use log::{debug, info};
use std::fs::File;
use std::io::{BufReader, Read};
use xdg::BaseDirectories;

use crate::logging::EventLogging;

const RELATIVE_CONFIG_PATH: &'static str = "reme/config.json";

#[derive(Debug)]
pub(crate) struct Config {
    file: File,
    data: String,
}

impl Config {
    pub(crate) fn parse() -> Self {
        let cfg_file = Self::file();
        let cfg_str = Self::read(&cfg_file);

        info!("config file and config string are fine");

        // let event = serde_json::from_str::<Event>(cfg_str.as_str()).unwrap_or_else(|err| {
        //     let msg = format!("unable to deserialize config file content: {err}");
        //     EventLogging::panic(msg.as_str());
        // });

        // info!("events parsed fine");
        // debug!("events are: {:?}", event);

        debug!("config file content is\n{}", cfg_str);

        Self {
            file: cfg_file,
            data: cfg_str,
        }
    }

    fn file() -> File {
        let cfg_dir = BaseDirectories::new().unwrap_or_else(|_| {
            EventLogging::panic("unable to find XDG-compliant directory hierarchy!")
        });
        info!("config dir is {:?}", &cfg_dir);

        let cfg_path = cfg_dir
            .find_config_file(RELATIVE_CONFIG_PATH)
            .unwrap_or_else(|| EventLogging::panic("unable to find config file in directories!"));
        info!("config path is {:?}", &cfg_path);

        // SAFETY: we've already checked this path for existence so
        // it is totally safe to unwrap it like so
        let cfg_file = unsafe { File::open(cfg_path).unwrap_unchecked() };
        info!("config file state is {:?}", &cfg_file);

        cfg_file
    }

    fn read(cfg_file: &File) -> String {
        let mut cfg_str = String::new();
        let mut cfg_reader = BufReader::new(cfg_file);

        cfg_reader
            .read_to_string(&mut cfg_str)
            .unwrap_or_else(|_| EventLogging::panic("unable to read config file content"));
        info!("config reader state is {:?}", &cfg_reader);

        cfg_str
    }
}
