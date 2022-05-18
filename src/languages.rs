use serde::{Deserialize, Serialize};
use std::fs::{metadata, read};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LanguagePack {
    pub name: String,
    pub time: TimePack,
    pub launch: LaunchPack,
    pub titles: TitlePack,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TimePack {
    pub year: String,
    pub year_plural: String,

    pub week: String,
    pub week_plural: String,

    pub day: String,
    pub day_plural: String,

    pub hour: String,
    pub hour_plural: String,

    pub minute: String,
    pub minute_plural: String,

    pub second: String,
    pub second_plural: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LaunchPack {
    pub name: String,
    pub provider: String,
    pub vehicle: String,
    pub mission: String,
    pub pad: String,
    pub location: String,
    pub status: StatusPack,
    pub unknown_mission: String,
    pub unknown_vehicle: String,
    pub unknown_launch: String,
    pub unknown_provider: String,
    pub unknown_launchpad: String,
    pub unknown_location: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TitlePack {
    pub launch: String,
    pub updates: String,
    pub news: String,
    pub logs: String,
    pub countdown: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StatusPack {
    pub name: String,
    pub success: String,
    pub to_be_determined: String,
    pub to_be_confirmed: String,
    pub partial_failure: String,
    pub failure: String,
    pub go_for_liftoff: String,
    pub in_flight: String,
    pub on_hold: String,
    pub fetching: String,
}

pub fn select_language(id: &str) -> LanguagePack {
    let mut language = default();

    let raw_data_dir = dirs_2::data_dir();
    if let Some(data_path) = raw_data_dir {
        let path = format!("{}/nextlaunch/languages/{}.json", data_path.as_os_str().to_str().unwrap(), id);
        let m = metadata(&path);
        if let Ok(_) = m {
            let x = read(&path);
            if let Ok(data) = x {
                let pack: serde_json::Result<LanguagePack> = serde_json::from_slice(data.as_slice());
                if let Ok(p) = pack {
                    language = p;
                } else {
                    println!("Failed to parse language pack: {}", id);
                    println!("{}", path);
                    let _ = dbg!(pack);
                    std::process::exit(1);
                }
            } else {
                println!("Failed to read language pack: {}", id);
                println!("{}", path);
                let _ = dbg!(m);
                std::process::exit(1);
            }
        } else {
            println!("Failed to read metadata for language pack: {}", id);
            println!("{}", path);
            let _ = dbg!(m);
            std::process::exit(1);
        }
    }

    return language;
}


fn default() -> LanguagePack {
    LanguagePack {
        name: "en-GB".to_string(),
        time: TimePack {
            year: "Year".to_string(),
            year_plural: "Years".to_string(),
            week: "Week".to_string(),
            week_plural: "Weeks".to_string(),
            day: "Day".to_string(),
            day_plural: "Days".to_string(),
            hour: "Hour".to_string(),
            hour_plural: "Hours".to_string(),
            minute: "Minute".to_string(),
            minute_plural: "Minutes".to_string(),
            second: "Second".to_string(),
            second_plural: "Seconds".to_string(),
        },
        launch: LaunchPack {
            name: "Name".to_string(),
            provider: "Provider".to_string(),
            vehicle: "Vehicle".to_string(),
            mission: "Mission".to_string(),
            pad: "Pad".to_string(),
            location: "Location".to_string(),
            status: StatusPack {
                name: "Status".to_string(),
                success: "Success".to_string(),
                to_be_determined: "To Be Determined".to_string(),
                to_be_confirmed: "To Be Confirmed".to_string(),
                partial_failure: "Partial Failure".to_string(),
                failure: "Failure".to_string(),
                go_for_liftoff: "Go For Liftoff".to_string(),
                in_flight: "In Flight".to_string(),
                on_hold: "On Hold".to_string(),
                fetching: "Fetching".to_string(),
            },
            unknown_launch: "Unkown Launch".to_string(),
            unknown_mission: "Unknown Mission".to_string(),
            unknown_provider: "Unknown Provider".to_string(),
            unknown_launchpad: "Unknown Launchpad".to_string(),
            unknown_location: "Unknown Location".to_string(),
            unknown_vehicle: "Unknown Vehicle".to_string()
        },
        titles: TitlePack {
            launch: "Launch".to_string(),
            updates: "Updates".to_string(),
            news: "News".to_string(),
            logs: "Logo".to_string(),
            countdown: "Countdown".to_string(),
        },
    }
}

