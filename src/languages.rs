use serde::{Deserialize, Serialize};

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
    pub unknown_mission: &'static str,
    pub unknown_launch: &'static str,
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
    pub name: &'static str,
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

pub fn select_language(_id: &str) -> LanguagePack {
    // let f = dirs_2;

    LanguagePack {
        name: "en-GB",
        time: TimePack {
            year: "Year",
            year_plural: "Years",
            week: "Week",
            week_plural: "Weeks",
            day: "Day",
            day_plural: "Days",
            hour: "Hour",
            hour_plural: "Hours",
            minute: "Minute",
            minute_plural: "Minutes",
            second: "Second",
            second_plural: "Seconds",
        },
        launch: LaunchPack {
            name: "Name",
            provider: "Provider",
            vehicle: "Vehicle",
            mission: "Mission",
            pad: "Pad",
            location: "Location",
            status: StatusPack {
                name: "Status",
                success: "Success",
                to_be_determined: "To Be Determined",
                to_be_confirmed: "To Be Confirmed",
                partial_failure: "Partial Failure",
                failure: "Failure",
                go_for_liftoff: "Go For Liftoff",
                in_flight: "In Flight",
                on_hold: "On Hold",
                fetching: "Fetching",
            },
            unknown_launch: "Unkown Launch",
            unknown_mission: "Unknown Mission",
        },
        titles: TitlePack {
            launch: "Launch",
            updates: "Updates",
            news: "News",
            logs: "Logo",
            countdown: "Countdown",
        },
    }
}
