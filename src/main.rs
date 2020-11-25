extern crate image;

use std::process;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use display::Display;
use proctitle::set_title;
use reqwest::blocking::{ClientBuilder, Client};
use std::time::{Duration, Instant};
use std::io::{Write, Read};
use bytes::Bytes;
use crate::structure::Launch;
use std::process::Command;

mod structure;
mod display;
mod braille;
mod agencies;

fn main() {
    let mut duration = Instant::now();

    let client = ClientBuilder::new()
        .connect_timeout(Duration::from_millis(5000))
        .timeout(Duration::from_millis(5000))
        .user_agent("NextLaunch Command Line Client")
        .build()
        .unwrap();


    let (_, mut previous_launch) = fetch_latest(&client, "https://lldev.thespacedevs.com/2.1.0/launch/upcoming/?format=json&status=1");

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
            process_image(image_path.as_str(), previous_launch.clone().unwrap())
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

fn process_image(path: &str, launch: structure::Launch) {
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

    print!("\x1B[1;1H");
    print!("{}", display.render(launch));
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

