use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct LaunchResponse {
    pub count: Option<isize>,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: Option<Vec<Launch>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Launch {
    pub id: Option<String>,
    pub url: Option<String>,
    pub launch_library_id: Option<isize>,
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
    pub launch_service_provider: Option<LSP>,
    pub rocket: Option<Rocket>,
    pub mission: Option<Mission>,
    pub pad: Option<LaunchPad>,
    pub webcast_live: Option<bool>,
    pub image: Option<String>,
    pub infographic: Option<String>,
    pub program: Option<Vec<Program>>
}

#[derive(Deserialize, Debug, Clone)]
pub struct LSP {
    pub id: Option<isize>,
    pub url: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub org: Option<String>
}

#[derive(Deserialize, Debug, Clone)]
pub struct Status {
    pub id: Option<isize>,
    pub name: Option<String>,
    pub abbrev: Option<String>,
    pub description: Option<String>
}

#[derive(Deserialize, Debug, Clone)]
pub struct Rocket {
    pub id: Option<isize>,
    pub configuration: Option<RocketConfiguration>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct RocketConfiguration {
    pub id: Option<isize>,
    pub launch_library_id: Option<isize>,
    pub url: Option<String>,
    pub name: Option<String>,
    pub family: Option<String>,
    pub full_name: Option<String>,
    pub variant: Option<String>
}

#[derive(Deserialize, Debug, Clone)]
pub struct Mission {
    pub id: Option<isize>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub orbit: Option<Orbit>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Orbit {
    pub id: Option<isize>,
    pub name: Option<String>,
    pub abbrev: Option<String>
}

#[derive(Deserialize, Debug, Clone)]
pub struct LaunchPad {
    pub id: Option<isize>,
    pub url: Option<String>,
    pub agency_id: Option<isize>,
    pub name: Option<String>,
    pub info_url: Option<String>,
    pub wiki_url: Option<String>,
    pub map_url: Option<String>,
    pub latitude: Option<String>,
    pub longitude: Option<String>,
    pub location: PadLocation,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PadLocation {
    pub id: Option<isize>,
    pub url: Option<String>,
    pub name: Option<String>,
    pub country_code: Option<String>,
    pub map_image: Option<String>,
    pub total_launch_count: Option<isize>,
    pub total_landing_count: Option<isize>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Program {
    pub id: Option<isize>,
    pub url: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub agencies: Option<Vec<LSP>>
}

#[derive(Deserialize, Debug, Clone)]
pub struct Article {
    pub id: Option<String>,
    pub title: Option<String>,
    pub url: Option<String>,
    pub imageUrl: Option<String>,
    pub summary: Option<String>,
    pub newsSite: Option<String>,
}