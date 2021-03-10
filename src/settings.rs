use tui::style::Style;

pub struct Config {
    pub unsaved: SubConfig,
    pub saved: SubConfig,
}

pub struct SubConfig {
    pub language: String,
    pub api_token: String,
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

pub struct LaunchInfoModuleConfig {}

pub struct UpdateModuleConfig {}

pub struct NewsModuleConfig {}

pub struct LogModuleConfig {}

pub struct CountdownModuleConfig {}

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