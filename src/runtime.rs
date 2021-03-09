use crate::runtime::data::launches::structures::{Launch, Article};
use crate::runtime::data::launches::{update, news_update};
use crate::runtime::flags::Flags;

use tokio::time::{Instant, Duration};

use crossterm::event::{KeyCode, Event, poll, read};

use chrono::{DateTime, Local};
use std::sync::{Arc, Mutex};
use crate::languages::select_language;


pub mod flags;
pub mod data;
pub mod renderer;

pub mod keybindings;
// pub mod json_subsystem;

pub async fn launch(f: Flags) {
    match f.view {
        // 1 => launch_json().await,
        _ => launch_main().await,
    }
}

pub fn print(body: String) {
    println!("{}", body);
}


pub async fn launch_main() {
    crossterm::terminal::enable_raw_mode();
    let mut stdout = std::io::stdout();


    let mut language = select_language();

    renderer::process(
        &crate::languages::en_gb::PACK,
        &None,
        &None,
        &mut vec![
            (Local::now(), "fetching information, please wait".to_string(), 2)
        ],
        true,
        0,
        0,
        0,
        0,
        false,
        false,
    ).await;

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
    let mut view_screen: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
    let mut selected_article: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
    let mut selected_update: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
    let mut selected_side: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
    let mut should_clear: Arc<Mutex<bool>> = Arc::new(Mutex::new(true));
    let mut render_help: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));

    let mut launch_update_count: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
    let mut open_selected: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
    let mut news_article_count: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));

    if launch.is_some() {
        let tpl = launch.clone().unwrap();
        *should_clear.lock().unwrap() = true;
        *launch_update_count.lock().unwrap() = tpl.updates.unwrap_or(vec![]).len() as i32;
    }

    if news.is_some() {
        let tpn = news.clone().unwrap();
        *should_clear.lock().unwrap() = true;
        *news_article_count.lock().unwrap() = tpn.len() as i32;
    }

    let mut view_screen2: Arc<Mutex<i32>> = view_screen.clone();
    let mut selected_article2: Arc<Mutex<i32>> = selected_article.clone();
    let mut selected_update2: Arc<Mutex<i32>> = selected_update.clone();
    let mut selected_side2: Arc<Mutex<i32>> = selected_side.clone();
    let mut should_clear2: Arc<Mutex<bool>> = should_clear.clone();
    let mut open_selected2: Arc<Mutex<bool>> = open_selected.clone();
    let mut render_help2: Arc<Mutex<bool>> = render_help.clone();

    let mut launch_update_count2: Arc<Mutex<i32>> = launch_update_count.clone();
    let mut news_article_count2: Arc<Mutex<i32>> = news_article_count.clone();

    keybindings::launch_thread(
        view_screen2,
        selected_article2,
        selected_update2,
        selected_side2,
        should_clear2,
        open_selected2,
        render_help2,
        launch_update_count2,
        news_article_count2,
    );


    loop {
        refresh_cycle += 1;

        if needs_refresh {
            let temp_launch = update(&client, &mut log).await;
            let temp_news = news_update(&client, &mut log).await;
            if temp_launch.is_some() {
                let tpl = temp_launch.clone().unwrap();
                *should_clear.lock().unwrap() = true;
                *launch_update_count.lock().unwrap() = tpl.updates.unwrap_or(vec![]).len() as i32;
                launch = temp_launch;
            }
            if temp_news.is_some() {
                let tpn = temp_news.clone().unwrap();
                *should_clear.lock().unwrap() = true;
                *news_article_count.lock().unwrap() = tpn.len() as i32;
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


        if refresh_cycle >= 2 {
            log.push((Local::now(), "Press F1 to open the help menu.".to_string(), 10));
            refresh_cycle = 0;
            let (w2, h2) = if let Some((w1, h1)) = term_size::dimensions() {
                (w1, h1)
            } else {
                (0, 0)
            };

            renderer::process(
                &crate::languages::en_gb::PACK,
                &launch,
                &news,
                &mut log,
                w != w2 || h != h2 || *should_clear.lock().unwrap(),
                *view_screen.lock().unwrap(),
                *selected_side.lock().unwrap(),
                *selected_article.lock().unwrap(),
                *selected_update.lock().unwrap(),
                *open_selected.lock().unwrap(),
                *render_help.lock().unwrap()
            ).await;

            w = w2;
            h = h2;

            if *should_clear.lock().unwrap() {
                *should_clear.lock().unwrap() = false;
            }

            if *open_selected.lock().unwrap() {
                *open_selected.lock().unwrap() = false;
            }

            log.pop();
        }


        if last.elapsed().as_secs() > 60 * 10 {
            last = Instant::now();
            needs_refresh = true;
        }

        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}