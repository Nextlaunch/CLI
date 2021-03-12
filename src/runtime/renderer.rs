use tui::{Terminal, Frame};
use std::io::Stdout;
use tui::backend::CrosstermBackend;
use crossterm::terminal::{ClearType, Clear};
use crossterm::ExecutableCommand;
use crate::runtime::data::launches::structures::{Launch, Article};
use chrono::{DateTime, Local};
use crate::languages::LanguagePack;
use tui::layout::{Rect, Layout, Direction, Constraint};
use tui::widgets::{Row, Table, Borders, Block, Clear as Blank};
use tui::style::{Color, Style, Modifier};
use tui::text::Text;

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
        let _ = stdout.execute(Clear(ClearType::All));
    }

    let backend = CrosstermBackend::new(stdout);
    let mut out: Terminal<CrosstermBackend<Stdout>> = Terminal::new(backend).unwrap();


    let launch_present = i.is_some();

    if cfg!(debug_assertions) {
        match view {
            0 => views::default::run(language, &mut out, launch_present, i, news, log, side, selected_article, selected_update, should_open, render_help),
            1 => views::deep_dive::run(&mut out, launch_present, i, render_help),
            _ => views::default::run(language, &mut out, launch_present, i, news, log, side, selected_article, selected_update, should_open, render_help),
        }
    } else {
        match view {
            _ => views::default::run(language, &mut out, launch_present, i, news, log, side, selected_article, selected_update, should_open, render_help),
        }
    }
}

pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
                .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
                .as_ref(),
        )
        .split(popup_layout[1])[1]
}

pub fn render_help_menu(f: &mut Frame<CrosstermBackend<Stdout>>) {
    let area = centered_rect(50, 80, f.size());
    f.render_widget(Blank, area);

    let help_menu = Table::new(vec![
        Row::new(vec![Text::styled(" General Commands", Style::default().fg(Color::Magenta)), Text::raw("")]),
        Row::new(vec![" Help", "F1"]),
        Row::new(vec![" Help", "?"]),
        Row::new(vec![" Exit", "Q"]),
        Row::new(vec![" Exit", "CTRL+C"]),
        Row::new(vec![" Close Popup", "ESC"]),
        Row::new(vec!["", ""]),
        Row::new(vec![Text::styled(" Selection Commands", Style::default().fg(Color::Magenta)), Text::raw("")]),
        Row::new(vec![" Move Left", "LEFT ARROW"]),
        Row::new(vec![" Move Right", "RIGHT ARROW"]),
        Row::new(vec![" Move Up", "UP ARROW"]),
        Row::new(vec![" Move Down", "DOWN ARROW"]),
        Row::new(vec![" Open Selected Article", "ENTER"]),
        Row::new(vec![" Open Selected Update", "ENTER"]),
    ])
        .widths(&[
            Constraint::Percentage(50),
            Constraint::Percentage(50)
        ])
        .header(
            Row::new(vec![" Command", "Key Binding"])
                .style(Style::default().add_modifier(Modifier::UNDERLINED))
        )
        .block(Block::default().title(" Help ").borders(Borders::ALL));

    f.render_widget(Blank, area);
    f.render_widget(help_menu, area);
}


pub fn render_settings_menu(f: &mut Frame<CrosstermBackend<Stdout>>) {
    let area = centered_rect(50, 80, f.size());
    f.render_widget(Blank, area);

    let help_menu = Table::new(vec![
        Row::new(vec![Text::styled(" General Settings", Style::default().fg(Color::Magenta)), Text::raw("")]),
        Row::new(vec![" Help", "F1"]),

    ])
        .widths(&[
            Constraint::Percentage(50),
            Constraint::Min(5),
            Constraint::Min(5)
        ])
        .header(
            Row::new(vec![" Command", "Key Binding"])
                .style(Style::default().add_modifier(Modifier::UNDERLINED))
        )
        .block(Block::default().title(" Help ").borders(Borders::ALL));

    f.render_widget(Blank, area);
    f.render_widget(help_menu, area);
}