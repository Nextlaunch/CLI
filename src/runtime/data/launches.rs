pub use structures::{LaunchAPI, LaunchAPIop, LaunchCache};


use reqwest::Client;
use chrono::{Utc, DateTime, Local};
// use std::process::exit;

pub mod structures;

pub async fn update(c: &Client, logs: &mut Vec<(DateTime<Local>, String, u8)>, token: String) -> Option<structures::Launch> {
    let req = if token.len() == 0 {
        c.get(crate::constants::LAUNCH_API).send().await
    } else {
        c.get(crate::constants::LAUNCH_API).header("Authorization", format!("Token {}", token)).send().await
    };

    return if let Ok(resp) = req {
        let raw_launch: reqwest::Result<structures::LaunchResponse> = resp.json().await;
        if let Ok(launches) = raw_launch {
            if launches.results.is_some() {
                let launch_list = launches.results.unwrap();

                let mut next = launch_list.first().unwrap().clone();
                let previous = crate::utilities::countdown(next.net.clone().unwrap_or(Utc::now().to_string()));
                for launch in launch_list {
                    let time_remaining = crate::utilities::countdown(launch.net.clone().unwrap_or(Utc::now().to_string()));
                    if previous.has_passed {
                        match launch.status.id.clone().unwrap() {
                            1 => {
                                if time_remaining.total_seconds < (30 * 60) {
                                    next = launch;
                                }
                            }
                            2 | 5 | 6 | 8 => {
                                if (previous.total_seconds > (20 * 60) && previous.has_passed) && (time_remaining.total_seconds < (30 * 60) && !time_remaining.has_passed) {
                                    next = launch;
                                } else if previous.total_seconds < (30 * 60) && previous.has_passed {
                                    next = launch;
                                } else if time_remaining.total_seconds < (15 * 60) && !time_remaining.has_passed {
                                    next = launch;
                                }
                            }
                            _ => {}
                        }
                    }
                };
                // let time_remaining = crate::utilities::countdown(next.net.clone().unwrap_or(Utc::now().to_string()));
                // println!("Selecting launch - {} in {} sec (passed: {})", next.name.clone().unwrap(), time_remaining.total_seconds, time_remaining.has_passed);
                // exit(0);

                Some(next)
            } else {
                if launches.detail.is_some() {
                    logs.push((Local::now(), "Failed to update launch cache".to_string(), 1));
                    logs.push((Local::now(), " ^--> Request throttled by API".to_string(), 1));
                } else {
                    logs.push((Local::now(), "Failed to update launch cache".to_string(), 1));
                    logs.push((Local::now(), " ^--> Unknown error".to_string(), 1));
                }
                None
            }
        } else {
            logs.push((Local::now(), "Failed to update launch cache".to_string(), 1));
            None
        }
    } else {
        logs.push((Local::now(), "Failed to update launch cache".to_string(), 1));
        None
    };
}

pub async fn news_update(c: &Client, logs: &mut Vec<(DateTime<Local>, String, u8)>) -> Option<Vec<structures::Article>> {
    let req = c.get(crate::constants::NEWS_API).send().await;

    return if let Ok(resp) = req {
        let raw_launch: reqwest::Result<Vec<structures::Article>> = resp.json().await;
        if let Ok(launches) = raw_launch {
            Some(launches)
        } else {
            logs.push((Local::now(), "Failed to update news cache".to_string(), 1));
            None
        }
    } else {
        logs.push((Local::now(), "Failed to update news cache".to_string(), 1));
        None
    };
}