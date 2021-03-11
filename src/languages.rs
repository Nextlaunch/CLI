use locale_config::Locale;
use std::process::exit;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LanguagePack {
    pub name: &'static str,
    pub time: TimePack,
    pub launch: LaunchPack,
    pub titles: TitlePack,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TimePack {
    pub year: &'static str,
    pub year_plural: &'static str,

    pub week: &'static str,
    pub week_plural: &'static str,

    pub day: &'static str,
    pub day_plural: &'static str,

    pub hour: &'static str,
    pub hour_plural: &'static str,

    pub minute: &'static str,
    pub minute_plural: &'static str,

    pub second: &'static str,
    pub second_plural: &'static str,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LaunchPack {
    pub name: &'static str,
    pub provider: &'static str,
    pub vehicle: &'static str,
    pub mission: &'static str,
    pub pad: &'static str,
    pub location: &'static str,
    pub status: StatusPack,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TitlePack {
    pub launch: &'static str,
    pub updates: &'static str,
    pub news: &'static str,
    pub logs: &'static str,
    pub countdown: &'static str,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StatusPack {
    pub success: &'static str,
    pub to_be_determined: &'static str,
    pub to_be_confirmed: &'static str,
    pub partial_failure: &'static str,
    pub failure: &'static str,
    pub go_for_liftoff: &'static str,
    pub in_flight: &'static str,
    pub on_hold: &'static str,
    pub fetching: &'static str,
}


pub fn select_language(id: &str) -> LanguagePack {
    match id {
        _ => load_language(1)
    }
}

pub fn load_language(id: usize) -> LanguagePack {
    // let f = dirs_2;
    
    LanguagePack {
        name: "",
        time: TimePack {
            year: "",
            year_plural: "",
            week: "",
            week_plural: "",
            day: "",
            day_plural: "",
            hour: "",
            hour_plural: "",
            minute: "",
            minute_plural: "",
            second: "",
            second_plural: ""
        },
        launch: LaunchPack {
            name: "",
            provider: "",
            vehicle: "",
            mission: "",
            pad: "",
            location: "",
            status: StatusPack {
                success: "",
                to_be_determined: "",
                to_be_confirmed: "",
                partial_failure: "",
                failure: "",
                go_for_liftoff: "",
                in_flight: "",
                on_hold: "",
                fetching: ""
            }
        },
        titles: TitlePack {
            launch: "",
            updates: "",
            news: "",
            logs: "",
            countdown: ""
        }
    }
}