use chrono::NaiveDateTime;
use log::{debug, info};
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
            EventLogging::panic(msg.as_str());
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
    post: NaiveDateTime,
    warn: Option<EventTimeWarn>,
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
    additional: Option<String>,
    main: String,
}

#[derive(Debug, Deserialize)]
struct Event {
    message: EventMessage,
    priority: EventPriority,
    time: EventTime,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Events {
    event: Vec<Event>,
}

impl Events {
    pub(crate) fn parse(raw_data: &str) -> Self {
        let events = toml::from_str(raw_data).unwrap_or_else(|err| {
            let msg = format!("unable to deserialize config file content: {err}");
            EventLogging::panic(msg.as_str());
        });

        info!("events parsed fine");
        debug!("events are {:?}", &events);

        events
    }
}
