use tui::Terminal;
use std::io::Stdout;
use tui::backend::CrosstermBackend;
use crossterm::terminal::{ClearType, Clear};
use crossterm::ExecutableCommand;
use crate::runtime::data::launches::structures::{Launch, Article};
use chrono::{DateTime, Local};

pub mod views;

pub async fn process(i: &Option<Launch>, news: &Option<Vec<Article>>, log: &Vec<(DateTime<Local>, String, u8)>, has_changed: bool, view: i32) {
    let mut stdout = std::io::stdout();

    if has_changed {
        stdout.execute(Clear(ClearType::All));
    }

    let backend = CrosstermBackend::new(stdout);
    let mut out: Terminal<CrosstermBackend<Stdout>> = Terminal::new(backend).unwrap();


    let launch_present = i.is_some();

    if cfg!(debug_assertions) {
        match view {
            0 => views::default::run(out, launch_present, i, news, log),
            1 => views::deep_dive::run(out, launch_present, i),
            _ => views::default::run(out, launch_present, i, news, log),
        }
    } else {
        match view {
            _ => views::default::run(out, launch_present, i, news, log),
        }
    }
}