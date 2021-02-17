use crate::management::data::RenderFrame;

use tui::Terminal;
use tui::backend::CrosstermBackend;
use std::io::{Stdout, stdout};
use tui::layout::{Layout, Direction, Constraint};
use tui::widgets::{Block, Borders, Row, Table};
use crossterm::terminal::ClearType;
use crossterm::ExecutableCommand;
use tui::text::{Text};
use tui::style::{Style, Color, Modifier};
use crossterm::style::Colorize;
use tui::buffer::Cell;
use tui::style::Color::Yellow;

pub async fn process(i: Option<RenderFrame>, out: &mut Terminal<CrosstermBackend<Stdout>>) {
    let mut s = stdout();

    let g4l = Text::styled("                             Go For Launch", Style::default().fg(Color::Green));
    let tbc = Text::styled("                           To Be Confirmed", Style::default().fg(Color::LightYellow));
    let tbd = Text::styled("                          To Be Determined", Style::default().fg(Color::Cyan));
    let suc = Text::styled("                         Launch Successful", Style::default().fg(Color::LightGreen));
    let fal = Text::styled("                            Launch Failure", Style::default().fg(Color::Red));
    let hol = Text::styled("                                   On Hold", Style::default().fg(Color::Gray));
    let inf = Text::styled("                                 In Flight", Style::default().fg(Color::LightGreen));
    let paf = Text::styled("                           Partial Failure", Style::default().fg(Color::LightYellow));

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
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(
                        [
                            Constraint::Min(8),
                            Constraint::Min(15),
                            Constraint::Min(9),
                        ]
                            .as_ref(),
                    )
                    .split(f.size());

                let launch_table = Table::new(vec![
                    Row::new(vec![" Name", "10 km Flight", "Opens", "T- 1 day, 16 hours, 27 minutes,  1 second"]),
                    Row::new(vec![" Provider", "SpaceX", "Closes", "T- 2 days, 3 hours, 27 minutes,  5 seconds"]),
                    Row::new(vec![" Vehicle", "Starship Prototype", "Net", "T- 1 day, 18 hours, 27 minutes,  1 second", ""]),
                    Row::new(vec![Text::from(" Pad"), Text::from("Launch Pad A"), Text::from("Status"), tbc]),
                    Row::new(vec![" Location", "SpaceX Space Launch Facility, TX, USA", "", ""]),
                ])
                    .header(
                        Row::new(vec![" Mission", "", "Data", ""])
                            .style(Style::default().fg(Color::White))
                        // If you want some space between the header and the rest of the rows, you can always
                        // specify some margin at the bottom.
                    )
                    .block(Block::default().title(" Launch Info ").borders(Borders::ALL))
                    .widths(&[
                        Constraint::Length(30),
                        Constraint::Length(45),
                        Constraint::Length(30),
                        Constraint::Length(42)
                    ]);
                f.render_widget(launch_table, chunks[0]);
                let block2 = Block::default().title(" News ").borders(Borders::ALL);
                f.render_widget(block2, chunks[1]);
                let block3 = Block::default().title(" Countdown ").borders(Borders::ALL);
                f.render_widget(block3, chunks[2]);
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