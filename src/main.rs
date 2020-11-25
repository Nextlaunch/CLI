extern crate image;

use std::process;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use display::Display;
use proctitle::set_title;
use reqwest::blocking::ClientBuilder;
use std::time::Duration;
use std::io::{Write, Read};
use bytes::Bytes;
use crate::structure::Launch;
use std::process::Command;

mod structure;
mod display;
mod braille;

fn main() {
    let mut previous_launch: Option<Launch> = None;

    let mut counter = 0;

    let client = ClientBuilder::new()
        .connect_timeout(Duration::from_millis(5000))
        .timeout(Duration::from_millis(5000))
        .user_agent("NextLaunch Command Line Client")
        .build()
        .unwrap();
    loop {
        let mut image_path: String = parse_path(previous_launch.clone());
        counter += 1;
        if counter == 1 {
            let response = client.get("https://lldev.thespacedevs.com/2.1.0/launch/upcoming/?format=json").send();
            if response.is_ok() {
                let body = response.unwrap();
                let json: structure::LaunchResponse = body.json().unwrap();
                if json.results.is_some() {
                    let next = json.results.unwrap().first().unwrap().clone();
                    let mut meta = next.clone();
                    if meta.image.is_none() {
                        meta.image = Some("https://spacelaunchnow-prod-east.nyc3.cdn.digitaloceanspaces.com/static/home/img/launcher.png".to_string());
                    }

                    image_path = parse_path(Some(meta.clone()));

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
                } else {
                    if previous_launch.is_some() {
                        process_image(image_path.as_str(), previous_launch.clone().unwrap())
                    } else {
                        print!("\rUnable to connect to the internet")
                    }
                }
            } else {
                if previous_launch.is_some() {
                    process_image(image_path.as_str(), previous_launch.clone().unwrap())
                } else {
                    print!("\rUnable to connect to the internet")
                }
            }
        } else if counter == 10 {
            counter = 0;
            if previous_launch.is_some() {
                process_image(image_path.as_str(), previous_launch.clone().unwrap())
            } else {
                print!("\rUnable to connect to the internet")
            }
        } else {
            if previous_launch.is_some() {
                process_image(image_path.as_str(), previous_launch.clone().unwrap())
            } else {
                print!("\rUnable to connect to the internet")
            }
        }
        std::thread::sleep(Duration::from_millis(250));
    }
}


fn process_image(path: &str, launch: structure::Launch) {
    let img = match image::open(path) {
        Ok(image) => image.to_luma(),
        Err(err) => {
            println!("Couldn't open image!");
            println!("{}", err);
            process::exit(1);
        }
    };

    let (naive_width, naive_height) = img.dimensions();
    let desired_width: Option<u32> = Some(50);
    let desired_height: Option<u32> = Some(14);

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

pub fn parse_path(previous: Option<Launch>) -> String {
    let tmp_dir_opt = std::env::temp_dir();
    let tmp_dir = tmp_dir_opt.to_str().unwrap();

    let mut source: String = "https://spacelaunchnow-prod-east.nyc3.cdn.digitaloceanspaces.com/static/home/img/launcher.png".to_string();

    let mut encoded = String::new();

    if previous.is_some() {
        let pl = previous.clone().unwrap();
        if pl.image.is_some() {
            source = pl.image.unwrap();
        }
        encoded = pl.id.unwrap();
    } else {
        encoded = String::from("39418e8d-d00e-43a4-b58b-b7b1a7335a9b")
    }

    let mut components: Vec<&str> = source.split(".").collect();
    let mut extension: String = components.last().unwrap().clone().to_string();

    return format!("{}{}.{}", tmp_dir, encoded, extension);
}