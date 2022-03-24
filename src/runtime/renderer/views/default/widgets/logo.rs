use crate::runtime::data::launches::structures::Launch;
use crate::runtime::state::State;
use tui::style::Style;
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Paragraph};

pub fn render(
    _state: &mut State,
    _news_dimensions: (u16, u16),
    launch: Launch,
) -> Paragraph<'static> {
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
