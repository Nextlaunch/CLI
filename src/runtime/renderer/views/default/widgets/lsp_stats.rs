use crate::runtime::data::launches::structures::Launch;
use tui::widgets::{Paragraph, Borders, Block};
use tui::text::{Spans, Span};
use crate::utilities::countdown;
use chrono::Utc;
use webbrowser::BrowserOptions;
use tui::style::{Style, Color};
use crate::runtime::state::State;

pub fn render_list(state: &mut State, launch: Launch) -> Paragraph<'static> {
    let mut lines: Vec<Spans> = vec![];
    
    let lsp = launch.launch_service_provider.unwrap();

    lines.push(Spans::from(vec![
        Span::styled(format!(" Abbreviation          : {}", lsp.abbrev.unwrap()), Style::default())
    ]));

    lines.push(Spans::from(vec![
        Span::styled(format!(" Administrator         : {}", lsp.administrator.unwrap()), Style::default())
    ]));

    lines.push(Spans::from(vec![
        Span::styled(format!(" Successful Launches   : {}", lsp.consecutive_successful_launches.unwrap()), Style::default())
    ]));

    lines.push(Spans::from(vec![
        Span::styled(format!(" Failed Launches       : {}", lsp.failed_launches.unwrap()), Style::default())
    ]));

    lines.push(Spans::from(vec![
        Span::styled(format!(" Founded In            : {}", lsp.founding_year.unwrap()), Style::default())
    ]));

    Paragraph::new(lines)
            .block(Block::default().title(" Launch Provider ")
                .borders(Borders::ALL))
}