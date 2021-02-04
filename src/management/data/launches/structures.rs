use serde::{Serialize, Deserialize};

#[derive(Debug, Clone)]
pub enum LaunchAPIop {
    FETCH,
    UPDATE,
    CACHE,
    READ,
    RESPONSE,
    ERROR,
}

#[derive(Debug, Clone)]
pub struct LaunchAPI {
    pub op: LaunchAPIop,
    pub launch: Option<Launch>,
}



#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LaunchResponse {
    pub count: Option<isize>,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: Option<Vec<Launch>>,
    pub detail: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Launch {
    pub id: Option<String>,
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
    pub program: Option<Vec<Program>>,
    pub orbital_launch_attempt_count: Option<isize>,
    pub location_launch_attempt_count: Option<isize>,
    pub pad_launch_attempt_count: Option<isize>,
    pub agency_launch_attempt_count: Option<isize>,
    pub orbital_launch_attempt_count_year: Option<isize>,
    pub location_launch_attempt_count_year: Option<isize>,
    pub pad_launch_attempt_count_year: Option<isize>,
    pub agency_launch_attempt_count_year: Option<isize>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LSP {
    pub id: Option<isize>,
    pub name: Option<String>,
    pub features: Option<bool>,
    #[serde(rename = "type")]
    pub org: Option<String>,
    pub country_code: Option<String>,
    pub abbrev: Option<String>,
    pub description: Option<String>,
    pub administrator: Option<String>,
    pub founding_year: Option<String>,
    pub launchers: Option<String>,
    pub spacecraft: Option<String>,
    pub launch_library_url: Option<String>,
    pub total_launch_count: Option<isize>,
    pub consecutive_successful_launches: Option<isize>,
    pub successful_launches: Option<isize>,
    pub failed_launches: Option<isize>,
    pub pending_launches: Option<isize>,
    pub consecutive_successful_landings: Option<isize>,
    pub successful_landings: Option<isize>,
    pub failed_landings: Option<isize>,
    pub attempted_landings: Option<isize>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Status {
    pub id: Option<isize>,
    pub name: Option<String>,
    pub abbrev: Option<String>,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Rocket {
    pub id: Option<isize>,
    pub configuration: Option<RocketConfiguration>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RocketConfiguration {
    pub id: Option<isize>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub family: Option<String>,
    pub full_name: Option<String>,
    pub manufacturer: Option<LSP>,
    pub variant: Option<String>,
    pub alias: Option<String>,
    pub min_stage: Option<isize>,
    pub max_stage: Option<isize>,
    pub length: Option<f32>,
    pub diameter: Option<f32>,
    pub maiden_flight: Option<String>,
    pub launch_mass: Option<isize>,
    pub leo_capacity: Option<isize>,
    pub gto_capacity: Option<isize>,
    pub to_thrust: Option<isize>,
    pub apogee: Option<isize>,
    pub vehicle_range: Option<isize>,
    pub total_launch_count: Option<isize>,
    pub consecutive_successful_launches: Option<isize>,
    pub successful_launches: Option<isize>,
    pub failed_launches: Option<isize>,
    pub pending_launches: Option<isize>,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Mission {
    pub id: Option<isize>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub launch_designator: Option<String>,
    #[serde(rename = "type")]
    pub mission_type: Option<String>,
    pub orbit: Option<Orbit>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Orbit {
    pub id: Option<isize>,
    pub name: Option<String>,
    pub abbrev: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LaunchPad {
    pub id: Option<isize>,
    pub agency_id: Option<isize>,
    pub name: Option<String>,
    pub latitude: Option<String>,
    pub longitude: Option<String>,
    pub location: PadLocation,
    pub total_launch_count: Option<isize>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PadLocation {
    pub id: Option<isize>,
    pub name: Option<String>,
    pub country_code: Option<String>,
    pub total_launch_count: Option<isize>,
    pub total_landing_count: Option<isize>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Program {
    pub id: Option<isize>,
    pub url: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub agencies: Option<Vec<LSP>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Article {
    pub id: Option<String>,
    pub title: Option<String>,
    pub url: Option<String>,
    pub imageUrl: Option<String>,
    pub summary: Option<String>,
    pub newsSite: Option<String>,
}