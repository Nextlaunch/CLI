use tui::widgets::{Row, Table, Borders, Block};
use tui::text::Text;
use tui::style::{Style, Color};
use tui::layout::Constraint;
use chrono::{DateTime, Local};
use crate::languages::LanguagePack;

pub fn render(language: &LanguagePack, logs: &Vec<(DateTime<Local>, String, u8)>) -> Table<'static> {
    let mut parsed_logs = vec![];

    let mut unprocessed = logs.clone();
    unprocessed.reverse();
    for (time, message, level) in unprocessed {
        let (lvl, style) = match level {
            0 => ("INFO".to_string(), Style::default().fg(Color::Gray)),
            1 => ("ERROR".to_string(), Style::default().fg(Color::Red)),
            2 => ("WARN".to_string(), Style::default().fg(Color::Yellow)),
            10 => ("NOTE".to_string(), Style::default().fg(Color::Magenta)),
            _ => ("INFO".to_string(), Style::default().fg(Color::Gray))
        };

        parsed_logs.push(
            Row::new(
                vec![
                    Text::styled(time.format(" %Y/%b/%d %H:%M").to_string(), style),
                    Text::styled(lvl, style),
                    Text::styled(message.clone(), style),
                ]
            )
        )
    }


    Table::new(parsed_logs)
        .widths(&[
            Constraint::Min(19),
            Constraint::Min(6),
            Constraint::Min(30)
        ])
        .block(Block::default().title(format!(" {} ", language.titles.logs))
            .borders(Borders::ALL))
}