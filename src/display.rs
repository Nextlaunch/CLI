extern crate image;

use chrono::{DateTime, NaiveDateTime, Utc, Local};
use self::image::imageops::resize;
use proctitle::set_title;

use crate::braille::ToBraille;

pub struct Display {
    image: image::GrayImage,
    char_width: u32,
    char_height: u32,
}

impl Display {
    pub fn new(img: image::GrayImage, width: u32, height: u32) -> Display {
        Display {
            image: resize(&img, width * 2, height * 4, image::Lanczos3),
            char_width: width,
            char_height: height,
        }
    }

    pub fn render(&self, launch: crate::structure::Launch) -> String {
        let mut target = Vec::<String>::new();
        let vehicle = launch.rocket.unwrap();
        let v_config = vehicle.configuration.unwrap();
        let provider = launch.launch_service_provider.unwrap();
        let pad = launch.pad.unwrap();

        for y in 0..self.char_height {
            for x in 0..self.char_width {
                target.push(self.braillify_block(x, y).to_string());
            }
            if y == 0 {
                if launch.mission.is_some() {
                    target.push(format!("\t\tMission: {}", launch.mission.clone().unwrap().name.clone().unwrap()));
                    set_title(format!("NextLaunch - {}", launch.mission.clone().unwrap().name.clone().unwrap()));
                } else {
                    let mut name = launch.name.clone().unwrap();
                    let name_vec = name.split(" | ").collect::<Vec<&str>>();
                    name = name_vec.last().unwrap().to_string();
                    target.push(format!("\t\tMission: {}", name));
                    set_title(format!("NextLaunch - {}", name));
                }
            } else if y == 1 {
                if launch.status.abbrev.clone().unwrap() == "TBD".to_string() {
                    target.push("\x1b[33m".to_string());
                } else if launch.status.abbrev.clone().unwrap() == "Go".to_string() {
                    target.push("\x1b[32m".to_string());
                }
                target.push(format!("\t\tStatus: {}\x1b[0m", launch.status.name.clone().unwrap()))
            } else if y == 2 {
                let (days, hours, minutes, seconds) = countdown(launch.net.clone().unwrap().as_str());
                if launch.status.abbrev.clone().unwrap() == "TBD".to_string() {
                    target.push("\x1b[33m".to_string());
                } else if launch.status.abbrev.clone().unwrap() == "Go".to_string() {
                    target.push("\x1b[32m".to_string());
                }
                target.push(format!("\t\tCountdown: T - {} Days, {} Hours, {} Minutes, {} Seconds\x1b[0m", days, hours, minutes, seconds))
            } else if y == 4 {
                target.push(format!("\t\tLaunch Vehicle: {}", v_config.name.clone().unwrap()))
            } else if y == 5 {
                target.push(format!("\t\tProvider: {}", provider.name.clone().unwrap()))
            } else if y == 7 {
                let (days, hours, minutes, seconds) = countdown(launch.window_start.clone().unwrap().as_str());
                if launch.status.abbrev.clone().unwrap() == "TBD".to_string() {
                    target.push("\x1b[33m".to_string());
                } else if launch.status.abbrev.clone().unwrap() == "Go".to_string() {
                    target.push("\x1b[32m".to_string());
                }
                target.push(format!("\t\tWindow Open: T - {} Days, {} Hours, {} Minutes, {} Seconds\x1b[0m", days, hours, minutes, seconds))
            } else if y == 8 {
                let (days, hours, minutes, seconds) = countdown(launch.window_end.clone().unwrap().as_str());
                if launch.status.abbrev.clone().unwrap() == "TBD".to_string() {
                    target.push("\x1b[33m".to_string());
                } else if launch.status.abbrev.clone().unwrap() == "Go".to_string() {
                    target.push("\x1b[32m".to_string());
                }
                target.push(format!("\t\tWindow Close: T - {} Days, {} Hours, {} Minutes, {} Seconds\x1b[0m", days, hours, minutes, seconds))
            } else if y == 10 {
                target.push(format!("\t\tLocation: {}", pad.name.clone().unwrap()))
            } else if y == 11 {
                target.push(format!("\t\tCountry: {}", pad.location.name.clone().unwrap()))
            }
            target.push('\n'.to_string());
        }

        let mut final_str = String::new();

        for line in target {
            final_str = format!("{}{}", final_str, line)
        }

        final_str
    }

    fn braillify_block(&self, x: u32, y: u32) -> char {
        let mut dot_map = 0b0000_0000;
        for i in 0..8 {
            let abs_x = (x * 2) + (i % 2);
            let abs_y = (y * 4) + (i / 2);

            dot_map |= if self.sample(abs_x, abs_y) {
                0b1000_0000 >> i
            } else {
                0
            };
        }
        dot_map.to_braille()
    }

    fn sample(&self, x: u32, y: u32) -> bool {
        self.image[(x, y)][0] < 128
    }
}

fn countdown(timestamp: &str) -> (i32, i32, i32, i64) {
    let scheduled_naive = NaiveDateTime::parse_from_str(timestamp, "%Y-%m-%dT%H:%M:%SZ").unwrap();
    let scheduled = DateTime::<Utc>::from_utc(scheduled_naive, Utc).signed_duration_since(Utc::now());
    let mut seconds = scheduled.num_seconds();
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