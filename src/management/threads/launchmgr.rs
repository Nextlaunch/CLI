use crate::management::data::launches::{LaunchAPI, LaunchAPIop, LaunchCache, process_incoming};
use tokio::sync::broadcast::*;
use tokio::time::sleep;
use std::time::{Instant, Duration};
use crate::management::flags::Flags;
use tokio::fs::OpenOptions;
use chrono::{Local, DateTime};
use crate::{FG_RED, RESET, NAME};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

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

        let f_o = OpenOptions::new()
            .create(true)
            .write(true)
            .read(true)
            .append(false)
            .open(&launch_path).await;

        let mut file_ok = true;

        if let Ok(mut cache_file) = f_o {
            let mut timestamp: DateTime<Local> = Local::now();
            let mut launches = LaunchCache { launches: vec![], last_checked: timestamp.to_string() };
            let mut l_vec: Vec<u8> = vec!();
            let res = cache_file.read_to_end(&mut l_vec).await;

            if let Err(reason) = res {
                println!("{}FATAL EXCEPTION:{} Unable to read local data cache.\n{}", FG_RED, RESET, reason);
            } else {

                let parse_res: bincode::Result<LaunchCache>  = bincode::deserialize(l_vec.as_slice());

                println!("{}", launch_path);

                if let Ok(cache) = parse_res {
                    launches = cache;
                } else {
                    println!("{}EXCEPTION:{} Unable to parse local data cache.\n{}\nThe program will attempt to recover, by writing over the contents of the file\n{} will now halt for 5 minutes before writing the new contents", FG_RED, RESET, parse_res.unwrap_err(), NAME);
                    s.send(LaunchAPI::new(LaunchAPIop::HALT));
                    sleep(Duration::from_secs(5*60)).await;
                    println!("five minutes has passed, Next Launch is now writing the content of the cache file");
                    cache_file.write_all(bincode::serialize(&launches).unwrap().as_slice()).await;
                    cache_file.flush().await;
                }

                dbg!(&launches);

                loop {
                    let inc_res = r.try_recv();

                    if let Ok(payload) = inc_res {
                        process_incoming(payload, &mut cache_file);
                    }
                    sleep(Duration::from_millis(500)).await;
                }

            }
        } else {
            println!("{}FATAL EXCEPTION:{} Unable to open or create local data cache.", FG_RED, RESET);
        }
    });
}