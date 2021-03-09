use locale_config::Locale;
use std::process::exit;

pub struct LanguagePack {
    pub name: &'static str,
    pub time: TimePack,
    pub launch: LaunchPack,
    pub titles: TitlePack,
}

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

pub struct LaunchPack {
    pub name: &'static str,
    pub provider: &'static str,
    pub vehicle: &'static str,
    pub mission: &'static str,
    pub pad: &'static str,
    pub location: &'static str,
    pub status: StatusPack,
}

pub struct TitlePack {
    pub launch: &'static str,
    pub updates: &'static str,
    pub news: &'static str,
    pub logs: &'static str,
    pub countdown: &'static str,
}

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

pub mod en_gb;
pub mod de_de;


pub fn select_language() {
    let locale = Locale::user_default();
    let tags = locale.tags();
    for tag in tags {
        println!("tag: {}  range: {}", tag.0.unwrap_or(""), tag.1)
    }
    exit(1);
}