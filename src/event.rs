use chrono::{Datelike, Local, NaiveDateTime, Timelike};
use log::{debug, info};
use notify_rust::{Notification, Urgency};
use serde::Deserialize;
use std::thread;
use std::time::Duration;

use crate::logging::EventLogging;

#[derive(Debug, PartialEq, PartialOrd)]
pub(crate) enum EventTimeWarn {
    Seconds(u8),
    Minutes(u8),
    Hours(u8),
    Days(u8),
    Weeks(u8),
    Months(u8),
    Years(u32),
}

impl EventTimeWarn {
    fn is_time(&self, when: NaiveDateTime) -> bool {
        match self {
            Self::Seconds(val) => {
                let seconds = when.second() as u8;
                debug!("seconds are {:#?}", &seconds);
                seconds % val == 0
            }
            Self::Minutes(val) => {
                let minutes = when.minute() as u8;
                debug!("minutes are {:#?}", &minutes);
                minutes % val == 0
            }
            Self::Hours(val) => {
                let hours = when.hour() as u8;
                debug!("hours are {:#?}", &hours);
                hours % val == 0
            }
            Self::Days(val) => {
                let days = when.day() as u8;
                debug!("days are {:#?}", &days);
                days % val == 0
            }
            Self::Weeks(val) => {
                let weeks = when.weekday().number_from_monday() as u8;
                debug!("weeks are {:#?}", &weeks);
                weeks % val == 0
            }
            Self::Months(val) => {
                let months = when.month() as u8;
                debug!("months are {:#?}", &months);
                months % val == 0
            }
            Self::Years(val) => {
                let years = when.year() as u32;
                debug!("years are {:#?}", &years);
                years % val == 0
            }
        }
    }
}

impl<'de> Deserialize<'de> for EventTimeWarn {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value: String = Deserialize::deserialize(deserializer)?;
        debug!(
            "deserialized event time warn string as raw is {:#?}",
            &value
        );
        let splitted = value.split_whitespace().collect::<Vec<&str>>();
        let (raw_amount, variant) = (splitted[0], splitted[1]);

        let amount = raw_amount.parse::<u32>().unwrap_or_else(|err| {
            let msg = format!("deserialization of time value amount has failed because of {err}");
            EventLogging::panic(&msg);
        });
        debug!("deserialized value amount of warn string is {:#?}", amount);

        Ok(match variant {
            "second" | "seconds" => EventTimeWarn::Seconds(amount as u8),
            "minute" | "minutes" => EventTimeWarn::Minutes(amount as u8),
            "hour" | "hours" => EventTimeWarn::Hours(amount as u8),
            "day" | "days" => EventTimeWarn::Days(amount as u8),
            "week" | "weeks" => EventTimeWarn::Weeks(amount as u8),
            "month" | "months" => EventTimeWarn::Months(amount as u8),
            "year" | "years" => EventTimeWarn::Years(amount),
            _ => {
                EventLogging::panic("unable to deserialize specified time warn time variant format")
            }
        })
    }
}

#[derive(Debug, Deserialize, PartialEq, PartialOrd)]
pub(crate) struct EventTime {
    pub(crate) post: NaiveDateTime,
    pub(crate) warn: Option<EventTimeWarn>,
}

impl EventTime {
    fn is_post_time(&self, when: NaiveDateTime) -> bool {
        self.post == when
    }

    fn is_post_active(&self, last: NaiveDateTime) -> bool {
        self.post > last
    }
}

#[derive(Default, Debug, Deserialize, PartialEq, PartialOrd)]
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

#[derive(Debug, Deserialize, PartialEq, PartialOrd)]
pub(crate) struct EventMessage {
    pub(crate) additional: Option<String>,
    pub(crate) main: String,
}

#[derive(Debug, Deserialize, PartialEq, PartialOrd)]
pub(crate) struct Event {
    pub(crate) message: EventMessage,
    pub(crate) priority: Option<EventPriority>,
    pub(crate) time: EventTime,
}

impl Event {
    fn notify(&self) {
        let mut basic_notif = Notification::new().summary(&self.message.main).clone();
        debug!("create basic notification is {:#?}", &basic_notif);

        let is_set_additional = &self.message.additional.is_some();
        debug!("took state of additional is {:#?}", is_set_additional);
        let is_set_priority = &self.priority.is_some();
        debug!("took state of priority is {:#?}", is_set_additional);

        unsafe {
            match (is_set_additional, is_set_priority) {
                // SAFETY: both additionoal and priorities are presented
                (true, true) => {
                    let additional = &self.message.additional.as_ref().unwrap_unchecked();
                    let priority = &self.priority.as_ref().unwrap_unchecked();
                    basic_notif.body(additional).urgency(priority.urgency());
                    debug!("both states adopted");
                }
                // SAFETY: additional is presented, priority is not
                (true, false) => {
                    let additional = &self.message.additional.as_ref().unwrap_unchecked();
                    basic_notif
                        .body(additional)
                        .urgency(EventPriority::default().urgency());
                    debug!("additional state only adopted");
                }
                // SAFETY: additional is not presented, priority is
                (false, true) => {
                    let priority = &self.priority.as_ref().unwrap_unchecked();
                    basic_notif.urgency(priority.urgency());
                    debug!("priority state only adopted");
                }
                // SAFETY: both additional and priority are not presented
                (false, false) => {
                    basic_notif.urgency(EventPriority::default().urgency());
                    debug!("no state adopted");
                }
            }
        }

        basic_notif.show().unwrap_or_else(|err| {
            let msg = format!("failed to create notification because of {err}");
            EventLogging::panic(&msg);
        });
        info!("created finished notification instance");
        info!("finished notification instance is {:#?}", &basic_notif);
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

    pub(crate) fn start(&mut self, sleep_time: Duration) {
        let mut now: NaiveDateTime;

        loop {
            now = Local::now().naive_local();
            info!("got current timestamp");
            debug!("current timestamp is {:#?}", &now);

            for event in self.event.iter() {
                if event.time.is_post_time(now) {
                    event.notify();
                    info!("notified event last time due to post time");
                    debug!("notified post time event is {:#?}", &event);
                    continue;
                } else {
                    debug!("post time is not fine");
                }

                if let Some(warn) = &event.time.warn {
                    if warn.is_time(now) && event.time.is_post_active(now) {
                        event.notify();
                        info!("notified event due to warn time");
                        debug!("notified warn time event is {:#?}", &event);
                        continue;
                    } else if event.time.is_post_active(now) {
                        debug!("warn time is not fine but post is active");
                    } else {
                        debug!("warn time is not fine and post is inactive");
                    }
                }
            }

            thread::sleep(sleep_time);
            thread::yield_now();
        }
    }

    // fn delete(&mut self, event: &Event) {
    //     let idx = self
    //         .event
    //         .iter()
    //         .position(|inner_event| event == inner_event);

    //     if let Some(idx) = idx {
    //         self.event.remove(idx);
    //         info!("delete expired event");
    //         debug!("delete expired event with data {:#?}", event);
    //     } else {
    //         EventLogging::panic("failed to find event index to be deleted");
    //     }
    // }
}
