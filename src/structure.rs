use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct LaunchResponse {
    pub count: Option<isize>,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: Vec<Launch>,
}

#[derive(Deserialize, Clone)]
pub struct Launch {
    pub id: Option<String>,
    pub url: Option<String>,
    pub slug: Option<String>,
    pub name: Option<String>,
    pub status: Status,
    pub net: Option<String>,
    pub window_end: Option<String>,
    pub window_start: Option<String>,
    pub inhold: Option<bool>,
    pub tbdtime: Option<bool>,
    pub tbddate: Option<bool>,
    pub probability: Option<isize>,
    pub holdreason: Option<String>,
    pub failreason: Option<String>,
    pub hashtag: Option<String>,
    pub rocket: Option<Rocket>,
    pub mission: Option<Mission>,
    pub pad: Option<LaunchPad>,
    pub image_url: Option<String>,
    pub infographic_url: Option<String>,
}

#[derive(Deserialize, Clone)]
pub struct Status {
    pub id: Option<isize>,
    pub name: Option<String>,
}

#[derive(Deserialize, Clone)]
pub struct Rocket {
    pub id: Option<isize>,
    pub configuration: Option<RocketConfiguration>,
}

#[derive(Deserialize, Clone)]
pub struct RocketConfiguration {
    pub id: Option<isize>,
    pub url: Option<String>,
    pub name: Option<String>,
    pub launch_service_provider: Option<String>,
}

#[derive(Deserialize, Clone)]
pub struct Mission {
    pub id: Option<isize>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub orbit: Option<String>,
    pub orbit_abbrev: Option<String>,
}

#[derive(Deserialize, Clone)]
pub struct LaunchPad {
    pub id: Option<isize>,
    pub agency_id: Option<isize>,
    pub name: Option<String>,
    pub info_url: Option<String>,
    pub wiki_url: Option<String>,
    pub map_url: Option<String>,
    pub location: PadLocation,
    pub latitude: Option<String>,
    pub longitude: Option<String>,
}

#[derive(Deserialize, Clone)]
pub struct PadLocation {
    pub id: Option<isize>,
    pub name: Option<String>,
    pub country_code: Option<String>,
}