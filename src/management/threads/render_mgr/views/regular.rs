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

pub async fn process(i: Option<RenderFrame>, out: &mut Terminal<CrosstermBackend<Stdout>>) {
    let mut s = stdout();

    let g4l = Text::styled("Go For Launch", Style::default().fg(Color::Green));
    let tbc = Text::styled("To Be Confirmed", Style::default().fg(Color::LightYellow));
    let tbd = Text::styled("To Be Determined", Style::default().fg(Color::Cyan));
    let suc = Text::styled("Launch Successful", Style::default().fg(Color::LightGreen));
    let fal = Text::styled("Launch Failure", Style::default().fg(Color::Red));
    let hol = Text::styled("On Hold", Style::default().fg(Color::Gray));
    let inf = Text::styled("In Flight", Style::default().fg(Color::LightGreen));
    let paf = Text::styled("Partial Failure", Style::default().fg(Color::LightYellow));

    if i.is_some() {
        let frame = i.unwrap();

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
                            Constraint::Max(9),
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
                    Row::new(vec![" Name", "10 km Flight"]),
                    Row::new(vec![" Provider", "SpaceX"]),
                    Row::new(vec![" Vehicle", "Starship Prototype"]),
                    Row::new(vec![" Pad", "Launch Pad A"]),
                    Row::new(vec![" Location", "SpaceX Space Launch Facility, TX, USA"]),
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
                    Row::new(vec![" Description", "Partly Cloudy"]),
                    Row::new(vec![" Temp (c)", "10.6"]),
                    Row::new(vec![" Feels Like (c)", " 8.4", weather_icon[0]]),
                    Row::new(vec![" Wind (KM/H)", "20.2", weather_icon[1]]),
                    Row::new(vec![" Gusts (KM/H)", "20.2", weather_icon[2]]),
                    Row::new(vec![" Direction", "North", weather_icon[3]]),
                    Row::new(vec![" Bearing", "350", weather_icon[4]]),
                    Row::new(vec![" Humidity", "65%"]),
                    Row::new(vec![" Cloud Cover", "50%"]),
                    Row::new(vec![" Visibility (KM)", "16"]),
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
                    "#####       #        #####   #####        #####   #####        #   #   #####",
                    "#   #       #   ##   #   #   #       ##       #       #   ##   #   #       #",
                    "#   #       #   ##   #   #   #       ##       #       #   ##   #   #       #",
                    "#   #       #        #   #   #####        #####     ###        #####       #",
                    "#   #       #   ##   #   #       #   ##   #           #   ##       #       #",
                    "#   #       #   ##   #   #       #   ##   #           #   ##       #       #",
                    "#####       #        #####   #####        #####   #####            #       #",
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
    }
}