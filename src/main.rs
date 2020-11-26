extern crate image;

use display::Display;

use reqwest::blocking::{ClientBuilder, Client};

use bytes::Bytes;

use clap::{App, Arg};

use chrono::{DateTime, NaiveDateTime, Utc, Local};

use std::time::{Duration, Instant};
use std::io::{Write, Read};
use std::process::{Command, exit};
use std::process;

use crate::structure::Launch;

mod structure;
mod display;
mod braille;
mod agencies;

fn main() {
    let m = App::new("My Program")
        .author("Thomas Bardsley, tom.b.2k2@gmail.com")
        .version("0.0.2")
        .about("Watch a countdown until the next rocket launch, live in your terminal!!");

    let mut duration = Instant::now();

    let client = ClientBuilder::new()
        .connect_timeout(Duration::from_millis(5000))
        .timeout(Duration::from_millis(5000))
        .user_agent("NextLaunch Command Line Client")
        .build()
        .unwrap();


    let (img, mut previous_launch) = fetch_latest(&client, "https://lldev.thespacedevs.com/2.1.0/launch/upcoming/?format=json&status=1");

    let mut lines: Vec<String> = process_image(img.as_str(), previous_launch.clone().unwrap());

    loop {
        let (mut image_path, prv) = parse_path(previous_launch.clone());

        previous_launch = Some(prv.clone());

        if (duration.elapsed().as_secs() as f32 / 60 as f32) > 5.0 {
            let (path, prev) = fetch_latest(&client, "https://lldev.thespacedevs.com/2.1.0/launch/upcoming/?format=json&status=1");
            image_path = path;
            previous_launch = prev;
            duration = Instant::now();
        }

        if previous_launch.is_some() {
            let mut content = String::new();
            let launch = previous_launch.clone().unwrap();
            let vehicle = launch.rocket.unwrap();
            let v_config = vehicle.configuration.unwrap();
            let provider = launch.launch_service_provider.unwrap();
            let pad = launch.pad.unwrap();

            for y in 0..lines.len() {
                content = format!("{}{}", content, lines[y]);
                if y == 0 {
                    if launch.mission.is_some() {
                        content = format!("{}\t\tMission: {}", content, launch.mission.clone().unwrap().name.clone().unwrap());
                    } else {
                        let mut name = launch.name.clone().unwrap();
                        let name_vec = name.split(" | ").collect::<Vec<&str>>();
                        name = name_vec.last().unwrap().to_string();
                        content = format!("{}\t\tMission: {}", content, name);
                    }
                } else if y == 1 {
                    if launch.status.abbrev.clone().unwrap() == "TBD".to_string() {
                        content = format!("{}\x1b[33m", content);
                    } else if launch.status.abbrev.clone().unwrap() == "Go".to_string() {
                        content = format!("{}\x1b[32m", content);
                    }
                    content = format!("{}\t\tStatus: {}\x1b[0m", content, launch.status.name.clone().unwrap())
                } else if y == 2 {
                    let (days, hours, minutes, seconds) = countdown(launch.net.clone().unwrap().as_str());
                    if launch.status.abbrev.clone().unwrap() == "TBD".to_string() {
                        content = format!("{}\x1b[33m", content);
                    } else if launch.status.abbrev.clone().unwrap() == "Go".to_string() {
                        content = format!("{}\x1b[32m", content);
                    }
                    content = format!("{}\t\tCountdown: T - {} Days, {} Hours, {} Minutes, {} Seconds\x1b[0m", content, days, hours, minutes, seconds)
                } else if y == 4 {
                    content = format!("{}\t\tLaunch Vehicle: {}", content, v_config.name.clone().unwrap())
                } else if y == 5 {
                    content = format!("{}\t\tProvider: {}", content, provider.name.clone().unwrap())
                } else if y == 7 {
                    let (days, hours, minutes, seconds) = countdown(launch.window_start.clone().unwrap().as_str());
                    if launch.status.abbrev.clone().unwrap() == "TBD".to_string() {
                        content = format!("{}\x1b[33m", content);
                    } else if launch.status.abbrev.clone().unwrap() == "Go".to_string() {
                        content = format!("{}\x1b[32m", content);
                    }
                    content = format!("{}\t\tWindow Open: T - {} Days, {} Hours, {} Minutes, {} Seconds\x1b[0m", content, days, hours, minutes, seconds)
                } else if y == 8 {
                    let (days, hours, minutes, seconds) = countdown(launch.window_end.clone().unwrap().as_str());
                    if launch.status.abbrev.clone().unwrap() == "TBD".to_string() {
                        content = format!("{}\x1b[33m", content);
                    } else if launch.status.abbrev.clone().unwrap() == "Go".to_string() {
                        content = format!("{}\x1b[32m", content);
                    }
                    content = format!("{}\t\tWindow Close: T - {} Days, {} Hours, {} Minutes, {} Seconds\x1b[0m", content, days, hours, minutes, seconds)
                } else if y == 10 {
                    content = format!("{}\t\tLocation: {}", content, pad.name.clone().unwrap())
                } else if y == 11 {
                    content = format!("{}\t\tCountry: {}", content, pad.location.name.clone().unwrap())
                } else if y == 19 {
                    let chrondur = Instant::now();
                    let elapsed = chrondur.duration_since(duration.clone());
                    let (_, _, minutes, seconds) = get_time(elapsed.as_secs() as i64);
                    content = format!("{}\t\tTime since last refresh: {} Minutes, {} Seconds", content, minutes, seconds)
                }
                content = format!("{}\n", content);
            }
            print!("\x1B[1;1H");
            print!("{}", content);
            std::thread::sleep(Duration::from_millis(1000));
        } else {
            // print!("\rUnable to connect to the internet")
        }
    }
}

fn fetch_latest(client: &Client, url: &str) -> (String, Option<Launch>) {
    let mut image_path = String::new();
    let mut previous_launch: Option<Launch> = None;
    let response = client.get(url).send();
    if response.is_ok() {
        let body = response.unwrap();
        let json: structure::LaunchResponse = body.json().unwrap();
        if json.results.is_some() {
            let next = json.results.unwrap().first().unwrap().clone();
            let mut meta = next.clone();

            let (impath, mta) = parse_path(Some(meta.clone()));

            meta = mta;
            image_path = impath;

            let image_fetch = client.get(meta.image.unwrap().as_str()).send().unwrap().bytes().unwrap();
            if std::fs::File::open(image_path.as_str()).is_err() {
                let mut file = std::fs::File::create(image_path.as_str()).unwrap();
                file.write(image_fetch.as_ref()).unwrap();
            } else {
                std::fs::remove_file(image_path.as_str());
                let mut file = std::fs::File::create(image_path.as_str()).unwrap();
                file.write(image_fetch.as_ref()).unwrap();
            }
            previous_launch = Some(next.clone());
            process_image(image_path.as_str(), next);
        }
    }

    return (image_path, previous_launch);
}

fn process_image(path: &str, launch: structure::Launch) -> Vec<String> {
    let img = match image::open(path) {
        Ok(image) => image.to_luma(),
        Err(err) => {
            println!("Couldn't open image!");
            println!("{}", err);
            println!("Failed to open: {}", path);
            process::exit(1);
        }
    };

    let (naive_width, naive_height) = img.dimensions();
    let desired_width: Option<u32> = Some(40);
    let desired_height: Option<u32> = Some(20);

    let (width, height) = match (desired_width, desired_height) {
        (None, None) => (naive_width / 10, naive_height / 20),
        (Some(w), None) => (w, ((naive_height * w) / naive_width) / 2),
        (None, Some(h)) => (((naive_width * h) / naive_height) * 2, h),
        (Some(w), Some(h)) => (w, h)
    };

    let display = Display::new(img, width, height);

    let output = if cfg!(target_os = "windows") {
        print!("{}", String::from_utf8_lossy(&*Command::new("cls")
            .output()
            .expect("failed to execute process")
            .stdout));
    } else {
        print!("{}", String::from_utf8_lossy(&*Command::new("clear")
            .output()
            .expect("failed to execute process")
            .stdout));
    };

    let lines = display.render();
    return lines;
}

pub fn parse_path(previous: Option<Launch>) -> (String, Launch) {
    let tmp_dir_opt = std::env::temp_dir();
    let mut tmp_dir = tmp_dir_opt.to_str().unwrap().to_string();

    let mut source: String = "https://www.iconspng.com/images/mono-unknown/mono-unknown.png".to_string();

    let encoded = String::from("logo-nextlaunch-dnf");

    let mut x = previous.clone().unwrap();

    if previous.is_some() {
        let pl = previous.clone().unwrap();
        source = agencies::agency_logo(pl.launch_service_provider.unwrap()).to_string();
        x.image = Some(source.clone());
    }

    let mut components: Vec<&str> = source.split(".").collect();
    let mut extension: String = components.last().unwrap().clone().to_string();

    if tmp_dir.chars().last().unwrap() != '/' {
        tmp_dir = format!("{}/", tmp_dir);
    }
    return (format!("{}{}.{}", tmp_dir, encoded, extension), x);
}

fn countdown(timestamp: &str) -> (i32, i32, i32, i64) {
    let scheduled_naive = NaiveDateTime::parse_from_str(timestamp, "%Y-%m-%dT%H:%M:%SZ").unwrap();
    let scheduled = DateTime::<Utc>::from_utc(scheduled_naive, Utc).signed_duration_since(Utc::now());
    get_time(scheduled.num_seconds())
}

fn get_time(mut seconds: i64) -> (i32, i32, i32, i64) {
    let mut minutes = -1;
    let mut hours = 0;
    let mut days = 0;
    while seconds > 60 {
        if minutes == 59 {
            minutes = 0;
            hours += 1;
        }
        if hours == 23 {
            hours = 0;
            days += 1
        }
        minutes += 1;
        seconds -= 60;
    };
    return (days, hours, minutes, seconds);
}


