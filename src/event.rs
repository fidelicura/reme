use log::{debug, info};
use serde::Deserialize;
use std::time::SystemTime;

use crate::logging::EventLogging;

#[derive(Debug, Deserialize)]
pub(crate) struct EventTime {
    post: SystemTime,
    warn: Option<SystemTime>,
}

#[derive(Debug, Deserialize)]
pub(crate) enum EventPriority {
    Low,
    Normal,
    Critical,
}

#[derive(Debug, Deserialize)]
pub(crate) struct EventMessage {
    main: String,
    additional: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Event {
    time: EventTime,
    priority: EventPriority,
    message: EventMessage,
}

impl Event {
    pub(crate) fn parse(data: &str) -> Vec<Self> {
        let events: Vec<Self> = serde_json::from_str(data).unwrap_or_else(|err| {
            let msg = format!("unable to deserialize config file content: {err}");
            EventLogging::panic(msg.as_str());
        });

        info!("events parsed fine");
        debug!("events are: {:?}", &events);

        events
    }
}
