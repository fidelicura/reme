use chrono::NaiveDateTime;
use log::{debug, info};
use notify_rust::Notification;
use serde::Deserialize;

use crate::logging::EventLogging;

#[derive(Debug)]
enum EventTimeWarn {
    Seconds(u8),
    Minutes(u8),
    Hours(u8),
    Days(u8),
    Weeks(u8),
    Months(u8),
    Years(u8),
}

impl<'de> Deserialize<'de> for EventTimeWarn {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value: String = Deserialize::deserialize(deserializer)?;
        let splitted = value.split_whitespace().collect::<Vec<&str>>();
        let (raw_amount, variant) = (splitted[0], splitted[1]);

        let amount = raw_amount.parse::<u8>().unwrap_or_else(|err| {
            let msg = format!("deserialization of time value amount has failed because of {err}");
            EventLogging::panic(&msg);
        });

        Ok(match variant {
            "second" | "seconds" => EventTimeWarn::Seconds(amount),
            "minute" | "minutes" => EventTimeWarn::Minutes(amount),
            "hour" | "hours" => EventTimeWarn::Hours(amount),
            "day" | "days" => EventTimeWarn::Days(amount),
            "week" | "weeks" => EventTimeWarn::Weeks(amount),
            "month" | "months" => EventTimeWarn::Months(amount),
            "year" | "years" => EventTimeWarn::Years(amount),
            _ => {
                EventLogging::panic("unable to deserialize specified time warn time variant format")
            }
        })
    }
}

#[derive(Debug, Deserialize)]
struct EventTime {
    pub(crate) post: NaiveDateTime,
    pub(crate) warn: Option<EventTimeWarn>,
}

#[derive(Debug, Deserialize)]
enum EventPriority {
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "critical")]
    Critical,
}

#[derive(Debug, Deserialize)]
struct EventMessage {
    pub(crate) additional: Option<String>,
    pub(crate) main: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Event {
    pub(crate) message: EventMessage,
    pub(crate) priority: EventPriority,
    pub(crate) time: EventTime,
}

impl Event {
    pub(crate) fn notify(&self) {
        let notif = if let Some(additional) = &self.message.additional {
            Notification::new()
                .summary(&self.message.main)
                .body(&additional)
                .show()
                .unwrap_or_else(|err| {
                    let msg = format!("failed to create notification because of {err}");
                    EventLogging::panic(&msg);
                })
        } else {
            Notification::new()
                .summary(&self.message.main)
                .show()
                .unwrap_or_else(|err| {
                    let msg = format!("failed to create notification because of {err}");
                    EventLogging::panic(&msg);
                })
        };
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct Events {
    event: Vec<Event>,
}

impl Events {
    pub(crate) fn parse(raw_data: &str) -> Self {
        let events = toml::from_str(raw_data).unwrap_or_else(|err| {
            let msg = format!("unable to deserialize config file content: {err}");
            EventLogging::panic(&msg);
        });

        debug!("events are {:?}", &events);
        info!("events parse and deserialization has finished fine");

        events
    }

    pub(crate) fn events(&mut self) -> &mut Vec<Event> {
        &mut self.event
    }
}
