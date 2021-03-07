use crate::runtime::data::launches::structures::{Launch, Article};
use crate::runtime::data::launches::{update, news_update};
use crate::runtime::flags::Flags;

use tokio::time::{Instant, Duration};

use crossterm::event::{KeyCode, Event, KeyModifiers, poll, read};

use chrono::{DateTime, Local};


pub mod flags;
pub mod data;
pub mod renderer;

pub async fn launch(_f: Flags) {
    let client = reqwest::Client::new();
    let mut last = Instant::now();

    let (mut w, mut h) = if let Some((w1, h1)) = term_size::dimensions() {
        (w1, h1)
    } else {
        (0, 0)
    };

    let mut log: Vec<(DateTime<Local>, String, u8)> = vec![];

    let mut launch: Option<Launch> = update(&client, &mut log).await;
    let mut news: Option<Vec<Article>> = news_update(&client, &mut log).await;

    if launch.is_some() && news.is_some() {
        log.push((Local::now(), "updated launch and news caches".to_string(), 0));
    } else if launch.is_some() && news.is_none() {
        log.push((Local::now(), "updated launch cache".to_string(), 0));
    } else if launch.is_none() && news.is_some() {
        log.push((Local::now(), "updated news cache".to_string(), 0));
    }

    let mut needs_refresh = false;

    for _ in 0..50 {
        println!();
    }

    let mut refresh_cycle: u8 = 0;
    let mut view_screen: i32 = 0;

    let mut should_clear = false;

    loop {
        match poll(Duration::from_millis(250)) {
            Ok(is_ready) => {
                if is_ready {
                    let raw_event = read();
                    if let Ok(event) = raw_event {
                        match event {
                            Event::Key(raw_key) => {
                                match raw_key.code {
                                    KeyCode::Char(c) => {
                                        match c {
                                            '1' => {
                                                view_screen = 0;
                                                should_clear = true;
                                            }
                                            '2' => {
                                                view_screen = 1;
                                                should_clear = true;
                                            }
                                            _ => {}
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
            Err(_) => {}
        }

        refresh_cycle += 1;

        if needs_refresh {
            let temp_launch = update(&client, &mut log).await;
            let temp_news = news_update(&client, &mut log).await;
            if temp_launch.is_some() {
                launch = temp_launch;
            }
            if temp_news.is_some() {
                news = temp_news;
            }
            if launch.is_some() && news.is_some() {
                log.push((Local::now(), "updated launch and news caches".to_string(), 0));
            } else if launch.is_some() && news.is_none() {
                log.push((Local::now(), "updated launch cache".to_string(), 0));
            } else if launch.is_none() && news.is_some() {
                log.push((Local::now(), "updated news cache".to_string(), 0));
            }
            needs_refresh = false;
        }


        if refresh_cycle >= 4 {
            refresh_cycle = 0;
            let (w2, h2) = if let Some((w1, h1)) = term_size::dimensions() {
                (w1, h1)
            } else {
                (0, 0)
            };
            renderer::process(&launch, &news, &mut log, w != w2 || h != h2 || should_clear, view_screen).await;
            w = w2;
            h = h2;
            if should_clear {
                should_clear = false;
            }
        }


        if last.elapsed().as_secs() > 60 * 10 {
            last = Instant::now();
            needs_refresh = true;
        }

        // tokio::time::sleep(Duration::from_millis(250)).await;
    }
}