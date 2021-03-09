use std::env::args;
use std::process::exit;

pub async fn check_flags() -> Flags {
    let args: Vec<String> = args().collect();

    let mut flags = Flags {
        view: 0,
        help: false,
        version: false,
        credits: false,
    };

    for arg in args {
        match arg.to_lowercase().as_str() {
            "--version" | "-v" => {
                flags.version = true;
            }
            // "--json" => {
            //     flags.view = 1;
            // }
            "--help" | "-h" => {
                flags.help = true
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
    pub version: bool,
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
"#, crate::NAME, crate::DESCRIPTION, crate::VERSION, crate::AUTHOR);
        exit(0);
    } else if f.version {
        println!(
            r#"{}

Version {}
"#, crate::NAME, crate::VERSION);
        exit(0);
    } else if f.credits {
        //Weather:   WeatherAPI            <https://www.weatherapi.com/>
        println!(
            "{} - Credits
Without the following people, services, and open-source libraries, NextLaunch would not have been possible.
Thank you to everyone on this list from the bottom of my heart for helping me make this program,
and putting up with my constant requests for comments on design and style.

\x1b[32mData Providers:\x1b[0m
News:      Space Flight News API <https://thespacedevs.com/snapi>
Launches:  Launch Library 2      <https://thespacedevs.com/llapi>
Telemetry: Launch Dashboard      <https://github.com/shahar603/Launch-Dashboard-API>

\x1b[32mDeveloper:\x1b[0m AltriusRS             <https://github.com/AltriusRS>

\x1b[32mLanguage:\x1b[0m  Rust                  <https://rust-lang.org>

\x1b[32mInterface: (Alphabetical)\x1b[0m
           Accusitive            <https://github.com/accusitive>
           Jas777                <https://github.com/jas777>
           Nosu                  <https://twitter.com/Nosudrum>
           Starman               <No link specified>
           Zane                  <https://github.com/AnotherZane>

\x1b[32mAlpha Testers: (Alphabetical)\x1b[0m
           Nosu                  <https://twitter.com/Nosudrum>
           Starman               <No link specified>
", crate::NAME);
        exit(0);
    }
}
