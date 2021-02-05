use crate::management::rendering::structures::Frame;
use crate::management::data::RenderFrame;
use crate::management::flags::Flags;
use crate::constants::*;

use std::time::Duration;

use chrono::{Local, DateTime};
use tokio::sync::broadcast::*;
use tokio::time::{sleep, Instant};
use crate::management::rendering::render;

pub mod views;


pub async fn spawn(mut s: Sender<RenderFrame>, mut r: Receiver<RenderFrame>, flags: Flags) {
    tokio::spawn(async move {
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

        let mut previous: Option<Frame> = None;
        loop {
            let inc_res = r.try_recv();

            if let Ok(payload) = inc_res {
                let frame = match payload.view {
                    0 => {
                        views::regular::process(payload).await
                    }
                    _ => {
                        Frame {
                            cells: vec![]
                        }
                    }
                };

                render(frame.clone(), previous.clone()).await;
            }
            sleep(Duration::from_millis(500)).await;
        }
    });
}