use chrono::NaiveDateTime;
use log::{debug, info};
use notify_rust::{Notification, Urgency};
use serde::Deserialize;

use crate::logging::EventLogging;

#[derive(Debug)]
pub(crate) enum EventTimeWarn {
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
pub(crate) struct EventTime {
    pub(crate) post: NaiveDateTime,
    pub(crate) warn: Option<EventTimeWarn>,
}

#[derive(Default, Debug, Deserialize)]
pub(crate) enum EventPriority {
    #[default]
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "critical")]
    Critical,
}

impl EventPriority {
    pub(crate) fn urgency(&self) -> Urgency {
        match self {
            Self::Low => Urgency::Low,
            Self::Normal => Urgency::Normal,
            Self::Critical => Urgency::Critical,
        }
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct EventMessage {
    pub(crate) additional: Option<String>,
    pub(crate) main: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Event {
    pub(crate) message: EventMessage,
    pub(crate) priority: Option<EventPriority>,
    pub(crate) time: EventTime,
}

impl Event {
    pub(crate) fn notify(&self) {
        let mut basic_notif = Notification::new().summary(&self.message.main).clone();

        let is_set_additional = &self.message.additional.is_some();
        let is_set_priority = &self.priority.is_some();

        unsafe {
            match (is_set_additional, is_set_priority) {
                // SAFETY: both additionoal and priorities are presented
                (true, true) => {
                    let additional = &self.message.additional.as_ref().unwrap_unchecked();
                    let priority = &self.priority.as_ref().unwrap_unchecked();
                    basic_notif.body(additional).urgency(priority.urgency());
                }
                // SAFETY: additional is presented, priority is not
                (true, false) => {
                    let additional = &self.message.additional.as_ref().unwrap_unchecked();
                    basic_notif
                        .body(additional)
                        .urgency(EventPriority::default().urgency());
                }
                // SAFETY: additional is not presented, priority is
                (false, true) => {
                    let priority = &self.priority.as_ref().unwrap_unchecked();
                    basic_notif.urgency(priority.urgency());
                }
                // SAFETY: both additional and priority are not presented
                (false, false) => {
                    basic_notif.urgency(EventPriority::default().urgency());
                }
            }
        }

        basic_notif.show().unwrap_or_else(|err| {
            let msg = format!("failed to create notification because of {err}");
            EventLogging::panic(&msg);
        });
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

        debug!("events are {:#?}", &events);
        info!("events parse and deserialization has finished fine");

        events
    }

    pub(crate) fn events(&mut self) -> &mut Vec<Event> {
        &mut self.event
    }
}
