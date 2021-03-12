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

pub fn process_seconds(s: i64) -> TimeFrame {
    let days = s / (60 * 60 * 24);
    let hours = s % (60 * 60 * 24) / (60 * 60);
    let minutes = s % (60 * 60) / (60);
    let seconds = s % (60);
    TimeFrame::new(seconds, minutes, hours, days, s.is_negative())
}

pub struct TimeFrame {
    pub seconds: u64,
    pub minutes: u64,
    pub hours: u64,
    pub days: u64,
    pub has_passed: bool,
}

impl TimeFrame {
    pub fn new(s: i64, m: i64, h: i64, d: i64, has_passed: bool) -> TimeFrame {
        let seconds = if s.is_negative() {
            (s * -1) as u64
        } else {
            s as u64
        };

        let minutes = if m.is_negative() {
            (m * -1) as u64
        } else {
            m as u64
        };

        let hours = if h.is_negative() {
            (h * -1) as u64
        } else {
            h as u64
        };

        let days = if d.is_negative() {
            (d * -1) as u64
        } else {
            d as u64
        };


        TimeFrame {
            seconds,
            minutes,
            hours,
            days,
            has_passed
        }
    }
}