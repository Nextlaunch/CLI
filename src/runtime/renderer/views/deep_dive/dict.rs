pub mod space_x;

pub struct Rocket {
    pub name: String,
    pub stages: Vec<Stage>,
    pub reusable: String,
    pub entered_service: String,
    pub retired: bool,
    pub retired_date: String,
    pub human_rated: bool,
    pub capacity_leo: String,
    pub capacity_gto: String,
    pub capacity_mto: String,
    pub height: String,
    pub diameter: String,
    pub country: String,
    pub manufacturer: String,
    pub family: String,
    pub first_flight: String,
}

pub struct Stage {
    pub name: String,
    pub engines: Vec<Engine>,
    pub thrust_weight: String,
}

pub struct Engine {
    pub count: usize,
    pub name: String,
    pub class: String,
    pub manufacturer: String,
    pub engine_type: String,
    pub thrust_sea: String,
    pub thrust_vac: String,
    pub thrust_weight_sea: String,
    pub thrust_weight_vac: String,
    pub cycle: String,
    pub fuels: Vec<Fuel>,
    pub specific_impulse: String,
    pub gimbal: String,
    pub min_throttle: String,
    pub min_throttle_as_newtons: String,
    pub min_throttle_sea: String,
    pub min_throttle_sea_as_newtons: String,
}

pub struct Fuel {
    pub name: String,
    pub abbrev: String,
    pub class: String,
    pub density_g_ml: String,
    pub mass_kg_l: String,
}

pub fn match_rocket(rocket: isize) -> Option<Rocket> {
    match rocket {
        2746 => Some(space_x::falcon_9::block_5::wiki()),
        _ => None
    }
}