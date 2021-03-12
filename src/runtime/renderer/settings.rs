use crate::runtime::renderer::centered_rect;
use tui::text::Text;
use tui::widgets::{Row, Block, Borders, Table, Clear};
use tui::layout::{Constraint, Layout, Direction};
use tui::style::Style;
use tui::style::Color;
use tui::Frame;
use std::io::Stdout;
use tui::backend::CrosstermBackend;
use crate::settings::{Config, compute_style};
use crate::runtime::state::State;
use std::iter::FromIterator;

pub fn menu(f: &mut Frame<CrosstermBackend<Stdout>>, settings: &mut Config, state: &mut State) {
    let area = centered_rect(100, 100, f.size());
    f.render_widget(Clear, area);

    let mut headers: Vec<Text> = vec![
        Text::raw(" Launch Info"),
        Text::raw("News"),
        Text::raw("Updates"),
        Text::raw("Logs"),
        Text::raw("Countdown"),
    ];

    for (id, text) in headers.iter_mut().enumerate() {
        if id as u8 == state.settings_pane {
            text.patch_style(Style::default().fg(Color::Magenta));
        }
    }


    let halves = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Min(3),
                Constraint::Min(30),
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

    match state.settings_pane {
        1 => {
            f.render_widget(tab_2(settings), halves[1])
        }
        _ => {
            f.render_widget(tab_1(settings), halves[1])
        }
    };
}

pub fn tab_1(settings: &mut Config) -> Table {
    let enabled = Style::default().fg(Color::Green);
    let disabled = Style::default().fg(Color::Red);
    let module_states = if settings.saved.modules.launch_info.enabled {
        (Text::styled("True", enabled), Text::raw("False"))
    } else {
        (Text::raw("True"), Text::styled("False", disabled))
    };
    Table::new(vec![
        Row::new(vec![Text::styled(" General Settings", Style::default().fg(Color::Magenta)), Text::raw("")]),
        Row::new(vec![Text::raw(" Enabled"), Text::raw(""), module_states.0, module_states.1]),
        Row::new(vec![" Cache Update Frequency".to_string(), "".to_string(), "".to_string(), format!("{} Seconds", settings.saved.cache_update_frequency)]),
        Row::new(vec![Text::raw("")]),
        Row::new(vec![Text::styled(" Color Settings", Style::default().fg(Color::Magenta)), Text::raw("")]),
        Row::new(vec![Text::styled("   Status", Style::default().fg(Color::Magenta)), Text::raw(""), Text::styled("Foreground", Style::default().fg(Color::Magenta)), Text::styled("Background", Style::default().fg(Color::Magenta))]),
        Row::new(vec![Text::raw("    Success"), Text::styled("SAMPLE", compute_style(&settings.saved.colors.status.suc)), Text::raw(capitalize(&settings.saved.colors.status.suc.fg.color)), Text::raw(capitalize(&settings.saved.colors.status.suc.bg.color))]),
        Row::new(vec![Text::raw("    Go For Liftoff"), Text::styled("SAMPLE", compute_style(&settings.saved.colors.status.g4l)), Text::raw(capitalize(&settings.saved.colors.status.g4l.fg.color)), Text::raw(capitalize(&settings.saved.colors.status.g4l.bg.color))]),
        Row::new(vec![Text::raw("    To Be Determined"), Text::styled("SAMPLE", compute_style(&settings.saved.colors.status.tbd)), Text::raw(capitalize(&settings.saved.colors.status.tbd.fg.color)), Text::raw(capitalize(&settings.saved.colors.status.tbd.bg.color))]),
        Row::new(vec![Text::raw("    To Be Confirmed"), Text::styled("SAMPLE", compute_style(&settings.saved.colors.status.tbc)), Text::raw(capitalize(&settings.saved.colors.status.tbc.fg.color)), Text::raw(capitalize(&settings.saved.colors.status.tbc.bg.color))]),
        Row::new(vec![Text::raw("    Partial Failure"), Text::styled("SAMPLE", compute_style(&settings.saved.colors.status.paf)), Text::raw(capitalize(&settings.saved.colors.status.paf.fg.color)), Text::raw(capitalize(&settings.saved.colors.status.paf.bg.color))]),
        Row::new(vec![Text::raw("    Failure"), Text::styled("SAMPLE", compute_style(&settings.saved.colors.status.fal)), Text::raw(capitalize(&settings.saved.colors.status.fal.fg.color)), Text::raw(capitalize(&settings.saved.colors.status.fal.bg.color))]),
        Row::new(vec![Text::raw("    In Flight"), Text::styled("SAMPLE", compute_style(&settings.saved.colors.status.inf)), Text::raw(capitalize(&settings.saved.colors.status.inf.fg.color)), Text::raw(capitalize(&settings.saved.colors.status.inf.bg.color))]),
        Row::new(vec![Text::raw("    Fetching"), Text::styled("SAMPLE", compute_style(&settings.saved.colors.status.fetching)), Text::raw(capitalize(&settings.saved.colors.status.fetching.fg.color)), Text::raw(capitalize(&settings.saved.colors.status.fetching.bg.color))]),
    ])
        .widths(&[
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ])
        .block(Block::default().borders(Borders::ALL))
}

pub fn tab_2(settings: &mut Config) -> Table {
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