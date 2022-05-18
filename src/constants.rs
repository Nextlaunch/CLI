pub use metadata::*;

// metadata
mod metadata {
    /// The version of the application (should be in sync with `Cargo.toml`)
    pub const VERSION: &str = "2.2.0";

    /// The name of the program
    pub const NAME: &str = "Nextlaunch";

    /// The program author
    pub const AUTHOR: &str = "Thomas B. <tom.b.2k2@gmail.com>";

    /// The description of the program
    pub const DESCRIPTION: &str = "Watch a countdown until the next rocket launch, live in your terminal!";


    // data
    /// The api route that the program uses to fetch launch data
    #[cfg(debug_assertions)]
    pub const LAUNCH_API: &str = "https://lldev.thespacedevs.com/2.2.0/launch/upcoming/?format=json&mode=detailed&limit=5&hide_recent_previous=true";
    #[cfg(not(debug_assertions))]
    pub const LAUNCH_API: &str = "https://ll.thespacedevs.com/2.2.0/launch/upcoming/?format=json&mode=detailed&limit=5&hide_recent_previous=true";

    /// The api route that the program uses to fetch telemetry information
    // pub const TELEMETRY_API: &str = "ws://api.launchdashboard.space";

    /// The API where the program will get its news from
    pub const NEWS_API: &str = "https://api.spaceflightnewsapi.net/v3/articles";

    // pub const WEATHER_API: &str = "https://aviationweather.gov/adds/dataserver_current/httpparam?dataSource=metars&requestType=retrieve&format=xml&hoursBeforeNow=10&stationString=";
    // pub const FC_BASE: &str = "https://flightclub.io/result/2d?id=";
    // pub const SLN_BASE: &str = "https://spacelaunchnow.me/launch/";
    // pub const G4L_BASE: &str = "https://go4liftoff.com/launch/";
    // pub const NL_BASE: &str = "https://nextlaunch.net/forum/";
}
//
// pub mod weather {
//     pub const UNKNOWN: [&str; 5] = [
//         "    .-.      ",
//         "     __)     ",
//         "    (        ",
//         "     `-᾿     ",
//         "      •      ",
//     ];
//
//     pub const CLEAR: [&str; 5] = [
//         "             ",
//         "             ",
//         "             ",
//         "             ",
//         "             ",
//     ];
//
//
//     pub const CLOUDY: [&str; 5] = [
//         "             ",
//         "     .--.    ",
//         "  .-(    ).  ",
//         " (___.__)__) ",
//         "             ",
//     ];
//
//     pub const FOG: [&str; 5] = [
//         "             ",
//         " _ - _ - _ - ",
//         "  _ - _ - _  ",
//         " _ - _ - _ - ",
//         "             ",
//     ];
//
//     pub const HEAVY_RAIN: [&str; 5] = [
//         "     .-.     ",
//         "    (   ).   ",
//         "   (___(__)  ",
//         "  ‚ʻ‚ʻ‚ʻ‚ʻ   ",
//         "  ‚ʻ‚ʻ‚ʻ‚ʻ   ",
//     ];
//
//     pub const HEAVY_SHOWERS: [&str; 5] = [
//         " _`/\"\".-.    ",
//         "  ,\\_(   ).  ",
//         "   /(___(__) ",
//         "   ‚ʻ‚ʻ‚ʻ‚ʻ  ",
//         "   ‚ʻ‚ʻ‚ʻ‚ʻ  ",
//     ];
//
//     pub const HEAVY_SNOW: [&str; 5] = [
//         "     .-.     ",
//         "    (   ).   ",
//         "   (___(__)  ",
//         "   * * * *   ",
//         "  * * * *    ",
//     ];
//
//     pub const HEAVY_SNOW_SHOWERS: [&str; 5] = [
//         " _`/\"\".-.    ",
//         "  ,\\_(   ).  ",
//         "   /(___(__) ",
//         "    * * * *  ",
//         "   * * * *   ",
//     ];
//
//     pub const LIGHT_RAIN: [&str; 5] = [
//         "     .-.     ",
//         "    (   ).   ",
//         "   (___(__)  ",
//         "    ʻ ʻ ʻ ʻ  ",
//         "   ʻ ʻ ʻ ʻ   ",
//     ];
//
//     pub const LIGHT_SHOWERS: [&str; 5] = [
//         " _`/\"\".-.    ",
//         "  ,\\_(   ).  ",
//         "   /(___(__) ",
//         "     ʻ ʻ ʻ ʻ ",
//         "    ʻ ʻ ʻ ʻ  ",
//     ];
//
//     pub const LIGHT_SLEET: [&str; 5] = [
//         "     .-.     ",
//         "    (   ).   ",
//         "   (___(__)  ",
//         "    ʻ * ʻ *  ",
//         "   * ʻ * ʻ   ",
//     ];
//
//     pub const LIGHT_SLEET_SHOWERS: [&str; 5] = [
//         " _`/\"\".-.    ",
//         "  ,\\_(   ).  ",
//         "   /(___(__) ",
//         "     ʻ * ʻ * ",
//         "    * ʻ * ʻ  ",
//     ];
//
//     pub const LIGHT_SNOW: [&str; 5] = [
//         "     .-.     ",
//         "    (   ).   ",
//         "   (___(__)  ",
//         "    *  *  *  ",
//         "   *  *  *   ",
//     ];
//
//     pub const LIGHT_SNOW_SHOWERS: [&str; 5] = [
//         " _`/\"\".-.    ",
//         "  ,\\_(   ).  ",
//         "   /(___(__) ",
//         "     *  *  * ",
//         "    *  *  *  ",
//     ];
//
//     pub const PARTLY_CLOUDY: [&str; 5] = [
//         "   \\  /      ",
//         " _ /\"\".-.    ",
//         "   \\_(   ).  ",
//         "   /(___(__) ",
//         "             ",
//     ];
//
//     pub const SUNNY: [&str; 5] = [
//         "    \\   /    ",
//         "     .-.     ",
//         "  ‒ (   ) ‒  ",
//         "     `-᾿     ",
//         "    /   \\    ",
//     ];
//
//     pub const THUNDER_HEAVY_RAIN: [&str; 5] = [
//         "     .-.     ",
//         "    (   ).   ",
//         "   (___(__)  ",
//         "  ‚ʻ⚡ʻ‚⚡‚ʻ   ",
//         "  ‚ʻ‚ʻ⚡ʻ‚ʻ   ",
//     ];
//
//     pub const THUNDERY_SHOWERS: [&str; 5] = [
//         " _`/\"\".-.    ",
//         "  ,\\_(   ).  ",
//         "   /(___(__) ",
//         "    ⚡ʻ ʻ⚡ʻ ʻ ",
//         "    ʻ ʻ ʻ ʻ  ",
//     ];
//
//     pub const THUNDERY_SNOW_SHOWERS: [&str; 5] = [
//         " _`/\"\".-.    ",
//         "  ,\\_(   ).  ",
//         "   /(___(__) ",
//         "     *⚡ *⚡ * ",
//         "    *  *  *  ",
//     ];
//
//     pub const VERY_CLOUDY: [&str; 5] = [
//         "             ",
//         "     .--.    ",
//         "  .-(    ).  ",
//         " (___.__)__) ",
//         "             ",
//     ];
// }
