use tui::Terminal;
use std::io::Stdout;
use tui::backend::CrosstermBackend;
use crossterm::terminal::{ClearType, Clear};
use crossterm::ExecutableCommand;
use crate::runtime::data::launches::structures::{Launch, Article};
use chrono::{DateTime, Local};
use crate::languages::LanguagePack;

pub mod views;

pub async fn process(
    language: &LanguagePack,
    i: &Option<Launch>,
    news: &Option<Vec<Article>>,
    log: &Vec<(DateTime<Local>, String, u8)>,
    has_changed: bool,
    view: i32,
    side: i32,
    selected_article: i32,
    selected_update: i32,
    should_open: bool,
    render_help: bool,
) {
    let mut stdout = std::io::stdout();


    if has_changed {
        stdout.execute(Clear(ClearType::All));
    }

    let backend = CrosstermBackend::new(stdout);
    let mut out: Terminal<CrosstermBackend<Stdout>> = Terminal::new(backend).unwrap();


    let launch_present = i.is_some();

    if cfg!(debug_assertions) {
        match view {
            0 => views::default::run(language, out, launch_present, i, news, log, side, selected_article, selected_update, should_open),
            1 => views::deep_dive::run(out, launch_present, i),
            _ => views::default::run(language, out, launch_present, i, news, log, side, selected_article, selected_update, should_open),
        }
    } else {
        match view {
            _ => views::default::run(language, out, launch_present, i, news, log, side, selected_article, selected_update, should_open),
        }
    }
}