use tui::style::Style;

pub struct Config {
    pub unsaved: SubConfig,
    pub saved: SubConfig,
}

pub struct SubConfig {
    pub language: String,
    pub api_token: String,
    pub cache_update_frequency: usize,
    pub modules: ModuleConfig,
    pub colors: ColorConfig,
}

pub struct ModuleConfig {
    pub launch_info: LaunchInfoModuleConfig,
    pub updates: UpdateModuleConfig,
    pub news: NewsModuleConfig,
    pub logs: LogModuleConfig,
    pub countdown: CountdownModuleConfig,
}

pub struct LaunchInfoModuleConfig {
    pub enabled: bool,
}

pub struct UpdateModuleConfig {
    pub enabled: bool,
}

pub struct NewsModuleConfig {
    pub enabled: bool,
}

pub struct LogModuleConfig {
    pub enabled: bool,
    pub show_help_log: bool,
    pub show_level: bool,
    pub show_time: bool,
    pub date_format_id: u8,
}

pub struct CountdownModuleConfig {
    pub enabled: bool,
    pub formate: String,
}

pub struct ColorConfig {
    pub launch_info: LaunchInfoColorConfig,
    pub updates: UpdateColorConfig,
    pub news: NewsColorConfig,
    pub logs: LogColorConfig,
    pub countdown: CountdownColorConfig,

}

pub struct LaunchInfoColorConfig {}

pub struct UpdateColorConfig {}

pub struct NewsColorConfig {}

pub struct LogColorConfig {}

pub struct CountdownColorConfig {}

pub struct RawStyle {
    pub computed: Style,
    pub fg: RawColor,
    pub bg: RawColor,
    pub modifiers: RawModifiers,
}

pub struct RawColor {
    pub color: String,
    pub light: bool,
}

pub struct RawModifiers {
    pub bold: bool,
    pub dim: bool,
    pub italic: bool,
    pub underlined: bool,
    pub slow_blink: bool,
    pub rapid_blink: bool,
    pub reversed: bool,
    pub hidden: bool,
    pub crossed_out: bool,
}