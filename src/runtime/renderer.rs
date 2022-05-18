use crate::languages::LanguagePack;
use crate::runtime::data::launches::structures::{Article, Launch};
use crate::runtime::state::State;
use crate::settings::Config;
use chrono::{DateTime, Local};
use crossterm::terminal::{Clear, ClearType};
use crossterm::ExecutableCommand;
use std::io::Stdout;
use std::sync::{Arc, Mutex};
use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::Text;
use tui::widgets::{Block, Borders, Clear as Blank, Row, Table};
use tui::{Frame, Terminal};

pub mod settings;
pub mod views;

pub async fn process(
    language: &LanguagePack,
    i: &Option<Launch>,
    news: &Option<Vec<Article>>,
    log: &Vec<(DateTime<Local>, String, u8)>,
    has_changed: bool,
    state: &Arc<Mutex<State>>,
    settings: &mut Config,
) {
    let mut stdout = std::io::stdout();

    if has_changed || state.lock().unwrap().should_clear {
        let _ = stdout.execute(Clear(ClearType::All));
    }

    let backend = CrosstermBackend::new(stdout);
    let mut out: Terminal<CrosstermBackend<Stdout>> = Terminal::new(backend).unwrap();

    let view = state.lock().unwrap().view_screen.clone();
    let render_help = state.lock().unwrap().render_settings.clone();
    if render_help {
        let _ = out.draw(|f| {
            render_settings_menu(f, settings, state);
        });
    } else {
        let launch_present = i.is_some();
        if cfg!(debug_assertions) {
            match view {
                0 => views::default::run(
                    language,
                    &mut out,
                    launch_present,
                    i,
                    news,
                    log,
                    state.lock().unwrap().clone(),
                    settings,
                ),
                // 1 => views::deep_dive::run(&mut out, launch_present, i, state.lock().unwrap().clone(), settings),
                _ => views::default::run(
                    language,
                    &mut out,
                    launch_present,
                    i,
                    news,
                    log,
                    state.lock().unwrap().clone(),
                    settings,
                ),
            }
        } else {
            match view {
                _ => views::default::run(
                    language,
                    &mut out,
                    launch_present,
                    i,
                    news,
                    log,
                    state.lock().unwrap().clone(),
                    settings,
                ),
            }
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
    let area = centered_rect(75, 100, f.size());
    f.render_widget(Blank, area);

    let help_menu = Table::new(vec![
        Row::new(vec![
            Text::styled(" Debug Information", Style::default().fg(Color::Magenta)),
            Text::raw(""),
        ]),
        Row::new(vec![" Version".to_string(), format!("{}", crate::VERSION)]),
        Row::new(vec![" Author".to_string(), format!("{}", crate::AUTHOR)]),
        Row::new(vec!["", ""]),
        Row::new(vec![
            Text::styled(" Data Providers", Style::default().fg(Color::Magenta)),
            Text::raw(""),
        ]),
        Row::new(vec![" Launches", "The Space Devs"]),
        Row::new(vec![" ", "<thespacedevs.com>"]),
        Row::new(vec![" News", "Spaceflight News API (SNAPI)"]),
        Row::new(vec!["", "<spaceflightnewsapi.net>"]),
        Row::new(vec!["", ""]),
        Row::new(vec![" Toggle Help", "F1"]),
        Row::new(vec![" Toggle Help", "?"]),
        Row::new(vec![
            Text::styled(" General Commands", Style::default().fg(Color::Magenta)),
            Text::raw(""),
        ]),
        Row::new(vec![" Toggle LSP Info", "F2"]),
        Row::new(vec![" Update Cache", "F5"]),
        Row::new(vec![" Toggle Settings", "S"]),
        Row::new(vec![" Exit", "Q"]),
        Row::new(vec![" Exit", "CTRL+C"]),
        Row::new(vec!["", ""]),
        Row::new(vec![
            Text::styled(" Selection Commands", Style::default().fg(Color::Magenta)),
            Text::raw(""),
        ]),
        Row::new(vec![" Move Left", "LEFT ARROW"]),
        Row::new(vec![" Move Right", "RIGHT ARROW"]),
        Row::new(vec![" Move Up", "UP ARROW"]),
        Row::new(vec![" Move Down", "DOWN ARROW"]),
        Row::new(vec![" Open Selected Article", "ENTER"]),
        Row::new(vec![" Open Selected Update", "ENTER"]),
    ])
    .widths(&[Constraint::Percentage(50), Constraint::Percentage(80)])
    .header(
        Row::new(vec![" Command", "Key Binding"])
            .style(Style::default().add_modifier(Modifier::UNDERLINED)),
    )
    .block(Block::default().title(" Help ").borders(Borders::ALL));

    f.render_widget(Blank, area);
    f.render_widget(help_menu, area);
}

pub fn render_qr(f: &mut Frame<CrosstermBackend<Stdout>>, link: String) {
    let mut area = centered_rect(100, 100, f.size());
    area.x = 0;
    area.y = 0;
    area.width = 40;
    area.height = 21;

    f.render_widget(Blank, area);

    let artscii = qr2term::generate_qr_string(link).unwrap();
    let artscii_lines = artscii.split("\n").collect::<Vec<&str>>();

    f.render_widget(Blank, area);

    // Temporarily disable RAW mode (allows ANSI to function)
    let _ = crossterm::terminal::disable_raw_mode();

    println!("\x1b[0;0H\x1b[0;0F");

    println!("\x1b[2A┌─────────── Scan this QR Code ───────────┐");
    // println!("\x1b[2A┌─────── Scan this QR Code ───────┐");

    for line in artscii_lines {
        if line.len() > 0 {
            println!("│{}│", line);

        }
    }

    // println!("└───── To track on the move ──────┘");
    println!("└───────── To track on the move ──────────┘");

    // Reenable RAW mode until the next frame is drawn (resumes keybind mapping)
    let _ = crossterm::terminal::enable_raw_mode();
}

pub fn render_settings_menu(
    f: &mut Frame<CrosstermBackend<Stdout>>,
    settings: &mut Config,
    state: &Arc<Mutex<State>>,
) {
    settings::menu(f, settings, state)
}
