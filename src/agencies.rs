use crate::structure::LSP;

pub fn agency_logo(agency: LSP) -> &'static str {
    // println!("{}", agency.id.clone().unwrap());

    let url = match agency.id.clone().unwrap() {
        16 => "https://spacelaunchnow-prod-east.nyc3.digitaloceanspaces.com/media/logo/canadian2520space2520agency_logo_20190417034740.jpg",
        17 => "https://spacelaunchnow-prod-east.nyc3.digitaloceanspaces.com/media/logo/china2520national2520space2520administration_logo_20190207032431.png",
        27 => "https://spacelaunchnow-prod-east.nyc3.digitaloceanspaces.com/media/logo/european2520space2520agency_logo_20190207032435.png",
        31 => "https://spacelaunchnow-prod-east.nyc3.digitaloceanspaces.com/media/logo/indian2520space2520research2520organization_logo_20190215225409.png",
        34 => "https://spacelaunchnow-prod-east.nyc3.digitaloceanspaces.com/media/logo/iranian2520space2520agency_logo_20200422091550.jpg",
        37 => "https://spacelaunchnow-prod-east.nyc3.digitaloceanspaces.com/media/logo/japan2520aerospace2520exploration2520agency_logo_20190207032440.png",
        41 => "https://spacelaunchnow-prod-east.nyc3.digitaloceanspaces.com/media/logo/korea2520aerospace2520research2520institute_logo_20200806032236.png",
        44 => "https://spacelaunchnow-prod-east.nyc3.digitaloceanspaces.com/media/logo/national2520aeronautics2520and2520space2520administration_logo_20190207032448.png",
        63 => "https://spacelaunchnow-prod-east.nyc3.digitaloceanspaces.com/media/logo/russian2520federal2520space2520agency25202528roscosmos2529_logo_20190207032459.png",
        66 => "https://spacelaunchnow-prod-east.nyc3.digitaloceanspaces.com/media/logo/soviet2520space2520program_logo_20191229081307.png",
        88 => "https://spacelaunchnow-prod-east.nyc3.digitaloceanspaces.com/media/logo/china2520aerospace2520science2520and2520technology2520corporation_logo_20200114024619.png",
        96 => "http://tadviser.com/images/5/5c/200px-Khrun.png", //"https://spacelaunchnow-prod-east.nyc3.digitaloceanspaces.com/media/logo/khrunichev2520state2520research2520and2520production2520space2520center_logo_20190207032444.png",
        98 => "https://www.blugraphic.com/wp-content/uploads/2014/01/Mitsubishi-Vector-Logo-Ai-Eps-.jpg", //"https://spacelaunchnow-prod-east.nyc3.digitaloceanspaces.com/media/logo/mitsubishi2520heavy2520industries_logo_20200117065724.png",
        112 => "https://spacelaunchnow-prod-east.nyc3.digitaloceanspaces.com/media/logo/yuzhnoye2520design2520bureau_logo_20190207032505.png",
        115 => "https://pbs.twimg.com/profile_images/876626369509294081/RCtHSoFG_400x400.jpg", //"https://spacelaunchnow-prod-east.nyc3.digitaloceanspaces.com/media/logo/arianespace_logo_20190207032425.png",
        118 => "https://spacelaunchnow-prod-east.nyc3.digitaloceanspaces.com/media/logo/international2520launch2520services_logo_20200714154120.png",
        121 => "https://spacelaunchnow-prod-east.nyc3.digitaloceanspaces.com/media/logo/spacex_logo_20191121063502.png",
        124 => "https://spacelaunchnow-prod-east.nyc3.digitaloceanspaces.com/media/logo/united2520launch2520alliance_logo_20190223000119.png",
        141 => "https://spacelaunchnow-prod-east.nyc3.digitaloceanspaces.com/media/logo/blue2520origin_logo_20190207032427.png",
        147 => "https://spacelaunchnow-prod-east.nyc3.digitaloceanspaces.com/media/logo/rocket2520lab2520ltd_logo_20190207032456.png",
        178 => "https://spacelaunchnow-prod-east.nyc3.digitaloceanspaces.com/media/logo/airbus2520defence2520and2520space_logo_20190207032423.png",
        179 => "https://spacelaunchnow-prod-east.nyc3.digitaloceanspaces.com/media/logo/orbital2520atk_logo_20190207032453.png",
        184 => "https://spacelaunchnow-prod-east.nyc3.digitaloceanspaces.com/media/logo/china2520aerospace2520science2520and2520industry2520corporation_logo_20201111180047.png",
        191 => "https://spacelaunchnow-prod-east.nyc3.digitaloceanspaces.com/media/logo/united2520space2520alliance_logo_20190830220958.jpg",
        193 => "https://spacelaunchnow-prod-east.nyc3.digitaloceanspaces.com/media/logo/russian2520space2520forces_logo_20200805185824.png",
        199 => "https://spacelaunchnow-prod-east.nyc3.digitaloceanspaces.com/media/logo/virgin2520orbit_logo_20200101102856.png",
        257 => "https://spacelaunchnow-prod-east.nyc3.digitaloceanspaces.com/media/logo/northrop2520grumman2520innovation2520systems_logo_20190207032451.png",
        260 => "https://spacelaunchnow-prod-east.nyc3.digitaloceanspaces.com/media/logo/planet2520labs_logo_20190207032455.png",
        285 => "https://spacelaunchnow-prod-east.nyc3.digitaloceanspaces.com/media/logo/astra2520space_logo_20200216210038.png",
        999 => "https://spacelaunchnow-prod-east.nyc3.digitaloceanspaces.com/media/logo/north2520american2520aviation_logo_20200711032022.png",
        1001 => "https://spacelaunchnow-prod-east.nyc3.digitaloceanspaces.com/media/logo/exos2520aerospace_logo_20190629155319.png",
        1002 => "https://spacelaunchnow-prod-east.nyc3.digitaloceanspaces.com/media/logo/interstellar2520technologies_logo_20190705173722.png",
        1020 => "https://spacelaunchnow-prod-east.nyc3.digitaloceanspaces.com/media/logo/interstellar2520technologies_logo_20190705173722.png",
        1021 => "https://spacelaunchnow-prod-east.nyc3.digitaloceanspaces.com/media/logo/galactic2520energy_logo_20201106095229.png",
        _ => {
            println!("{}", agency.id.unwrap());
            "https://web.extension.illinois.edu/stain/stains-hi/235.jpg"
        }
    };
    return url;
}