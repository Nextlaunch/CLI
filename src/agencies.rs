use crate::structure::LSP;

pub fn agency_logo(agency: LSP) -> &'static str {
    return match agency.id.clone().unwrap() {
        17 => "https://spacelaunchnow-prod-east.nyc3.digitaloceanspaces.com/media/logo/china2520national2520space2520administration_logo_20190207032431.png",
        31 => "https://spacelaunchnow-prod-east.nyc3.digitaloceanspaces.com/media/logo/indian2520space2520research2520organization_logo_20190215225409.png",
        37 => "https://spacelaunchnow-prod-east.nyc3.digitaloceanspaces.com/media/logo/japan2520aerospace2520exploration2520agency_logo_20190207032440.png",
        44 => "https://spacelaunchnow-prod-east.nyc3.digitaloceanspaces.com/media/logo/national2520aeronautics2520and2520space2520administration_logo_20190207032448.png",
        63 => "https://spacelaunchnow-prod-east.nyc3.digitaloceanspaces.com/media/logo/russian2520federal2520space2520agency25202528roscosmos2529_logo_20190207032459.png",
        66 => "https://spacelaunchnow-prod-east.nyc3.digitaloceanspaces.com/media/logo/soviet2520space2520program_logo_20191229081307.png",
        88 => "https://spacelaunchnow-prod-east.nyc3.digitaloceanspaces.com/media/logo/china2520aerospace2520science2520and2520technology2520corporation_logo_20200114024619.png",
        96 => "https://spacelaunchnow-prod-east.nyc3.digitaloceanspaces.com/media/logo/khrunichev2520state2520research2520and2520production2520space2520center_logo_20190207032444.png",
        98 => "https://www.blugraphic.com/wp-content/uploads/2014/01/Mitsubishi-Vector-Logo-Ai-Eps-.jpg", //"https://spacelaunchnow-prod-east.nyc3.digitaloceanspaces.com/media/logo/mitsubishi2520heavy2520industries_logo_20200117065724.png",
        112 => "https://spacelaunchnow-prod-east.nyc3.digitaloceanspaces.com/media/logo/yuzhnoye2520design2520bureau_logo_20190207032505.png",
        121 => "https://spacelaunchnow-prod-east.nyc3.digitaloceanspaces.com/media/logo/spacex_logo_20191121063502.png",
        // 124 => "",
        // 139 => "",
        // 141 => "",
        // 147 => "",
        // 178 => "",
        // 179 => "",
        // 184 => "",
        // 191 => "",
        193 => "https://spacelaunchnow-prod-east.nyc3.digitaloceanspaces.com/media/logo/russian2520space2520forces_logo_20200805185824.png",
        // 199 => "",
        // 257 => "",
        // 260 => "",
        // 285 => "",
        // 1001 => "",
        // 1002 => "",
        // 1020 => "",
        // 1021 => "",
        _ => {
            println!("{}", agency.id.unwrap());
            "https://www.iconspng.com/images/mono-unknown/mono-unknown.png"
        }
    };
}

// Grab URL https://lldev.thespacedevs.com/2.1.0/agencies/<AGENCY ID>/?format=json