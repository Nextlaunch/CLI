use std::sync::{Arc, Mutex};
use std::io::Write;

use crate::runtime::data::launches::structures::{Launch, Article};
use crate::runtime::data::launches::{update, news_update};
use crate::runtime::keybindings::keybinder;
use crate::languages::select_language;
use crate::runtime::flags::Flags;
use crate::runtime::state::State;
use crate::settings::Config;

use tokio::time::{Instant, Duration};
use discord_rich_presence::{
    activity::{
        Activity,
        Button,
        // Party,
        Assets
    },
    DiscordIpc,
    DiscordIpcClient
};
use chrono::{DateTime, Local};


pub mod state;
pub mod flags;
pub mod data;
pub mod renderer;


pub mod keybindings;
// pub mod json_subsystem;

pub async fn launch(f: Flags, cfg: Config) {
    match f.view {
        // 1 => launch_json().await,
        _ => {
            if cfg.saved.api_token.len() > 0 {
                let token = cfg.saved.api_token.clone();
                launch_main(cfg, token).await
            } else {
                launch_main(cfg, f.token).await
            }
        }
    }
}

pub async fn launch_main(mut cfg: Config, token: String) {
    let _ = crossterm::terminal::enable_raw_mode();


    let mut language = select_language(&cfg.saved.language);
    let state: Arc<Mutex<State>> = Arc::new(Mutex::new(State::new()));

    let state2 = state.clone();


    std::thread::Builder::new().name("Keybinder".to_string()).spawn(move || {
        keybinder(state2)
    }).unwrap();

    let mut discord = DiscordIpcClient::new("850286429980327936").unwrap();
    let e = discord.connect();
    if e.is_err() {
        println!("RPC Client failed to connect");
        let _ = dbg!(e);
    } else {
        state.lock().unwrap().rpc = true;
    }

    if state.lock().unwrap().rpc {
        let _ = discord.set_activity(Activity::new()
            .details("Initializing")
            .state("Fetching Information")
            .buttons(vec!(
                Button::new("Try It Yourself", "https://github.com/nextlaunch/cli")
            ))
            .assets(
                Assets::new()
                    .small_image("tsd")
                    .small_text("Powered by The Space Devs")
                    .large_image("icon-red")
                    .large_text(format!("Nextlaunch V{}", crate::VERSION).as_str())
            )
        );
    }

    renderer::process(
        &language,
        &None,
        &None,
        &mut vec![
            (Local::now(), "fetching information, please wait".to_string(), 2)
        ],
        true,
        &state,
        &mut cfg,
    ).await;

    let client = reqwest::Client::new();
    let mut last = Instant::now();

    let (mut w, mut h) = if let Some((w1, h1)) = term_size::dimensions() {
        (w1, h1)
    } else {
        (0, 0)
    };

    let mut log: Vec<(DateTime<Local>, String, u8)> = vec![];
    let mut image_path = String::new();

    let _ = image_path;

    let mut launch: Option<Launch> = update(&client, &mut log, token.clone()).await;
    let mut news: Option<Vec<Article>> = news_update(&client, &mut log).await;
    if launch.is_some() && news.is_some() {
        log.push((Local::now(), "updated launch and news caches".to_string(), 0));
    } else if launch.is_some() && news.is_none() {
        log.push((Local::now(), "updated launch cache".to_string(), 0));
    } else if launch.is_none() && news.is_some() {
        log.push((Local::now(), "updated news cache".to_string(), 0));
    }

    let mut needs_refresh = false;

    for _ in 0..h {
        println!();
    }


    let mut refresh_cycle: u8 = 0;


    if launch.is_some() {
        let mut tpl = launch.clone().unwrap();
        state.lock().unwrap().should_clear = true;
        state.lock().unwrap().launch_update_count = tpl.updates.clone().unwrap_or(vec![]).len() as i8;
        image_path = renderer::views::parse_path(launch.clone());
        let raw = client.get(renderer::views::agency_logo(tpl.launch_service_provider.clone().unwrap().id.unwrap())).send().await;
        let raw_image = if let Ok(resp) = raw {
            resp.bytes().await.unwrap()
        } else {
            bytes::Bytes::new()
        };
        if std::fs::File::open(image_path.as_str()).is_err() {
            let mut file = std::fs::File::create(image_path.as_str()).unwrap();
            file.write(raw_image.as_ref()).unwrap();
        } else {
            std::fs::remove_file(image_path.as_str()).unwrap();
            let mut file = std::fs::File::create(image_path.as_str()).unwrap();
            file.write(raw_image.as_ref()).unwrap();
        }
        let img = renderer::views::process_image(image_path.as_str(), tpl.clone());
        tpl.set_logo(img);
        launch = Some(tpl);
    }

    if news.is_some() {
        let tpn = news.clone().unwrap();
        state.lock().unwrap().should_clear = true;
        state.lock().unwrap().news_article_count = tpn.len() as i8;
    }


    loop {
        refresh_cycle += 1;

        if needs_refresh || state.lock().unwrap().needs_update {
            state.lock().unwrap().needs_update = false;
            let temp_launch = update(&client, &mut log, token.clone()).await;
            let temp_news = news_update(&client, &mut log).await;
            if temp_launch.is_some() {
                let mut tpl = temp_launch.clone().unwrap();
                state.lock().unwrap().should_clear = true;
                state.lock().unwrap().launch_update_count = tpl.updates.clone().unwrap_or(vec![]).len() as i8;
                image_path = renderer::views::parse_path(launch.clone());
                let raw = client.get(renderer::views::agency_logo(tpl.launch_service_provider.clone().unwrap().id.unwrap())).send().await;
                let raw_image = if let Ok(resp) = raw {
                    resp.bytes().await.unwrap()
                } else {
                    bytes::Bytes::new()
                };
                if std::fs::File::open(image_path.as_str()).is_err() {
                    let mut file = std::fs::File::create(image_path.as_str()).unwrap();
                    file.write(raw_image.as_ref()).unwrap();
                } else {
                    std::fs::remove_file(image_path.as_str()).unwrap();
                    let mut file = std::fs::File::create(image_path.as_str()).unwrap();
                    file.write(raw_image.as_ref()).unwrap();
                }
                let img = renderer::views::process_image(image_path.as_str(), tpl.clone());
                tpl.set_logo(img);
                launch = Some(tpl);
            }
            if temp_news.is_some() {
                let tpn = temp_news.clone().unwrap();
                state.lock().unwrap().should_clear = true;
                state.lock().unwrap().news_article_count = tpn.len() as i8;
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
                &language,
                &launch,
                &news,
                &mut log,
                w != w2 || h != h2,
                &state,
                &mut cfg,
            ).await;

            w = w2;
            h = h2;

            if state.lock().unwrap().should_clear {
                state.lock().unwrap().should_clear = false;
                language = select_language(&cfg.saved.language);
                if state.lock().unwrap().rpc && launch.is_some() {
                    let l = launch.clone().unwrap();
                    let large = match l.launch_service_provider.clone().unwrap().id.unwrap() {
                        44 => "nasa",
                        121 => "spacex",
                        141 => "blue-origin",
                        147 => "rocketlab",
                        _ => "icon-red"
                    };

                    let _ = discord.set_activity(Activity::new()
                        .details(l.mission.unwrap().name.unwrap_or("Unknown Mission".to_string()).as_str())
                        .state(l.launch_service_provider.unwrap().name.unwrap_or("Unknown Launch Provider".to_string()).as_str())
                        .buttons(vec!(
                            Button::new("Space Launch Now", format!("https://spacelaunchnow.me/launch/{}", l.id.clone().unwrap()).as_str()),
                            Button::new("Go4Liftoff", format!("https://go4liftoff.com/launch/{}", l.id.clone().unwrap()).as_str())
                        ))
                        .assets(
                            Assets::new()
                                .small_image("tsd")
                                .small_text("Powered by The Space Devs")
                                .large_image(large)
                                .large_text(format!("Nextlaunch V{}", crate::VERSION).as_str())
                        )
                    );
                }
            }

            if state.lock().unwrap().open_selected {
                state.lock().unwrap().open_selected = false;
                language = select_language(&cfg.saved.language);
            }

            log.pop();
        }

        if token.len() > 0 {
            if last.elapsed().as_secs() > cfg.saved.cache_update_frequency.clone() as u64 {
                last = Instant::now();
                needs_refresh = true;
            }
        } else {
            state.lock().unwrap().needs_update = false;
            if last.elapsed().as_secs() > 600 as u64 {
                last = Instant::now();
                needs_refresh = true;
            }
        }

        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}
