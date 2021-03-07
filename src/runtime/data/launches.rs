pub use structures::{LaunchAPI, LaunchAPIop, LaunchCache};


use reqwest::Client;
use tokio::fs::File;
use chrono::{Utc, DateTime, Local};

pub mod structures;

pub async fn update(c: &Client, logs: &mut Vec<(DateTime<Local>, String, u8)>) -> Option<structures::Launch> {
    let req = c.get(crate::constants::LAUNCH_API).send().await;

    return if let Ok(resp) = req {
        let raw_launch: reqwest::Result<structures::LaunchResponse> = resp.json().await;
        if let Ok(launches) = raw_launch {
            if launches.results.is_some() {
                let launch_list = launches.results.unwrap();

                let mut next = launch_list.first().unwrap().clone();

                for launch in launch_list {
                    let time_remaining = crate::utilities::countdown(launch.net.clone().unwrap_or(Utc::now().to_string()));

                    if time_remaining.seconds > 0 {
                        next = launch;
                        break;
                    }
                }

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