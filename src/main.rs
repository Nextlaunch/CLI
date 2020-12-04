extern crate image;

use display::Display;

use reqwest::blocking::{ClientBuilder, Client};

use clap::{App, Arg, ArgMatches};

use chrono::{DateTime, NaiveDateTime, Utc};

use std::time::{Duration, Instant};
use std::io::Write;
use std::process::{Command, exit};
use std::process;

use crate::structure::{Launch, Article, LaunchResponse};

mod structure;
mod display;
mod braille;
mod minimal;
mod agencies;

fn main() {
    let m = App::new("Next Launch")
        .author("Thomas Bardsley, tom.b.2k2@gmail.com")
        .version("0.2.2")
        .about("Watch a countdown until the next rocket launch, live in your terminal!!")
        .arg(Arg::new("creds")
            .short('c')
            .long("credits")
            .about("Credits to all of the people who helped with this project")
            .takes_value(false))
        .arg(Arg::new("retro")
            .long("retro")
            .long_about("Adds a more retro feeling to the output of the program.\n\x1b[43m\x1b[30mNOTE:\x1b[0m this mode is more resource intense than the main viewing mode due to rendering constraints")
            .takes_value(false))
        .arg(Arg::new("minimal")
            .long("minimal")
            .long_about("Provides a minimalistic themed version of the program.\n\x1b[43m\x1b[30mNOTE:\x1b[0m this mode is more resource intense than the main viewing mode due to rendering constraints")
            .takes_value(false))
        .arg(Arg::new("nasa")
            .long("nasa")
            .long_about("Provides a NASA countdown clock style output.\n\x1b[43m\x1b[30mNOTE:\x1b[0m this mode is more resource intense than the main viewing mode due to rendering constraintsg")
            .takes_value(false))
        .arg(Arg::new("offline")
            .long("offline")
            .long_about("Forces the program to assume you are offline and read directly from stale data (where available)")
            .takes_value(false))
        .get_matches();

    if m.is_present("creds") {
        println!("NextLaunch - Credits\n\nData Providers:\nNews:      Space Flight News API\nLaunches: The Space Devs\n")
    } else {
        run(m)
    }
}

fn run(flags: ArgMatches) {
    for x in 0..50 {
        for y in 0..80 {
            print!(" ");
        }
        print!("\n");
    }

    println!("\x1b[5m\x1b[32mPlease wait, NextLaunch is fetching updated launch listings\x1b[0m");

    let mut duration = Instant::now();

    let client = ClientBuilder::new()
        .connect_timeout(Duration::from_millis(5000))
        .timeout(Duration::from_millis(5000))
        .user_agent("NextLaunch Command Line Client")
        .build()
        .unwrap();


    let retro_mode = flags.is_present("retro");
    let minimal_mode = flags.is_present("minimal");
    let nasa_mode = flags.is_present("nasa");
    let offline_mode = flags.is_present("offline");

    let mut url: &str = "https://lldev.thespacedevs.com/2.1.0/launch/upcoming/?format=json&mode=detailed&limit=5";

    let (img, mut previous_launch, mut articles, mut offline) = fetch_latest(&client, url, offline_mode);

    let mut lines: Vec<String> = process_image(img.as_str(), previous_launch.clone().unwrap());

    loop {
        let (mut image_path, prv) = parse_path(previous_launch.clone());

        previous_launch = Some(prv.clone());

        if (duration.elapsed().as_secs() as f32 / 60 as f32) > 5.0 {
            let (path, prev, arts, off) = fetch_latest(&client, url, offline_mode);
            image_path = path;
            previous_launch = prev;
            duration = Instant::now();
            articles = arts;
            offline = off;
        }

        if previous_launch.is_some() {
            let mut content = String::new();
            let launch = previous_launch.clone().unwrap();
            if nasa_mode {
                print!("\x1B[1;1H");
                if !cfg!(target_os = "windows") {
                    print!("{}", String::from_utf8_lossy(&*Command::new("clear")
                        .output()
                        .expect("failed to execute process")
                        .stdout));
                }
                let (da, ho, mi, se) = countdown(launch.net.clone().unwrap().as_str(), false);
                let days = crate::minimal::build_matrix(da as i64, 1);
                let hours = crate::minimal::build_matrix(ho as i64, 1);
                let mins = crate::minimal::build_matrix(mi as i64, 1);
                let secs = crate::minimal::build_matrix(se, 1);


                crate::minimal::render(days, hours, mins, secs, se, 1);
                let mut mission = "".to_string();
                if launch.mission.is_some() {
                    mission = format!("{}Mission:          {}", mission, launch.mission.clone().unwrap().name.clone().unwrap());
                } else {
                    let mut name = launch.name.clone().unwrap();
                    let name_vec = name.split(" | ").collect::<Vec<&str>>();
                    name = name_vec.last().unwrap().to_string();
                    mission = format!("{}Mission:          {}", mission, name);
                }
                println!("\n{}{}\x1b[0m", color(launch.status.clone(), "".to_string()), mission);
                println!("{}{}\x1b[0m", color(launch.status.clone(), "".to_string()), format!("Status:           {}", launch.status.clone().name.unwrap()));
                println!("{}{}\x1b[0m", color(launch.status.clone(), "".to_string()), format!("Lauch Vehicle:    {}", launch.rocket.clone().unwrap().configuration.unwrap().name.unwrap()));
                if offline {
                    println!("Note: You are currently offline, this date will update once you have reconnected to the internet.")
                }
                std::thread::sleep(Duration::from_millis(500));
            } else {
                if !minimal_mode {
                    let vehicle = launch.rocket.unwrap();
                    let v_config = vehicle.configuration.unwrap();
                    let provider = launch.launch_service_provider.unwrap();
                    let pad = launch.pad.unwrap();

                    for y in 0..lines.len() {
                        content = format!("{}{}\x1b[0m", content, lines[y]);
                        match y {
                            0 => {
                                if launch.mission.is_some() {
                                    content = format!("{}\t\tMission:        {}", content, launch.mission.clone().unwrap().name.clone().unwrap());
                                } else {
                                    let mut name = launch.name.clone().unwrap();
                                    let name_vec = name.split(" | ").collect::<Vec<&str>>();
                                    name = name_vec.last().unwrap().to_string();
                                    content = format!("{}\t\tMission:        {}", content, name);
                                }
                            }
                            1 => {
                                content = color(launch.status.clone(), content.clone());

                                content = format!("{}\t\tStatus:         {}\x1b[0m", content, launch.status.name.clone().unwrap())
                            }
                            2 => {
                                let (days, hours, minutes, seconds) = countdown(launch.net.clone().unwrap().as_str(), false);

                                content = color(launch.status.clone(), content.clone());

                                content = format!("{}\t\tCountdown:      ", content);

                                if seconds < 0 {
                                    content = format!("{}T +", content);
                                } else {
                                    content = format!("{}T -", content);
                                }

                                if days <= 0 || days > 1 {
                                    content = format!("{} {} days", content, days.abs())
                                } else {
                                    content = format!("{} {} day ", content, days.abs())
                                }

                                if hours <= 0 || hours > 1 {
                                    content = format!("{} {} hours", content, hours.abs())
                                } else {
                                    content = format!("{} {} hour ", content, hours.abs())
                                }

                                if minutes <= 0 || minutes > 1 {
                                    content = format!("{} {} minutes", content, minutes.abs())
                                } else {
                                    content = format!("{} {} minute ", content, minutes.abs())
                                }

                                if seconds <= 0 || seconds > 1 {
                                    content = format!("{} {} seconds   ", content, seconds.abs())
                                } else {
                                    content = format!("{} {} second    ", content, seconds.abs())
                                }

                                content = format!("{}\x1b[0m", content);
                            }
                            4 => {
                                content = format!("{}\t\tLaunch Vehicle: {}", content, v_config.name.clone().unwrap())
                            }
                            5 => {
                                content = format!("{}\t\tProvider:       {}", content, provider.name.clone().unwrap())
                            }
                            7 => {
                                let (days, hours, minutes, seconds) = countdown(launch.window_start.clone().unwrap().as_str(), false);

                                content = color(launch.status.clone(), content.clone());

                                content = format!("{}\t\tWindow Open:    ", content);

                                if seconds < 0 {
                                    content = format!("{}T +", content);
                                } else {
                                    content = format!("{}T -", content);
                                }

                                if days <= 0 || days > 1 {
                                    content = format!("{} {} days", content, days.abs())
                                } else {
                                    content = format!("{} {} day ", content, days.abs())
                                }

                                if hours <= 0 || hours > 1 {
                                    content = format!("{} {} hours", content, hours.abs())
                                } else {
                                    content = format!("{} {} hour ", content, hours.abs())
                                }

                                if minutes <= 0 || minutes > 1 {
                                    content = format!("{} {} minutes", content, minutes.abs())
                                } else {
                                    content = format!("{} {} minute ", content, minutes.abs())
                                }

                                if seconds <= 0 || seconds > 1 {
                                    content = format!("{} {} seconds   ", content, seconds.abs())
                                } else {
                                    content = format!("{} {} second    ", content, seconds.abs())
                                }

                                content = format!("{}\x1b[0m", content);
                            }
                            8 => {
                                let (days, hours, minutes, seconds) = countdown(launch.window_end.clone().unwrap().as_str(), false);

                                content = color(launch.status.clone(), content.clone());

                                content = format!("{}\t\tWindow Close:   ", content);

                                if seconds < 0 {
                                    content = format!("{}T +", content);
                                } else {
                                    content = format!("{}T -", content);
                                }

                                if days <= 0 || days > 1 {
                                    content = format!("{} {} days", content, days.abs())
                                } else {
                                    content = format!("{} {} day ", content, days.abs())
                                }

                                if hours <= 0 || hours > 1 {
                                    content = format!("{} {} hours", content, hours.abs())
                                } else {
                                    content = format!("{} {} hour ", content, hours.abs())
                                }

                                if minutes <= 0 || minutes > 1 {
                                    content = format!("{} {} minutes", content, minutes.abs())
                                } else {
                                    content = format!("{} {} minute ", content, minutes.abs())
                                }

                                if seconds <= 0 || seconds > 1 {
                                    content = format!("{} {} seconds   ", content, seconds.abs())
                                } else {
                                    content = format!("{} {} second    ", content, seconds.abs())
                                }

                                content = format!("{}\x1b[0m", content);
                            }
                            10 => {
                                content = format!("{}\t\tLocation:       {}", content, pad.name.clone().unwrap())
                            }
                            11 => {
                                content = format!("{}\t\tCountry:        {}", content, pad.location.name.clone().unwrap())
                            }
                            12 => {
                                content = format!("{}\t\tLatitude:       {}", content, pad.latitude.clone().unwrap())
                            }
                            13 => {
                                content = format!("{}\t\tLongitude:      {}", content, pad.longitude.clone().unwrap())
                            }
                            15 => {
                                let programs = launch.program.clone().unwrap();
                                let program = programs.first();
                                if program.is_some() {
                                    content = format!("{}\t\tProgram:        {}", content, program.unwrap().clone().name.unwrap())
                                } else {
                                    content = format!("{}\t\tProgram:        None", content);
                                }
                            }
                            17 => {
                                let chrondur = Instant::now();

                                let elapsed = chrondur.duration_since(duration.clone());

                                let (_, _, minutes, seconds) = get_time(elapsed.as_secs() as i64);

                                content = format!("{}\t\tLast Refresh:  ", content);
                                if minutes <= 0 || minutes > 1 {
                                    content = format!("{} {} minutes", content, minutes)
                                } else {
                                    content = format!("{} {} minute", content, minutes)
                                }

                                if seconds <= 0 || seconds > 1 {
                                    content = format!("{} {} seconds   ", content, seconds)
                                } else {
                                    content = format!("{} {} second    ", content, seconds)
                                }
                            }
                            19 => {
                                content = format!("{}\t\tData automatically refreshes every 10 minutes", content);
                            }
                            _ => {}
                        }
                        content = format!("{}\n", content);
                    }

                    if articles.len() > 0 {
                        let ac = articles.first().unwrap().clone();
                        content = format!("{}Title:  {}\nSource: {}\n", content, ac.title.unwrap(), ac.newsSite.unwrap());
                        if cfg!(target_os = "windows") {
                            content = format!("{}Link:   \x1b[36m{}\x1b[0m\n", content, ac.url.unwrap());
                        } else {
                            content = format!("{}Link:   \x1b[34m{}\x1b[0m\n", content, ac.url.unwrap());
                        }
                    } else {
                        content = format!("{}Notice: News articles appear to be unavailable at this time\n", content);
                    }

                    if offline {
                        content = format!("{}Note: You are currently offline, this date will update once you have reconnected to the internet.", content)
                    }


                    if retro_mode {
                        print!("\x1B[1;1H");
                        let lines = content.split("\n").collect::<Vec<&str>>();
                        let mut previous = "";
                        let mut index = 0;
                        for line in lines {
                            print!(" {}", line);
                            previous = line.clone();
                            print!("\n");

                            if index > 0 && index < 20 {
                                print!("\x1b[1F{}\x1b[1E", previous)
                            }
                            index += 1;
                            std::thread::sleep(Duration::from_millis(50));
                        }
                    } else {
                        print!("\x1B[1;1H");
                        if !cfg!(target_os = "windows") {
                            print!("{}", String::from_utf8_lossy(&*Command::new("clear")
                                .output()
                                .expect("failed to execute process")
                                .stdout));
                        }
                        print!("{}\n", content);
                        std::thread::sleep(Duration::from_millis(500));
                    }
                } else {
                    print!("\x1B[1;1H");
                    if !cfg!(target_os = "windows") {
                        print!("{}", String::from_utf8_lossy(&*Command::new("clear")
                            .output()
                            .expect("failed to execute process")
                            .stdout));
                    }
                    let (da, ho, mi, se) = countdown(launch.net.clone().unwrap().as_str(), false);
                    let days = crate::minimal::build_matrix(da as i64, 0);
                    let hours = crate::minimal::build_matrix(ho as i64, 0);
                    let mins = crate::minimal::build_matrix(mi as i64, 0);
                    let secs = crate::minimal::build_matrix(se, 0);


                    crate::minimal::render(days, hours, mins, secs, se, 0);
                    let mut mission = "".to_string();
                    if launch.mission.is_some() {
                        mission = format!("{}Mission:          {}", mission, launch.mission.clone().unwrap().name.clone().unwrap());
                    } else {
                        let mut name = launch.name.clone().unwrap();
                        let name_vec = name.split(" | ").collect::<Vec<&str>>();
                        name = name_vec.last().unwrap().to_string();
                        mission = format!("{}Mission:          {}", mission, name);
                    }
                    println!("\n{}\x1b[0m", mission);
                    println!("{}\x1b[0m", format!("Status:           {}", launch.status.clone().name.unwrap()));
                    println!("{}\x1b[0m", format!("Lauch Vehicle:    {}", launch.rocket.clone().unwrap().configuration.unwrap().name.unwrap()));
                    if offline {
                        println!("Note: You are currently offline, this date will update once you have reconnected to the internet.")
                    }
                    std::thread::sleep(Duration::from_millis(500));
                }
            }
        }
    }
}

fn fetch_latest(client: &Client, url: &str, mut offline: bool) -> (String, Option<Launch>, Vec<Article>, bool) {
    let mut image_path = String::new();
    let mut previous_launch: Option<Launch> = None;

    let tmp_dir_opt = std::env::temp_dir();
    let mut tmp_dir = tmp_dir_opt.to_str().unwrap().to_string();
    let tree = tmp_dir.split("\\").collect::<Vec<&str>>();

    tmp_dir = tree.join("/");

    if tmp_dir.chars().last().unwrap() != '/' {
        tmp_dir = format!("{}/", tmp_dir);
    }

    let launch_response = client.get(url).send();
    let launch_path = format!("{}launches.nlx", tmp_dir);
    if launch_response.is_ok() && !offline {
        let body = launch_response.unwrap();

        let json: structure::LaunchResponse = body.json().unwrap();

        if json.results.is_some() {
            let mut results = json.results.clone().unwrap();
            let mut launches = results.iter();
            let mut next = launches.next().unwrap().clone();
            let mut status = next.status.clone().id.unwrap();
            let (mut days, mut hours, mut minutes, mut seconds) = countdown(next.net.clone().unwrap().as_str(), false);
            while (minutes < 0 || hours < 0 || days < 0) || (status == 3)  {
                next = launches.next().unwrap().clone();
                status = next.status.clone().id.unwrap();
                let (ds, hs, mins, secs) = countdown(next.net.clone().unwrap().as_str(), false);
                seconds = secs;
                minutes = mins;
                hours = hs;
                days = ds;
            }
            let mut meta = next.clone();

            let (impath, mta) = parse_path(Some(meta.clone()));

            meta = mta;
            image_path = impath;

            let image_fetch = client.get(""/*meta.image.unwrap().as_str()*/).send().unwrap().bytes().unwrap();
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

            if std::fs::File::open(launch_path.clone()).is_err() {
                let mut file = std::fs::File::create(launch_path.clone()).unwrap();
                file.write(bincode::serialize::<LaunchResponse>(&json).unwrap().as_slice()).unwrap();
            } else {
                std::fs::remove_file(launch_path.clone());
                let mut file = std::fs::File::create(launch_path.clone()).unwrap();
                file.write(bincode::serialize::<LaunchResponse>(&json).unwrap().as_slice()).unwrap();
            }
        }
    } else {
        if !std::fs::File::open(launch_path.clone()).is_err() {
            let mut file = std::fs::File::open(launch_path.clone()).unwrap();
            let json: structure::LaunchResponse = bincode::deserialize_from(file).unwrap();
            let mut results = json.results.unwrap();
            let mut launches = results.iter();
            let mut next = launches.next().unwrap().clone();
            let (_, _, mut minutes, mut seconds) = countdown(next.net.clone().unwrap().as_str(), false);
            while minutes < -30 {
                next = launches.next().unwrap().clone();
                let (_, _, mins, secs) = countdown(next.net.clone().unwrap().as_str(), false);
                seconds = secs;
                minutes = mins;
            }
            let mut meta = next.clone();

            let (impath, mta) = parse_path(Some(meta.clone()));

            meta = mta;
            image_path = impath;

            previous_launch = Some(next.clone());
            process_image(image_path.as_str(), next);
            offline = true;
        } else {
            println!("Unfortunately, the program has not got any cached data, and requires an internet connection to update itself.\nPlease re-launch the application once an internet connection has been established.");
            exit(1);
        }
    }

    let mut news_request = client.get("https://spaceflightnewsapi.net/api/v2/articles").send();

    let mut articles: Vec<Article> = Vec::new();

    let article_path = format!("{}articles.nlx", tmp_dir);
    if news_request.is_ok() && !offline {
        let article_response: Vec<Article> = news_request.unwrap().json().unwrap();
        articles = article_response.clone();

        if std::fs::File::open(article_path.clone()).is_err() {
            let mut file = std::fs::File::create(article_path.clone()).unwrap();
            file.write(bincode::serialize::<Vec<Article>>(&article_response).unwrap().as_slice()).unwrap();
        } else {
            std::fs::remove_file(article_path.as_str());
            let mut file = std::fs::File::create(article_path.clone()).unwrap();
            file.write(bincode::serialize::<Vec<Article>>(&article_response).unwrap().as_slice()).unwrap();
        }
    } else {
        if !std::fs::File::open(article_path.clone()).is_err() {
            let mut file = std::fs::File::open(article_path.clone()).unwrap();
            articles = bincode::deserialize_from(file).unwrap();
            offline = true;
        } else {
            println!("Unfortunately, the program has not got any cached data, and requires an internet connection to update itself.\nPlease re-launch the application once an internet connection has been established.");
            exit(1);
        }
    }

    return (image_path, previous_launch, articles, offline);
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

    let lines = display.render();
    return lines;
}

pub fn parse_path(previous: Option<Launch>) -> (String, Launch) {
    let tmp_dir_opt = std::env::temp_dir();
    let mut tmp_dir = tmp_dir_opt.to_str().unwrap().to_string();

    let mut source = "https://web.extension.illinois.edu/stain/stains-hi/235.jpg".to_string();

    let encoded = String::from("logo-nextlaunch-dnf");

    let mut x = previous.clone().unwrap();

    if previous.is_some() {
        let pl = previous.clone().unwrap();
        source = agencies::agency_logo(pl.launch_service_provider.unwrap()).to_string();
        x.image = Some(source.clone());
    }

    let mut components: Vec<&str> = source.split(".").collect();
    let mut extension: String = components.last().unwrap().clone().to_string();

    let tree = tmp_dir.split("\\").collect::<Vec<&str>>();

    tmp_dir = tree.join("/");

    if tmp_dir.chars().last().unwrap() != '/' {
        tmp_dir = format!("{}/", tmp_dir);
    }
    return (format!("{}{}.{}", tmp_dir, encoded, extension), x);
}

fn countdown(timestamp: &str, only_secs: bool) -> (i32, i32, i32, i64) {
    let scheduled_naive = NaiveDateTime::parse_from_str(timestamp, "%Y-%m-%dT%H:%M:%SZ").unwrap();
    let scheduled = DateTime::<Utc>::from_utc(scheduled_naive, Utc).signed_duration_since(Utc::now());
    get_remaining(scheduled.num_seconds(), only_secs)
}

fn get_time(mut seconds: i64) -> (i32, i32, i32, i64) {
    get_remaining(seconds, false)
}

fn get_remaining(mut seconds: i64, only_secs: bool) -> (i32, i32, i32, i64) {
    let mut minutes = 0;
    let mut hours = 0;
    let mut days = 0;
    let mut has_passed = false;
    if !only_secs {
        while seconds > 60 {
            if minutes == 59 {
                minutes = 0;
                if hours == 23 {
                    days += 1;
                    hours = 0;
                } else {
                    hours += 1
                }
            } else {
                minutes += 1
            }
            seconds -= 60;
        };
        if seconds == 60 {
            seconds = 0;
            if minutes > -1 {
                if minutes == 59 {
                    minutes = 0;
                    if hours == 23 {
                        days += 1;
                        hours = 0;
                    } else {
                        hours += 1
                    }
                } else {
                    minutes += 1
                }
            }
        }
        while seconds < 0 {
            has_passed = true;
            // println!("{}, {}, {}, {}", days, hours, minutes, seconds);
            if minutes == -59 {
                minutes = 0;
                if hours == -23 {
                    days -= 1;
                    hours = 0;
                } else {
                    hours -= 1
                }
            } else {
                minutes -= 1
            }
            seconds += 60;
        }
        if has_passed {
            seconds = seconds - 60;
            if seconds == -60 {
                seconds = 0;
                if minutes == -59 {
                    minutes = 0;
                    if hours == -23 {
                        days -= 1;
                        hours = 0;
                    } else {
                        hours -= 1
                    }
                } else {
                    minutes -= 1
                }
            }
        }
    };
    return (days, hours, minutes, seconds);
}

fn color(status: structure::Status, content: String) -> String {
    let id = status.id.unwrap();
    return match id {
        1 => format!("{}\x1b[32m", content),
        2 => format!("{}\x1b[33m", content),
        3 => format!("{}\x1b[32m", content),
        4 => format!("{}\x1b[31m\x1b[5m", content),
        5 => format!("{}\x1b[33m\x1b[5m", content),
        6 => format!("{}\x1b[34m\x1b[5m", content),
        7 => format!("{}\x1b[35m\x1b[5m", content),
        8 => format!("{}\x1b[33m", content),
        _ => content
    };
}