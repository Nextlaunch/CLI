use crate::management::data::launches::{LaunchAPI, process_incoming};
use tokio::sync::broadcast::*;
use tokio::time::sleep;
use std::time::{Instant, Duration};
use crate::management::flags::Flags;

pub async fn spawn(mut s: Sender<LaunchAPI>, mut r: Receiver<LaunchAPI>, flags: Flags) {
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



        loop {
            let inc_res = r.try_recv();

            if let Ok(payload) = inc_res {
                process_incoming(payload);
            }
            sleep(Duration::from_millis(500)).await;
        }
    });
}