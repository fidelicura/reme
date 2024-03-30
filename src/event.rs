use serde::Deserialize;
use std::time::SystemTime;

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
