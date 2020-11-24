use serde::ser::Serialize;
use serde::de::Deserialize;

#[derive(Serialize, Deserialize)]
pub struct Launch {
    pub id: String,
    pub url: String,
    pub slug: String,
    pub name: String,
    pub status: Status,
    pub net: String,
    pub window_end: String,
    pub window_start: String,
    pub inhold: bool,
    pub tbdtime: bool,
    pub tbddate: bool,
    pub probability: isize,
    pub holdreason: String,
    pub failreason: String,
    pub hashtag: String,
    pub rocket: Rocket,
    pub mission: Mission,
    pub pad: LaunchPad,
    pub image_url: String,
    pub infographic_url: String,
}

#[derive(Serialize, Deserialize)]
pub struct Status {
    pub id: isize,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Rocket {
    pub id: isize,
    pub configuration: RocketConfiguration,
}

#[derive(Serialize, Deserialize)]
pub struct RocketConfiguration {
    pub id: isize,
    pub url: String,
    pub name: String,
    pub launch_service_provider: String,
}

#[derive(Serialize, Deserialize)]
pub struct Mission {
    pub id: isize,
    pub name: String,
    pub description: String,
    pub orbit: String,
    pub orbit_abbrev: String,
}

#[derive(Serialize, Deserialize)]
pub struct LaunchPad {
    pub id: isize,
    pub agency_id: isize,
    pub name: String,
    pub info_url: String,
    pub wiki_url: String,
    pub map_url: String,
    pub location: PadLocation,
    pub latitude: String,
    pub longitude: String,
}

#[derive(Serialize, Deserialize)]
pub struct PadLocation {
    pub id: isize,
    pub name: String,
    pub country_code: String,
}