use crate::runtime::renderer::centered_rect;
use tui::text::Text;
use tui::widgets::{Row, Block, Borders, Table, Clear};
use tui::layout::{Constraint, Layout, Direction};
use tui::style::{Style, Modifier};
use tui::style::Color;
use tui::Frame;
use std::io::Stdout;
use tui::backend::CrosstermBackend;
use crate::settings::{Config/*, compute_style*/};
use crate::runtime::state::State;
use std::iter::FromIterator;
use std::sync::{Arc, Mutex};

pub fn menu(f: &mut Frame<CrosstermBackend<Stdout>>, settings: &mut Config, state: &Arc<Mutex<State>>) {
    let area = centered_rect(100, 100, f.size());
    f.render_widget(Clear, area);

    let mut headers: Vec<Text> = vec![
        Text::raw(" Launch Info"),
        // Text::raw("News"),
        // Text::raw("Updates"),
        // Text::raw("Logs"),
        // Text::raw("Countdown"),
    ];

    for (id, text) in headers.iter_mut().enumerate() {
        if id as i8 == state.lock().unwrap().settings_pane {
            text.patch_style(Style::default().fg(Color::Cyan));
        }
    }


    let halves = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Max(2),
                Constraint::Min(30),
                Constraint::Max(2),
            ]
                .as_ref(),
        )
        .split(area);

    f.render_widget(Table::new(vec![
        Row::new(headers),
    ])
                        .widths(&[
                            Constraint::Percentage(20),
                            Constraint::Percentage(20),
                            Constraint::Percentage(20),
                            Constraint::Percentage(20),
                            Constraint::Percentage(20),
                        ])
                        .block(Block::default().title(" Settings ").borders(Borders::from_iter(vec![Borders::TOP, Borders::LEFT, Borders::RIGHT]))), halves[0]);

    f.render_widget(Table::new(vec![
        Row::new(vec![
            " Close Settings - S",
            "|",
            " Save Settings - CTRL+X",
            "|",
            " Restore Default - CTRL+R",
            "|",
            " Modify Selected - ENTER",
            "|",
            " Change Value - +/-"]),
    ])
                        .widths(&[
                            Constraint::Min(22),
                            Constraint::Min(1),
                            Constraint::Min(26),
                            Constraint::Min(1),
                            Constraint::Min(28),
                            Constraint::Min(1),
                            Constraint::Min(27),
                            Constraint::Min(1),
                            Constraint::Min(22),
                        ])
                        .block(Block::default().borders(Borders::from_iter(vec![Borders::BOTTOM, Borders::LEFT, Borders::RIGHT]))), halves[2]);

    let pane = state.lock().unwrap().settings_pane.clone();

    match pane {
        // 1 => {
        //     f.render_widget(tab_2(settings, state), halves[1])
        // }
        _ => {
            f.render_widget(tab_1(settings, state), halves[1])
        }
    };
}

pub fn tab_1<'a>(settings: &'a mut Config, state: &'a Arc<Mutex<State>>) -> Table<'a> {
    let enabled = Style::default().fg(Color::Green);
    let disabled = Style::default().fg(Color::Red);
    let _module_states = if settings.saved.modules.launch_info.enabled {
        (Text::styled("True", enabled), Text::raw("False"))
    } else {
        (Text::raw("True"), Text::styled("False", disabled))
    };

    let token_states = if settings.saved.api_token.len() > 0 {
        (Text::styled("True", enabled), Text::raw("False"))
    } else {
        (Text::raw("True"), Text::styled("False", disabled))
    };

    let raw_rows: Vec<(i8, Vec<Text>)> = vec![
        (-1, vec![Text::styled(" General Settings", Style::default().fg(Color::Magenta)), Text::raw("")]),
        // (0, vec![Text::raw(" Enabled"), Text::raw(""), module_states.0, module_states.1]),
        (1, vec![Text::raw(" Cache Update Frequency"), Text::raw(""), Text::raw(""), Text::raw(format!("{} Seconds", settings.saved.cache_update_frequency))]),
        (1, vec![Text::raw(" Using Token"), Text::raw(""), token_states.0, token_states.1]),
        // (-1, vec![Text::raw("")]),
        // (-1, vec![Text::styled(" Color Settings", Style::default().fg(Color::Magenta)), Text::raw("")]),
        // (-1, vec![Text::styled("   Status", Style::default().fg(Color::Yellow)), Text::raw(""), Text::styled("Foreground", Style::default().fg(Color::Yellow)), Text::styled("Background", Style::default().fg(Color::Yellow))]),
        // (2, vec![Text::raw("    Success"), Text::styled("SAMPLE", compute_style(&settings.saved.colors.status.suc)), Text::raw(capitalize(&settings.saved.colors.status.suc.fg.color)), Text::raw(capitalize(&settings.saved.colors.status.suc.bg.color))]),
        // (3, vec![Text::raw("    Go For Liftoff"), Text::styled("SAMPLE", compute_style(&settings.saved.colors.status.g4l)), Text::raw(capitalize(&settings.saved.colors.status.g4l.fg.color)), Text::raw(capitalize(&settings.saved.colors.status.g4l.bg.color))]),
        // (4, vec![Text::raw("    To Be Determined"), Text::styled("SAMPLE", compute_style(&settings.saved.colors.status.tbd)), Text::raw(capitalize(&settings.saved.colors.status.tbd.fg.color)), Text::raw(capitalize(&settings.saved.colors.status.tbd.bg.color))]),
        // (5, vec![Text::raw("    To Be Confirmed"), Text::styled("SAMPLE", compute_style(&settings.saved.colors.status.tbc)), Text::raw(capitalize(&settings.saved.colors.status.tbc.fg.color)), Text::raw(capitalize(&settings.saved.colors.status.tbc.bg.color))]),
        // (6, vec![Text::raw("    Partial Failure"), Text::styled("SAMPLE", compute_style(&settings.saved.colors.status.paf)), Text::raw(capitalize(&settings.saved.colors.status.paf.fg.color)), Text::raw(capitalize(&settings.saved.colors.status.paf.bg.color))]),
        // (7, vec![Text::raw("    Failure"), Text::styled("SAMPLE", compute_style(&settings.saved.colors.status.fal)), Text::raw(capitalize(&settings.saved.colors.status.fal.fg.color)), Text::raw(capitalize(&settings.saved.colors.status.fal.bg.color))]),
        // (8, vec![Text::raw("    In Flight"), Text::styled("SAMPLE", compute_style(&settings.saved.colors.status.inf)), Text::raw(capitalize(&settings.saved.colors.status.inf.fg.color)), Text::raw(capitalize(&settings.saved.colors.status.inf.bg.color))]),
        // (9, vec![Text::raw("    Fetching"), Text::styled("SAMPLE", compute_style(&settings.saved.colors.status.fetching)), Text::raw(capitalize(&settings.saved.colors.status.fetching.fg.color)), Text::raw(capitalize(&settings.saved.colors.status.fetching.bg.color))]),
    ];

    let mut rows: Vec<Row> = vec![];

    if state.lock().unwrap().settings_selected > raw_rows.len() as i8 {
        state.lock().unwrap().settings_selected = 0;
    }

    for (id, cells) in raw_rows {
        rows.push(
            if id == state.lock().unwrap().settings_selected as i8 {
                Row::new(cells)
                    .style(
                        Style::default()
                            .bg(Color::DarkGray)
                            .add_modifier(Modifier::BOLD)
                    )
            } else {
                Row::new(cells)
            }
        );
    }

    Table::new(rows)
        .widths(&[
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ])
        .block(Block::default().borders(Borders::ALL))
}

pub fn tab_2<'a>(settings: &'a mut Config, _state: &'a Arc<Mutex<State>>) -> Table<'a> {
    let enabled = Style::default().fg(Color::Green);
    let disabled = Style::default().fg(Color::Red);
    let module_states = if settings.saved.modules.news.enabled {
        (Text::styled("True", enabled), Text::raw("False"))
    } else {
        (Text::raw("True"), Text::styled("False", disabled))
    };
    Table::new(vec![
        Row::new(vec![Text::styled(" General Settings", Style::default().fg(Color::Magenta)), Text::raw("")]),
        Row::new(vec![Text::raw(" Enabled"), Text::raw(""), module_states.0, module_states.1]),
        Row::new(vec![Text::raw("")]),
    ])
        .widths(&[
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ])
        .block(Block::default().borders(Borders::ALL))
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}