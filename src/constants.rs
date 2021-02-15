pub use metadata::*;

// metadata
mod metadata {
    /// The version of the application (should be in sync with `Cargo.toml`)
    pub const VERSION: &str = "2.0.0 (Phoenix Rising)";

    /// The name of the program
    pub const NAME: &str = "Next Launch";

    /// The program author
    pub const AUTHOR: &str = "Thomas B. <tom.b.2k2@gmail.com>";

    /// The description of the program
    pub const DESCRIPTION: &str = "Watch a countdown until the next rocket launch, live in your terminal!";


    // data
    /// The api route that the program uses to fetch launch data
    pub const LAUNCH_API: &str = "https://lldev.thespacedevs.com/2.2.0/launch/upcoming/?format=json&mode=detailed&limit=5";

    /// The api route that the program uses to fetch telemetry information
    pub const TELEMETRY_API: &str = "ws://api.launchdashboard.space";

    ///
    pub const NEWS_API: &str = "https://spaceflightnewsapi.net/api/v2/articles";
    pub const WEATHER_API: &str = "https://aviationweather.gov/adds/dataserver_current/httpparam?dataSource=metars&requestType=retrieve&format=xml&hoursBeforeNow=10&stationString=";
    pub const FC_BASE: &str = "https://flightclub.io/result/2d?id=";
    pub const SLN_BASE: &str = "https://spacelaunchnow.me/launch/";
    pub const G4L_BASE: &str = "https://go4liftoff.com/launch/";
    pub const NL_BASE: &str = "https://nextlaunch.org/rooms/";
}


mod controls {
    // ANSI codes based off of documentation found here:
    // https://gist.github.com/fnky/458719343aabd01cfb17a3a4f7296797


    pub const PREFIX: &str = "\x1b";

    // controls - Cursor
    pub const HOME: &str = "\x1b[H";
    pub const SAVE: &str = "\x1b[s";
    pub const RESTORE: &str = "\x1b[u";

    // controls - Screen
    pub const CLEAR: &str = "\x1b[J";
    pub const CLEAR_LINE: &str = "\x1b[K";
    pub const CLEAR_ALL: &str = "\x1b[2J";
}

mod colors {
    // ANSI codes based off of documentation found here:
// https://gist.github.com/fnky/458719343aabd01cfb17a3a4f7296797

    // controls - Color
    pub const RESET: &str = "\x1b[0m";
    pub const BRIGHT: &str = "\x1b[1m";

    pub const FG_BLACK: &str = "\x1b[30m";
    pub const FG_RED: &str = "\x1b[31m";
    pub const FG_GREEN: &str = "\x1b[32m";
    pub const FG_YELLOW: &str = "\x1b[33m";
    pub const FG_BLUE: &str = "\x1b[34m";
    pub const FG_MAGENTA: &str = "\x1b[35m";
    pub const FG_CYAN: &str = "\x1b[36m";
    pub const FG_WHITE: &str = "\x1b[37m";

    pub const BG_BLACK: &str = "\x1b[40m";
    pub const BG_RED: &str = "\x1b[41m";
    pub const BG_GREEN: &str = "\x1b[42m";
    pub const BG_YELLOW: &str = "\x1b[43m";
    pub const BG_BLUE: &str = "\x1b[44m";
    pub const BG_MAGENTA: &str = "\x1b[45m";
    pub const BG_CYAN: &str = "\x1b[46m";
    pub const BG_WHITE: &str = "\x1b[47m";
}