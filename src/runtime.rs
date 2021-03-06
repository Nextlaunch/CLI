use crate::runtime::flags::Flags;
use tokio::time::{Instant, Duration};
use crate::runtime::data::launches::structures::{Launch, Article};
use crate::runtime::data::launches::{update, news_update};
use crossterm::terminal::ClearType;
use crossterm::ExecutableCommand;
use chrono::{DateTime, Local};

pub mod flags;
pub mod data;
pub mod renderer;

pub async fn launch(f: Flags) {
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

    let mut needs_refresh = false;

    for _ in 0..50 {
        println!();
    }

    loop {
        if needs_refresh {
            launch = update(&client, &mut log).await;
            news = news_update(&client, &mut log).await;
            needs_refresh = false;
        }

        let (w2, h2) = if let Some((w1, h1)) = term_size::dimensions() {
            (w1, h1)
        } else {
            (0, 0)
        };

        renderer::process(&launch, &news, &mut log, (w != w2 || h != h2)).await;

        w = w2;
        h = h2;

        if last.elapsed().as_secs() > 60 * 10 {
            last = Instant::now();
            needs_refresh = true;
        }

        tokio::time::sleep(Duration::from_millis(1000)).await;
    }
}