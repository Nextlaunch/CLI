use crate::management::data::RenderFrame;

use tui::Terminal;
use tui::backend::CrosstermBackend;
use std::io::{Stdout, stdout};
use tui::layout::{Layout, Direction, Constraint};
use tui::widgets::{Block, Borders, Row};
use crossterm::terminal::ClearType;
use crossterm::ExecutableCommand;
use tui::buffer::Cell;
use tui::style::{Style, Color};

pub async fn process(i: Option<RenderFrame>, out: &mut Terminal<CrosstermBackend<Stdout>>) {
    let mut s = stdout();
    s.execute(crossterm::terminal::Clear(ClearType::All));

    if i.is_some() {
        let frame = i.unwrap();

        if let Some(error) = frame.error {

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