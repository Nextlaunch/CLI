extern crate image;

use std::process;
use display::Display;
use proctitle::set_title;
use reqwest::blocking::ClientBuilder;
use std::time::Duration;
use std::io::{Write, Read};
use bytes::Bytes;
use crate::structure::Launch;

mod structure;
mod display;
mod braille;

fn main() {
    let mut previous_launch: Option<Launch> = None;
    let tmp_dir_opt = std::env::temp_dir();
    let tmp_dir = tmp_dir_opt.to_str().unwrap();
    let tmp_image_path = format!("{}NextLaunch.thumb.png", tmp_dir);
    let temp_image = tmp_image_path.as_str();
    let mut counter = 0;
    let client = ClientBuilder::new()
        .connect_timeout(Duration::from_millis(5000))
        .timeout(Duration::from_millis(5000))
        .user_agent("NextLaunch Command Line Client")
        .build()
        .unwrap();
    loop {
        counter += 1;
        let response = client.get("https://spacelaunchnow.me/api/3.3.0/launch/upcoming/?format=json").send();
        if response.is_ok() {
            let body = response.unwrap();
            let json: structure::LaunchResponse = body.json().unwrap();
            if json.results.is_some() {
                let next = json.results.unwrap().first().unwrap().clone();
                let mut meta = next.clone();
                if meta.image_url.is_none() {
                    meta.image_url = Some("https://spacelaunchnow-prod-east.nyc3.cdn.digitaloceanspaces.com/static/home/img/launcher.png".to_string());
                }

                let image_fetch = client.get(meta.image_url.unwrap().as_str()).send().unwrap().bytes().unwrap();
                if std::fs::File::open(temp_image).is_err() {
                    let mut file = std::fs::File::create(temp_image).unwrap();
                    file.write(image_fetch.as_ref()).unwrap();
                } else {
                    std::fs::remove_file(temp_image);
                    let mut file = std::fs::File::create(temp_image).unwrap();
                    file.write(image_fetch.as_ref()).unwrap();
                }
                previous_launch = Some(next.clone());
                process_image(temp_image, next);
            } else {
                if previous_launch.is_some() {
                    process_image(temp_image, previous_launch.clone().unwrap())
                } else {
                    print!("\rUnable to connect to the internet")
                }
            }
        } else {
            if previous_launch.is_some() {
                process_image(temp_image, previous_launch.clone().unwrap())
            } else {
                print!("\rUnable to connect to the internet")
            }
        }
        std::thread::sleep(Duration::from_secs(10))
    }
    std::fs::remove_file(temp_image);
}


fn process_image(path: &str, launch: structure::Launch) {
    let img = match image::open(path) {
        Ok(image) => image.to_luma(),
        Err(_) => {
            println!("Couldn't open image!");
            process::exit(1);
        }
    };

    let (naive_width, naive_height) = img.dimensions();
    let desired_width: Option<u32> = Some(30);
    let desired_height: Option<u32> = Some(10);

    let (width, height) = match (desired_width, desired_height) {
        (None, None) => (naive_width / 10, naive_height / 20),
        (Some(w), None) => (w, ((naive_height * w) / naive_width) / 2),
        (None, Some(h)) => (((naive_width * h) / naive_height) * 2, h),
        (Some(w), Some(h)) => (w, h)
    };

    let display = Display::new(img, width, height);

    print!("\x1B[2J\x1B[1;1H");
    print!("{}", display.render(launch));
}