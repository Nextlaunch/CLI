use crate::runtime::data::launches::structures::Launch;
use crate::runtime::state::State;
use chrono::Utc;
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans, Text};
use tui::widgets::{Block, Borders, Paragraph};
use webbrowser::BrowserOptions;

pub fn render(
    state: &mut State,
    news_dimensions: (u16, u16),
    launch: Launch,
) -> Paragraph<'static> {
    let lines: Vec<Spans> = vec![];

    let lsp = launch.launch_service_provider.unwrap();

    let logo = lsp.logo;
    let mut lines: Vec<Spans> = vec![];

    for parts in logo {
        for line in parts {
            lines.push(Spans::from(vec![Span::styled(
                format!(" {}", line),
                Style::default()
            )]));
        }
    }

    Paragraph::new(lines).block(Block::default().title(" Logo ").borders(Borders::ALL))
}
