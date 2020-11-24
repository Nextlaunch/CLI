use proctitle::set_title;
use reqwest::blocking::Client;
use std::time::Duration;

mod structure;

fn main() {
    let mut counter = 0;
    loop {
        counter += 1;
        let client = Client::new();
        let response = client.get("https://spacelaunchnow.me/api/3.3.0/launch/upcoming/?format=json").send();
        if response.is_ok() {
            let body = response.unwrap();
            let json: structure::LaunchResponse = body.json().unwrap();
            let next = json.results.first().unwrap().clone();
            let mission = next.mission.unwrap();
            println!("{} - {}", counter, mission.name.clone().unwrap());
            set_title(format!("{} - {}", counter, mission.name.unwrap()));
        } else {
            set_title(format!("Failed to get next launch (HTTP {})", response.unwrap_err().status().unwrap()))
        }
        std::thread::sleep(Duration::from_secs(10))
    }
}
