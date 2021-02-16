use crate::management::data::RenderFrame;
use crate::management::flags::Flags;
use crate::constants::*;

use std::time::Duration;
use std::process::exit;

use chrono::{Local, DateTime};
use tokio::sync::broadcast::*;
use tokio::time::{sleep, Instant};
use tui::Terminal;
use tui::backend::CrosstermBackend;
use std::io::Stdout;
use crossterm::event::EventStream;

pub mod views;


pub async fn spawn(mut s: Sender<RenderFrame>, mut r: Receiver<RenderFrame>, flags: Flags) {
    tokio::spawn(async move {
        let mut reader = EventStream::new();
        let stdout = std::io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal: Terminal<CrosstermBackend<Stdout>> = Terminal::new(backend).unwrap();
        let tmp_dir_opt = std::env::temp_dir();
        let mut tmp_dir = tmp_dir_opt.to_str().unwrap().to_string();
        let tree = tmp_dir.split("\\").collect::<Vec<&str>>();

        tmp_dir = tree.join("/");

        if tmp_dir.chars().last().unwrap() != '/' {
            tmp_dir = format!("{}/", tmp_dir);
        }

        let launch_path = format!("{}launches.nlx", tmp_dir);

        let mut time_since_payload = Instant::now();

        let mut file_ok = true;

        let mut frame_count: u16 = 0;

        loop {
            frame_count += 1;
            // let inc_keys = reader.poll_next();
            let inc_res = r.try_recv();

            // if

            if let Ok(payload) = inc_res {
                match payload.view {
                    0 => {
                        views::regular::process(Some(payload), &mut terminal).await
                    }
                    _ => {
                        eprintln!("Unknown viewing mode, fatal error");
                        exit(1)
                    }
                }
            }
            sleep(Duration::from_millis(500)).await;
        }
    });
}