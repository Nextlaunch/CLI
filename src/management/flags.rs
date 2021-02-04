use std::env::args;
use std::process::exit;
use crate::{FC_BASE, SLN_BASE, G4L_BASE, NL_BASE};

pub async fn check_flags() -> Flags {
    let args: Vec<String> = args().collect();

    let mut flags = Flags {
        view: 0,
        help: false,
        preview_1: false,
        preview_2: false,
        version: false,
        offline: false,
        credits: false,
    };

    for arg in args {
        match arg.to_lowercase().as_str() {
            "--version" | "-v" => {
                flags.version = true;
            }
            "--help" | "-h" => {
                flags.help = true
            }
            "--offline" | "-o" => {
                flags.offline = true;
            }
            "--credits" | "-c" => {
                flags.credits = true;
            }
            "--p1" => {
                flags.preview_1 = true;
            }
            "--p2" => {
                flags.preview_2 = true;
            }
            _ => {
                if arg.starts_with("-") {
                    println!(r#"Unknown flag: "{}""#, arg)
                }
            }
        }
    }
    flags
}

#[derive(Debug, Clone)]
pub struct Flags {
    pub view: usize,
    pub help: bool,
    pub preview_1: bool,
    pub preview_2: bool,
    pub version: bool,
    pub offline: bool,
    pub credits: bool,
}


pub async fn process_flags(f: &Flags) {
    if f.help {
        println!(
            r#"
{}
{}

Version: {}
Author: {}
Usage: nextlaunch [Flags]
Flags:
    -h, --help
        Prints help information

    -v, --version
        Prints version information

    -c, --credits
        Prints the credits of all who helped influence the program

    -o, --offline
        Forces the program to try and operate from locally cached information
"#, crate::NAME, crate::DESCRIPTION, crate::VERSION, crate::AUTHOR);
        exit(1);
    } else if f.version {
        println!(
            r#"{}

Version {}
"#, crate::NAME, crate::VERSION);
        exit(1);
    } else if f.credits {
        println!(
            r#"{} - Credits

Data Providers:
News:      Space Flight News API <https://thespacedevs.com/snapi>
Launches:  Launch Library 2      <https://thespacedevs.com/llapi>
Telemetry: Launch Dashboard      <https://github.com/shahar603/Launch-Dashboard-API>

Developer: AltriusRS             <https://github.com/AltriusRS>

Language:  Rust                  <https://rust-lang.org>

Interface: (Alphabetical)
           Accusitive            <https://github.com/accusitive>
           Jas777                <https://github.com/jas777>
           Zane                  <https://github.com/AnotherZane>

Libraries: (Alphabetical)
           Bincode               <https://github.com/servo/bincode>
           Chrono                <https://github.com/chronotope/chrono>
           Reqwest               <https://github.com/seanmonstar/reqwest>
           Serde                 <https://github.com/serde-rs/serde>
           Terminal Size         <https://github.com/eminence/terminal-size>
           Tokio                 <https://github.com/tokio-rs/tokio>
           Websockets            <https://github.com/websockets-rs/rust-websocket>
"#, crate::NAME);
        exit(1);
    } else if f.preview_1 {
        println!("\
+======================================================================= NextLaunch ======================================================================+
| Mission:              Starlink 17 | Status:                     \x1b[32mGo For Launch\x1b[0m | State:                      \x1b[32mClear\x1b[0m | Humidity:                       52% |
| Vehicle:         Falcon 9 Block 5 | Window Open:  Feb 2nd 2021 - 10:14:00 GMT | Wind Direction:  North North East | Air Pressure:           1024.5 MBar |
| Location:      Launch Complex 39A | Window Close: Feb 2nd 2021 - 10:14:00 GMT | Wind Speed:                 \x1b[32m3 MPH\x1b[0m | Temperature:                   12 C |
| Provider:                  SpaceX | T-0:          Feb 2nd 2021 - 10:14:00 GMT | Wind Bearing:          27 Degrees | Minimum Temperature:            7 C |
| Destination:      Low Earth Orbit | Likelihood:                           \x1b[32m70%\x1b[0m | Visibility:              \x1b[32m14 Miles\x1b[0m | Maximum Temperatrue:           14 C |
+=========== Mission Info ==========+======+======== Status Info ===============+========================== Weather Information ==========================+
| Total Launch Count:                  122 |                                                                                                              |
| Successful Launches:                 113 |                                                                                                              |
| Failed Launches:                       8 |                                                                                                              |
| Pending Launches:                     35 |                                                                                                              |
| Consecutive Successes:                 1 |                                                                                                              |
+============ Launch Statistics ===========+                                                                                                              |
| Attempted Landings:                   96 |                                                                                                              |
| Successful Landings:                  81 |                                                                                                              |
| Failed Landings:                      14 |                                                                                                              |
| Consecutive Landings:                  1 |                                                                                                              |
+=========== Landing Statistics ===========+                                                                                                              |
| Launch Type:                  Commercial |                                                                                                              |
| Provider CEO:                  Elon Musk |                                                                                                              |
| Founded:                            2002 |                                                                                                              |
+========= Additional Information =========+                                                                                                              |
| Status:                           \x1b[32mOnline\x1b[0m |                                                                                                              |
| Webcast:                          \x1b[32mOnline\x1b[0m |                                                                                                              |
| Telemetry:                        \x1b[32mOnline\x1b[0m |                                                                                                              |
| Last Update:    1 minute, 32 seconds ago |                                                                                                              |
| NextLaunch V1.0.0 (Phoenix Rising)       |                                                                                                              |
+=========================================+======================+====== Countdown ==+====================+===============================================+
|                                                                                                                                                         |
|                                 #####          #####  #####        ###    #####        #  #   #####        #####  #####                                 |
|                                   #            #   #  #   #   ##     #        #   ##   #  #   #   #   ##   #          #                                 |
|                                   #    #####   #   #  #   #          #      ###        #####  #   #        #####  #####                                 |
|                                   #            #   #  #   #   ##     #        #   ##      #   #   #   ##       #  #                                     |
|                                   #            #####  #####        #####  #####           #   #####        #####  #####                                 |
|                                                                                                                                                         |
+=========================================================================================================================================================+
");
        exit(1);
    } else if f.preview_2 {
        println!("\
+======================================================================= NextLaunch ======================================================================+
| Mission:              Starlink 17 | Status:                     \x1b[32mGo For Launch\x1b[0m | State:                      \x1b[32mClear\x1b[0m | Humidity:                       52% |
| Vehicle:         Falcon 9 Block 5 | Window Open:  Feb 2nd 2021 - 10:14:00 GMT | Wind Direction:  North North East | Air Pressure:           1024.5 mBar |
| Location:      Launch Complex 39A | Window Close: Feb 2nd 2021 - 10:14:00 GMT | Wind Speed:                 \x1b[32m3 MPH\x1b[0m | Temperature:                   12 C |
| Provider:                  SpaceX | T-0:          Feb 2nd 2021 - 10:14:00 GMT | Wind Bearing:          27 Degrees | Minimum Temperature:            7 C |
| Destination:      Low Earth Orbit | Likelihood:                           \x1b[32m70%\x1b[0m | Visibility:              14 Miles | Maximum Temperatrue:           14 C |
+=========== Mission Info ==========+======+======== Status Info ===============+========================== Weather Information ==========================+
| Total Launch Count:                  122 | View on:                                                                                                     |
| Successful Launches:                 113 | Flight Club:      \x1b[34m{}272f4f21-0998-4078-9094-d1867dcb897d\x1b[0m                    |
| Failed Launches:                       8 | Space Launch Now: \x1b[34m{}falcon-9-block-5-starlink-17\x1b[0m                              |
| Pending Launches:                     35 | Go 4 Liftoff:     \x1b[34m{}falcon-9-block-5-starlink-17\x1b[0m                                 |
| Consecutive Successes:                 1 | Nextlaunch Web:   \x1b[34m{}falcon-9-block-5-starlink-17\x1b[0m                                  |
+============ Launch Statistics ===========+                                                                                                              |
| Attempted Landings:                   96 |                                                                                                              |
| Successful Landings:                  81 |                                                                                                              |
| Failed Landings:                      14 |                                                                                                              |
| Consecutive Landings:                  1 |                                                                                                              |
+=========== Landing Statistics ===========+                                                                                                              |
| Launch Type:                  Commercial |                                                                                                              |
| Provider CEO:                  Elon Musk |                                                                                                              |
| Founded:                            2002 | Velocity: [XXXXXXXXXXXXXXXXXXXXXXX                                                           ]     7165 km/h |
+========= Additional Information =========+======================================+============================ Flight Events ============================+
| Status:                           \x1b[32mOnline\x1b[0m |                                      | Time                                                             Name |
| Webcast:                          \x1b[32mOnline\x1b[0m | Distance Travelled:          5.47 Km | T+ 156 seconds                                     Main Engine Cutoff |
| Telemetry:                        \x1b[32mOnline\x1b[0m | Acceleration:                18.5 Gs | T+ 91 seconds                                             Throttle Up |
| Last Update:    1 minute, 32 seconds ago | Angle:                    52 Degrees | T+ 74 seconds                                                   Max Q |
| NextLaunch V1.0.0 (Phoenix Rising)       | Aerodynamic Pressure: 30,479 Pascals | T+ 53 seconds                                           Throttle Down |
+=========================================+======================+====== Countdown ==+====================+===============================================+
|                                                                                                                                                         |
|                                 #####          #####  #####        ###    #####        #  #   #####        #####  #####                                 |
|                                   #      #     #   #  #   #   ##     #        #   ##   #  #   #   #   ##   #          #                                 |
|                                   #    #####   #   #  #   #          #      ###        #####  #   #        #####  #####                                 |
|                                   #      #     #   #  #   #   ##     #        #   ##      #   #   #   ##       #  #                                     |
|                                   #            #####  #####        #####  #####           #   #####        #####  #####                                 |
|                                                                                                                                                         |
+=========================================================================================================================================================+",
                 FC_BASE, SLN_BASE, G4L_BASE, NL_BASE);
        exit(1);
    }

}