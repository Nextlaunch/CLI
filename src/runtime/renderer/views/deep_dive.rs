use tui::Terminal;
use tui::backend::CrosstermBackend;
use std::io::Stdout;
use crate::runtime::data::launches::structures::{Launch, Rocket};
use tui::layout::{Layout, Direction, Constraint, Alignment, Rect};
use tui::widgets::{Clear, Block, Borders, Paragraph, Table, Row};
use tui::text::Text;

pub mod dict;

pub fn run(mut out: Terminal<CrosstermBackend<Stdout>>, launch_present: bool, i: &Option<Launch>) {
    if launch_present {
        let launch = i.clone().unwrap();
        let rocket = launch.rocket.unwrap_or(Rocket {
            id: None,
            configuration: None,
        });
        let vehicle = dict::match_rocket(rocket.id.unwrap_or(0));


        if let Some(lv) = vehicle {
            let _ = out.draw(|f| {
                let size = f.size();

                let chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([
                        Constraint::Percentage(33),
                        Constraint::Percentage(33),
                        Constraint::Percentage(33)
                    ].as_ref())
                    .split(size);

                let paragraph = Table::new(vec![
                    Row::new(vec!["", ""]),
                    Row::new(vec![" Name", &lv.name]),
                    Row::new(vec![" Manufacturer", &lv.manufacturer]),
                    Row::new(vec![" Country of Origin", &lv.country]),
                    Row::new(vec!["", ""]),
                    Row::new(vec![" Reusable", &lv.reusable]),
                    Row::new(vec![" ", &lv.country]),
                ])
                    .widths(&[
                        Constraint::Percentage(50),
                        Constraint::Percentage(50),
                    ])
                    .block(Block::default().title(" Detailed Overview ").borders(Borders::ALL));
                f.render_widget(paragraph, chunks[0]);

                let paragraph = Table::new(vec![
                    Row::new(vec!["", ""]),
                    Row::new(vec![" Stages".to_string(), format!("{}", lv.stages.len())]),
                    Row::new(vec!["", ""]),
                    Row::new(vec![" First Stage", ""]),
                    Row::new(vec!["   Known As", &lv.stages[0].name]),
                    Row::new(vec!["   Engine Count", "9"]),
                    Row::new(vec!["   Thrust (Sea Level)", &lv.stages[0].thrust_weight_sea]),
                    Row::new(vec!["   Thrust (Vacuum)", &lv.stages[0].thrust_weight_vac]),
                ])
                    .widths(&[
                        Constraint::Percentage(50),
                        Constraint::Percentage(50),
                    ])
                    .block(Block::default().title(" Launcher Overview ").borders(Borders::ALL));
                f.render_widget(paragraph, chunks[1]);

                let paragraph = Table::new(vec![
                    Row::new(vec!["", ""]),
                    Row::new(vec![" Name", &lv.name]),
                    Row::new(vec![" Manufacturer", &lv.manufacturer]),
                    Row::new(vec![" Country of Origin", &lv.country]),
                    Row::new(vec!["", ""]),
                    Row::new(vec![" Reusable", &lv.reusable]),
                    Row::new(vec![" ", &lv.country]),
                ])
                    .widths(&[
                        Constraint::Percentage(50),
                        Constraint::Percentage(50),
                    ])
                    .block(Block::default().title(" Engine Overview ").borders(Borders::ALL));
                f.render_widget(paragraph, chunks[2]);
            });
        } else {
            let _ = out.draw(|f| {
                let size = f.size();

                let chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                    .split(size);

                let paragraph = Paragraph::new(Text::raw(""))
                    .block(Block::default().title(" Detailed Overview ").borders(Borders::ALL))
                    .alignment(Alignment::Left);
                f.render_widget(paragraph, chunks[0]);

                let paragraph = Paragraph::new(Text::raw(""))
                    .block(Block::default().borders(Borders::ALL))
                    .alignment(Alignment::Left);
                f.render_widget(paragraph, chunks[1]);

                let error = Paragraph::new(Text::raw("\nThis launch vehicle does not currently have a deep dive view implemented.\nPlease check back in a future version for deep dive."))
                    .block(Block::default().title(" Error ").borders(Borders::ALL))
                    .alignment(Alignment::Center);

                let area = centered_rect(60, 20, size);

                f.render_widget(Clear, area); //this clears out the background
                f.render_widget(error, area);
            });
        }
    } else {
        let _ = out.draw(|f| {
            let size = f.size();

            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(size);

            let paragraph = Paragraph::new(Text::raw(""))
                .block(Block::default().borders(Borders::ALL))
                .alignment(Alignment::Left);
            f.render_widget(paragraph, chunks[0]);

            let paragraph = Paragraph::new(Text::raw(""))
                .block(Block::default().title("Detailed Overview").borders(Borders::ALL))
                .alignment(Alignment::Left);
            f.render_widget(paragraph, chunks[1]);

            let error = Paragraph::new(Text::raw("There is currently no launch available.\nCheck this screen again once one is available."))
                .block(Block::default().title("Popup").borders(Borders::ALL))
                .alignment(Alignment::Left);

            let area = centered_rect(60, 20, size);

            f.render_widget(Clear, area); //this clears out the background
            f.render_widget(error, area);
        });
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
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