use std::fmt::{Display, Formatter, Result};
use chrono::{DateTime, NaiveDateTime, Utc};

pub mod map_lat_long;
pub mod map_weather;
pub mod digit_map;

pub fn countdown(timestamp: String) -> TimeFrame {
    let scheduled_naive = NaiveDateTime::parse_from_str(timestamp.as_str(), "%Y-%m-%dT%H:%M:%SZ").unwrap();
    let scheduled = DateTime::<Utc>::from_utc(scheduled_naive, Utc).signed_duration_since(Utc::now());
    process_seconds(scheduled.num_seconds())
}

pub fn countdown_news(timestamp: String) -> TimeFrame {
    let scheduled_naive = NaiveDateTime::parse_from_str(timestamp.as_str(), "%Y-%m-%dT%H:%M:%S%.3fZ").unwrap();
    let scheduled = DateTime::<Utc>::from_utc(scheduled_naive, Utc).signed_duration_since(Utc::now());
    process_seconds(scheduled.num_seconds())
}

pub fn process_seconds(mut seconds: i64) -> TimeFrame {
    let mut minutes = 0;
    let mut hours = 0;
    let mut days = 0;
    let mut weeks = 0;
    let mut years = 0;

    while seconds >= 60 {
        minutes += 1;

        if minutes >= 60 {
            minutes -= 60;
            hours += 1
        }

        if hours >= 24 {
            hours -= 24;
            days += 1;
        }

        if days >= 7 {
            days -= 7;
            weeks += 1;
        }

        if weeks >= 52 {
            weeks -= 52;
            years += 1;
        }

        seconds -= 60;
    }

    while seconds < 0 {
        minutes += 1;

        if minutes >= 60 {
            minutes -= 60;
            hours += 1
        }

        if hours >= 24 {
            hours -= 24;
            days += 1;
        }

        if days >= 7 {
            days -= 7;
            weeks += 1;
        }

        if weeks >= 52 {
            weeks -= 52;
            years += 1;
        }

        seconds += 60;
    }

    TimeFrame::new(seconds as u8, minutes, hours, days, weeks, years)
}

pub struct TimeFrame {
    pub seconds: u8,
    pub minutes: u8,
    pub hours: u8,
    pub days: u8,
    pub weeks: u8,
    pub years: u8,
}

impl TimeFrame {
    pub fn new(s: u8, m: u8, h: u8, d: u8, w: u8, y: u8) -> TimeFrame {
        TimeFrame {
            seconds: s,
            minutes: m,
            hours: h,
            days: d,
            weeks: w,
            years: y,
        }
    }
}

impl Display for TimeFrame {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} years, {} weeks, {} days, {} hours, {} minutes, {} seconds", self.years, self.weeks, self.days, self.hours, self.minutes, self.seconds)
    }
}