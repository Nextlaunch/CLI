use crate::management::data::RenderFrame;

use tui::Terminal;
use tui::backend::CrosstermBackend;
use std::io::{Stdout, stdout};
use tui::layout::{Layout, Direction, Constraint, Alignment};
use tui::widgets::{Block, Borders, Row, Table, Paragraph, Wrap};
use crossterm::terminal::ClearType;
use crossterm::ExecutableCommand;
use tui::text::{Text};
use tui::style::{Style, Color, Modifier};
use crossterm::style::Colorize;
use tui::buffer::Cell;
use tui::style::Color::Yellow;
use std::iter::FromIterator;
use crate::utilities::map_weather::map_weather;
use std::time::Instant;


pub async fn process(i: Option<RenderFrame>, out: &mut Terminal<CrosstermBackend<Stdout>>) {
    let frame_present = i.is_some();

    let mut s = stdout();

    let g4l = Text::styled("Go For Launch", Style::default().fg(Color::Green));
    let tbc = Text::styled("To Be Confirmed", Style::default().fg(Color::LightYellow));
    let tbd = Text::styled("To Be Determined", Style::default().fg(Color::Cyan));
    let suc = Text::styled("Launch Successful", Style::default().fg(Color::LightGreen));
    let fal = Text::styled("Launch Failure", Style::default().fg(Color::Red));
    let hol = Text::styled("On Hold", Style::default().fg(Color::Gray));
    let inf = Text::styled("In Flight", Style::default().fg(Color::LightGreen));
    let paf = Text::styled("Partial Failure", Style::default().fg(Color::LightYellow));
    let inf = Text::styled("In Flight", Style::default().fg(Color::LightGreen));
    let fetching = Text::raw("Fetching...");

    if frame_present {
        let frame = i.unwrap_or(RenderFrame {
            view: 0,
            launch_refresh: Instant::now(),
            launch: None,
            telemetry: None,
            error: None,
        });

        let status = match frame.launch {
            Some(frame) => {
                match frame.launch {
                    Some(launch) => {
                        match launch.status.id {
                            None => fetching,
                            Some(status) => {
                                match status {
                                    _ => fetching
                                }
                            }
                        }
                    }
                    None => fetching
                }
            }
            None => fetching
        };

        if let Some(error) = frame.error {
            out.draw(|f| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(
                        [
                            Constraint::Percentage(10),
                            Constraint::Percentage(80),
                            Constraint::Percentage(10),
                        ]
                            .as_ref(),
                    )
                    .split(f.size());

                let block = Block::default().title("Block").borders(Borders::ALL);
                f.render_widget(block, chunks[0]);
                let block2 = Block::default().title("Block 2").borders(Borders::ALL);
                f.render_widget(block2, chunks[1]);
                let block3 = Block::default().title("Block 3").borders(Borders::ALL);
                f.render_widget(block3, chunks[2]);
                let block4 = Block::default().title("Block 4").borders(Borders::ALL);
                // f.render_widget(block4, chunks[3]);
            });
        } else {
            out.draw(|f| {
                let whole = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(
                        [
                            Constraint::Ratio(7, 10),
                            Constraint::Min(9),
                        ]
                            .as_ref(),
                    )
                    .split(f.size());
                let right = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints(
                        [
                            Constraint::Percentage(50),
                            Constraint::Percentage(50),
                        ]
                            .as_ref(),
                    )
                    .split(whole[0]);

                let left = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(
                        [
                            Constraint::Max(10),
                            Constraint::Max(12),
                        ]
                            .as_ref(),
                    )
                    .split(right[0]);

                let launch_table = Table::new(vec![
                    Row::new(vec![" ", ""]),
                    Row::new(vec![" Name", "Fetching..."]),
                    Row::new(vec![" Provider", "Fetching..."]),
                    Row::new(vec![" Vehicle", "Fetching..."]),
                    Row::new(vec![" Pad", "Fetching..."]),
                    Row::new(vec![" Location", "Fetching..."]),
                    Row::new(vec![Text::from(" Status"), tbc.clone()]),
                    Row::new(vec![" ", ""]),
                ])
                    .widths(&[
                        Constraint::Min(10),
                        Constraint::Min(45)
                    ])
                    .block(Block::default().title(" Launch Info ").borders(Borders::from_iter(vec![Borders::LEFT, Borders::TOP, Borders::RIGHT])));


                f.render_widget(launch_table, left[0]);

                let weather_icon = map_weather(1003, true);

                let weather_table = Table::new(vec![
                    Row::new(vec![" ", ""]),
                    Row::new(vec![" Description", "Fetching..."]),
                    Row::new(vec![" Temp (c)", "Fetching..."]),
                    Row::new(vec![" Feels Like (c)", "Fetching...", weather_icon[0]]),
                    Row::new(vec![" Wind (KM/H)", "Fetching...", weather_icon[1]]),
                    Row::new(vec![" Gusts (KM/H)", "Fetching...", weather_icon[2]]),
                    Row::new(vec![" Direction", "Fetching...", weather_icon[3]]),
                    Row::new(vec![" Bearing", "Fetching...", weather_icon[4]]),
                    Row::new(vec![" Humidity", "Fetching..."]),
                    Row::new(vec![" Cloud Cover", "Fetching..."]),
                    Row::new(vec![" Visibility (KM)", "Fetching..."]),
                ])
                    .widths(&[
                        Constraint::Percentage(30),
                        Constraint::Percentage(30),
                        Constraint::Percentage(30)
                    ])
                    .block(Block::default().title(" Launch Site Weather Info ").borders(Borders::from_iter(vec![Borders::LEFT, Borders::TOP, Borders::RIGHT])));

                f.render_widget(weather_table, left[1]);
                let news = Block::default().title(" News ").borders(Borders::from_iter(vec![Borders::TOP, Borders::RIGHT]));
                f.render_widget(news, right[1]);

                let countdown = Paragraph::new(vec![
                    "",
                    "#####   #####        #####   #####        #####   #####        #####   #####",
                    "#   #   #   #   ##   #   #   #   #   ##   #   #   #   #   ##   #   #   #   #",
                    "#   #   #   #   ##   #   #   #   #   ##   #   #   #   #   ##   #   #   #   #",
                    "#   #   #   #        #   #   #   #        #   #   #   #        #   #   #   #",
                    "#   #   #   #   ##   #   #   #   #   ##   #   #   #   #   ##   #   #   #   #",
                    "#   #   #   #   ##   #   #   #   #   ##   #   #   #   #   ##   #   #   #   #",
                    "#####   #####        #####   #####        #####   #####        #####   #####",
                    "",
                ].join("\n"))
                    .block(Block::default().title(" Countdown ").borders(Borders::ALL))
                    .alignment(Alignment::Center)
                    .wrap(Wrap { trim: false });
                f.render_widget(countdown, whole[1]);
            });
        }
    } else {
        out.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Percentage(10),
                        Constraint::Percentage(80),
                        Constraint::Min(10),
                    ]
                        .as_ref(),
                )
                .split(f.size());

            let block = Block::default().title("Block").borders(Borders::ALL);
            f.render_widget(block, chunks[0]);
            let block2 = Block::default().title("Block 2").borders(Borders::ALL);
            f.render_widget(block2, chunks[1]);
            let block3 = Block::default().title("Block 3").borders(Borders::ALL);
            f.render_widget(block3, chunks[2]);
            let block4 = Block::default().title("Block 4").borders(Borders::ALL);
            // f.render_widget(block4, chunks[3]);
        });
    }
}