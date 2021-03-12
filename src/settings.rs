use serde::{Serialize, Deserialize, Serializer};
use tui::style::{Style, Color, Modifier};

pub struct Config {
    pub unsaved: SubConfig,
    pub saved: SubConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubConfig {
    pub language: String,
    pub api_token: String,
    pub cache_update_frequency: usize,
    pub modules: ModuleConfig,
    pub colors: ColorConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleConfig {
    pub launch_info: LaunchInfoModuleConfig,
    pub updates: UpdateModuleConfig,
    pub news: NewsModuleConfig,
    pub logs: LogModuleConfig,
    pub countdown: CountdownModuleConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaunchInfoModuleConfig {
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateModuleConfig {
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewsModuleConfig {
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogModuleConfig {
    pub enabled: bool,
    pub show_help_log: bool,
    pub show_level: bool,
    pub show_time: bool,
    pub date_format_id: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CountdownModuleConfig {
    pub enabled: bool,
    pub format: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorConfig {
    pub updates: UpdateColorConfig,
    pub articles: NewsColorConfig,
    pub logs: LogColorConfig,
    pub countdown: CountdownColorConfig,
    pub status: StatusColorConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusColorConfig {
    pub suc: RawStyle,
    pub tbd: RawStyle,
    pub tbc: RawStyle,
    pub paf: RawStyle,
    pub fal: RawStyle,
    pub g4l: RawStyle,
    pub inf: RawStyle,
    pub hol: RawStyle,
    pub fetching: RawStyle,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaunchInfoColorConfig {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateColorConfig {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewsColorConfig {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogColorConfig {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CountdownColorConfig {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawStyle {
    pub fg: RawColor,
    pub bg: RawColor,
    pub modifiers: RawModifiers,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawColor {
    pub color: String,
    pub light: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

pub fn compute_style(raw: &RawStyle) -> Style {
    let mut style = Style::default();

    style = match raw.fg.light {
        false => {
            match raw.fg.color.as_str() {
                "green" => style.fg(Color::Green),
                "red" => style.fg(Color::Red),
                "yellow" => style.fg(Color::Yellow),
                "gray" => style.fg(Color::Gray),
                "dark-gray" => style.fg(Color::DarkGray),
                "white" => style.fg(Color::White),
                "magenta" => style.fg(Color::Magenta),
                "cyan" => style.fg(Color::Cyan),
                _ => style
            }
        }
        true => {
            match raw.fg.color.as_str() {
                "green" => style.fg(Color::LightGreen),
                "red" => style.fg(Color::LightRed),
                "yellow" => style.fg(Color::LightYellow),
                "gray" => style.fg(Color::White),
                "dark-gray" => style.fg(Color::Gray),
                "white" => style.fg(Color::White),
                "magenta" => style.fg(Color::LightMagenta),
                "cyan" => style.fg(Color::LightCyan),
                _ => style
            }
        }
    };

    style = match raw.bg.light {
        false => {
            match raw.bg.color.as_str() {
                "green" => style.bg(Color::Green),
                "red" => style.bg(Color::Red),
                "yellow" => style.bg(Color::Yellow),
                "gray" => style.bg(Color::Gray),
                "dark-gray" => style.bg(Color::DarkGray),
                "white" => style.bg(Color::White),
                "magenta" => style.bg(Color::Magenta),
                "cyan" => style.bg(Color::Cyan),
                _ => style
            }
        }
        true => {
            match raw.bg.color.as_str() {
                "green" => style.bg(Color::LightGreen),
                "red" => style.bg(Color::LightRed),
                "yellow" => style.bg(Color::LightYellow),
                "gray" => style.bg(Color::White),
                "dark-gray" => style.bg(Color::Gray),
                "white" => style.bg(Color::White),
                "magenta" => style.bg(Color::LightMagenta),
                "cyan" => style.bg(Color::LightCyan),
                _ => style
            }
        }
    };

    if raw.modifiers.bold {
        style.add_modifier(Modifier::BOLD);
    }
    if raw.modifiers.dim {
        style.add_modifier(Modifier::DIM);
    }
    if raw.modifiers.italic {
        style.add_modifier(Modifier::ITALIC);
    }
    if raw.modifiers.underlined {
        style.add_modifier(Modifier::UNDERLINED);
    }
    if raw.modifiers.slow_blink {
        style.add_modifier(Modifier::SLOW_BLINK);
    }
    if raw.modifiers.rapid_blink {
        style.add_modifier(Modifier::RAPID_BLINK);
    }
    if raw.modifiers.reversed {
        style.add_modifier(Modifier::REVERSED);
    }
    if raw.modifiers.hidden {
        style.add_modifier(Modifier::HIDDEN);
    }
    if raw.modifiers.crossed_out {
        style.add_modifier(Modifier::CROSSED_OUT);
    }


    style
}

pub mod defaults;

pub fn import() -> Config {
    let root = dirs_2::data_dir().unwrap();
    let cfg_path = format!("{}/nextlaunch/config.json", root.to_str().unwrap());
    println!("Reading config file");
    println!("{}", cfg_path);
    let raw_file = std::fs::OpenOptions::new().read(true).open(cfg_path);

    let raw: SubConfig = if let Ok(file) = raw_file {
        let parsed: serde_json::Result<SubConfig> = serde_json::from_reader(file);

        if let Ok(done) = parsed {
            done
        } else {
            serde_json::from_str(defaults::DEFAULT).unwrap()
        }
    } else {
        println!("Nextlaunch failed to open the configuration file, resorting to defaults");
        serde_json::from_str(defaults::DEFAULT).unwrap()
    };

    Config {
        unsaved: raw.clone(),
        saved: raw,
    }
}