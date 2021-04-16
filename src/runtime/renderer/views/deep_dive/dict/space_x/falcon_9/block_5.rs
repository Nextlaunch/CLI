pub use super::super::super::{Rocket, Stage, Engine};
use crate::runtime::renderer::views::deep_dive::dict::Fuel;

pub fn wiki() -> Rocket {
    Rocket {
        name: "Falcon 9 Block 5".to_string(),
        stages: vec![
            Stage {
                name: "Booster".to_string(),
                engines: vec![
                    Engine {
                        count: 9,
                        name: "Merlin 1D+".to_string(),
                        class: "Atmospheric".to_string(),
                        manufacturer: "SpaceX".to_string(),
                        engine_type: "Bi-propellant".to_string(),
                        thrust_sea: "845 kN".to_string(),
                        thrust_vac: "981 kN".to_string(),
                        thrust_weight_sea: "86,166 kgf".to_string(),
                        thrust_weight_vac: "100,035 kgf".to_string(),
                        cycle: "Gas Generator".to_string(),
                        fuels: vec![
                            Fuel {
                                name: "Liquid Oxygen".to_string(),
                                abbrev: "LOX".to_string(),
                                class: "".to_string(),
                                density_g_ml: "1.141 g/ml".to_string(),
                                mass_kg_l: "1.141 kg/l".to_string(),
                            },
                            Fuel {
                                name: "Refined Petroleum 1".to_string(),
                                abbrev: "RP-1".to_string(),
                                class: "".to_string(),
                                density_g_ml: "0.81".to_string(),
                                mass_kg_l: "0.81 kg/l".to_string(),
                            }
                        ],
                        specific_impulse: "310 Seconds".to_string(),
                        gimbal: "??? Degrees".to_string(),
                        min_throttle: "57%".to_string(),
                        min_throttle_as_newtons: "482 kN".to_string(),
                        min_throttle_sea: "64%".to_string(),
                        min_throttle_sea_as_newtons: "626 kN".to_string(),
                    }
                ],
                thrust_weight_sea: "775, 494 kgf".to_string(),
                thrust_weight_vac: "900, 315 kgf".to_string(),
            }
        ],
        reusable: "Fully".to_string(),
        entered_service: "".to_string(),
        retired: false,
        retired_date: "".to_string(),
        human_rated: true,
        capacity_leo: "".to_string(),
        capacity_gto: "".to_string(),
        capacity_mto: "".to_string(),
        height: "".to_string(),
        diameter: "".to_string(),
        country: "USA".to_string(),
        manufacturer: "SpaceX".to_string(),
        family: "Falcon".to_string(),
        first_flight: "".to_string(),
    }
}