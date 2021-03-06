use std::env::args;
use std::process::exit;

pub async fn check_flags() -> Flags {
    let args: Vec<String> = args().collect();

    let mut flags = Flags {
        view: 0,
        help: false,
        sixel: false,
        sixel_preview: false,
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
    pub sixel: bool,
    pub sixel_preview: bool,
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
        Prints help information.

    -v, --version
        Prints version information.

    -c, --credits
        Prints the credits of all who helped influence the program.

    -s, --sxl
        Informs the program to render output using the Sixel format.
        This standard is not well supported, so please ensure your terminal
        supports it, lest you get a screen full of garbage output.
        For a list of compatible terminals, use "-sxl-ls"

        --sxl-ls
        Prints a list of Sixel compatible terminals to the user, as well as
        a "test" sixel for them to check against

    -o, --offline
        Forces the program to try and operate from locally cached information.
"#, crate::NAME, crate::DESCRIPTION, crate::VERSION, crate::AUTHOR);
        exit(0);
    } else if f.version {
        println!(
            r#"{}

Version {}
"#, crate::NAME, crate::VERSION);
        exit(0);
    } else if f.credits {
        println!(
            r#"{} - Credits

Data Providers:
News:      Space Flight News API <https://thespacedevs.com/snapi>
Launches:  Launch Library 2      <https://thespacedevs.com/llapi>
Telemetry: Launch Dashboard      <https://github.com/shahar603/Launch-Dashboard-API>
Weather:   WeatherAPI            <https://www.weatherapi.com/>

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
        exit(0);
    }
}
