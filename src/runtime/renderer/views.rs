pub mod default;
pub mod deep_dive;

use crate::runtime::data::launches::structures::Launch;
use std::char;

pub trait ToBraille {
    /// ```
    fn to_braille(&self) -> char;
}

impl ToBraille for u8 {
    fn to_braille(&self) -> char {
        let warped_dotmap: u8 =
            (self >> 7 & 0b00000001u8) | 
                (self >> 3 & 0b00001000u8) | 
                (self >> 4 & 0b00000010u8) | 
                (self >> 0 & 0b00010000u8) | 
                (self >> 1 & 0b00000100u8) | 
                (self << 3 & 0b00100000u8) | 
                (self << 5 & 0b01000000u8) | 
                (self << 7 & 0b10000000u8);  
        char::from_u32(10240 + warped_dotmap as u32).unwrap()
    }
}

use image::imageops::resize;

pub struct Display {
    image: image::GrayImage,
    char_width: u32,
    char_height: u32,
}

impl Display {
    pub fn new(img: image::GrayImage, width: u32, height: u32) -> Display {
        Display {
            image: resize(&img, width * 2, height * 4, image::Lanczos3),
            char_width: width,
            char_height: height,
        }
    }

    pub fn render(&self) -> Vec<String> {
        let mut target = Vec::<String>::new();

        for y in 0..self.char_height {
            let mut line = String::new();
            for x in 0..self.char_width {
                line = format!("{}{}", line, self.braillify_block(x, y).to_string());
            }
            target.push(line);
        };
        target
    }

    fn braillify_block(&self, x: u32, y: u32) -> char {
        let mut dot_map = 0b0000_0000;
        for i in 0..8 {
            let abs_x = (x * 2) + (i % 2);
            let abs_y = (y * 4) + (i / 2);

            dot_map |= if self.sample(abs_x, abs_y) {
                0b1000_0000 >> i
            } else {
                0
            };
        }
        dot_map.to_braille()
    }

    fn sample(&self, x: u32, y: u32) -> bool {
        self.image[(x, y)][0] < 128
    }
}

pub fn process_image(path: &str, _launch: Launch) -> Vec<String> {
    let img = match image::open(path) {
        Ok(image) => image.to_luma(),
        Err(err) => {
            println!("Couldn't open image!");
            println!("{}", err);
            println!("Failed to open: {}", path);
            std::process::exit(1);
        }
    };

    let (naive_width, naive_height) = img.dimensions();
    let desired_width: Option<u32> = Some(40);
    let desired_height: Option<u32> = Some(20);

    let (width, height) = match (desired_width, desired_height) {
        (None, None) => (naive_width / 10, naive_height / 20),
        (Some(w), None) => (w, ((naive_height * w) / naive_width) / 2),
        (None, Some(h)) => (((naive_width * h) / naive_height) * 2, h),
        (Some(w), Some(h)) => (w, h)
    };

    let display = Display::new(img, width, height);

    let lines = display.render();
    return lines;
}

pub fn parse_path(previous: Option<Launch>) -> String {
    let tmp_dir_opt = std::env::temp_dir();
    let mut tmp_dir = tmp_dir_opt.to_str().unwrap().to_string();

    let mut source = "https://web.extension.illinois.edu/stain/stains-hi/235.jpg".to_string();

    let encoded = String::from("logo-nextlaunch-dnf");

    let mut x = previous.clone().unwrap();

    if previous.is_some() {
        let pl = previous.clone().unwrap();
        source = agency_logo(pl.launch_service_provider.unwrap().id.unwrap()).to_string();
        x.raw_image = Some(source.clone());
    }

    let components: Vec<&str> = source.split(".").collect();
    let extension: String = components.last().unwrap().clone().to_string();

    let tree = tmp_dir.split("\\").collect::<Vec<&str>>();

    tmp_dir = tree.join("/");

    if tmp_dir.chars().last().unwrap() != '/' {
        tmp_dir = format!("{}/", tmp_dir);
    }
    return format!("{}{}.{}", tmp_dir, encoded, extension);
}


pub fn agency_logo(id: isize) -> &'static str {
    let url = match id {
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
        121 => "https://dehayf5mhw1h7.cloudfront.net/wp-content/uploads/sites/1028/2020/05/30143403/Space-X.jpg",
        124 => "https://spacelaunchnow-prod-east.nyc3.digitaloceanspaces.com/media/logo/united2520launch2520alliance_logo_20190223000119.png",
        141 => "https://i0.wp.com/logotaglines.com/wp-content/uploads/2021/06/Blue-Origin-Logo-slogan-tagline-founder-owner-motto.jpg",
        147 => "https://spacelaunchnow-prod-east.nyc3.digitaloceanspaces.com/media/logo/rocket2520lab2520ltd_logo_20190207032456.png",
        178 => "https://spacelaunchnow-prod-east.nyc3.digitaloceanspaces.com/media/logo/airbus2520defence2520and2520space_logo_20190207032423.png",
        179 => "https://spacelaunchnow-prod-east.nyc3.digitaloceanspaces.com/media/logo/orbital2520atk_logo_20190207032453.png",
        184 => "https://spacelaunchnow-prod-east.nyc3.digitaloceanspaces.com/media/logo/china2520aerospace2520science2520and2520industry2520corporation_logo_20201111180047.png",
        191 => "https://spacelaunchnow-prod-east.nyc3.digitaloceanspaces.com/media/logo/united2520space2520alliance_logo_20190830220958.jpg",
        193 => "https://spacelaunchnow-prod-east.nyc3.digitaloceanspaces.com/media/logo/russian2520space2520forces_logo_20200805185824.png",
        199 => "https://spacelaunchnow-prod-east.nyc3.digitaloceanspaces.com/media/logo/virgin2520orbit_logo_20200101102856.png",
        257 => "https://spacelaunchnow-prod-east.nyc3.digitaloceanspaces.com/media/logo/northrop2520grumman2520innovation2520systems_logo_20190207032451.png",
        260 => "https://spacelaunchnow-prod-east.nyc3.digitaloceanspaces.com/media/logo/planet2520labs_logo_20190207032455.png",
        //265 => "https://pbs.twimg.com/profile_images/1080623741929848832/y0iRPnfj_400x400.jpg",
        282 => "https://pbs.twimg.com/profile_images/758774066094432256/eh1oe7_x_400x400.jpg",
        285 => "https://spacelaunchnow-prod-east.nyc3.digitaloceanspaces.com/media/logo/astra2520space_logo_20200216210038.png",
        999 => "https://spacelaunchnow-prod-east.nyc3.digitaloceanspaces.com/media/logo/north2520american2520aviation_logo_20200711032022.png",
        1001 => "https://spacelaunchnow-prod-east.nyc3.digitaloceanspaces.com/media/logo/exos2520aerospace_logo_20190629155319.png",
        1002 => "https://spacelaunchnow-prod-east.nyc3.digitaloceanspaces.com/media/logo/interstellar2520technologies_logo_20190705173722.png",
        1020 => "https://spacelaunchnow-prod-east.nyc3.digitaloceanspaces.com/media/logo/interstellar2520technologies_logo_20190705173722.png",
        1021 => "https://spacelaunchnow-prod-east.nyc3.digitaloceanspaces.com/media/logo/galactic2520energy_logo_20201106095229.png",
        _ => {
            "https://web.extension.illinois.edu/stain/stains-hi/235.jpg"
        }
    };
    return url;
}