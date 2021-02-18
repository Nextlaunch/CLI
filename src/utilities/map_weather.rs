use crate::constants::weather::*;

pub fn map_weather(code: u32, day: bool) -> [&'static str; 5] {
    return match code {
        1000 => {
            if day {
                SUNNY
            } else {
                CLEAR
            }
        }
        1273 => THUNDERY_SHOWERS,
        1276 => THUNDER_HEAVY_RAIN,
        1282 => THUNDERY_SNOW_SHOWERS,
        1003 => PARTLY_CLOUDY,
        1255 => LIGHT_SNOW_SHOWERS,
        1006 | 1009 => CLOUDY,
        1030 | 1135 => FOG,
        1222 | 1258 => HEAVY_SNOW_SHOWERS,
        1204 | 1207 => LIGHT_SLEET,
        1249 | 1252 => LIGHT_SLEET_SHOWERS,
        1114 | 1210 | 1213 => LIGHT_SNOW,
        1153 | 1183 | 1189 => LIGHT_RAIN,
        1117 | 1219 | 1219 | 1225 => HEAVY_SNOW,
        1150 | 1180 | 1186 | 1240 => LIGHT_SHOWERS,
        1192 | 1195 | 1243 | 1246 => HEAVY_RAIN,
        _ => UNKNOWN
    };
}