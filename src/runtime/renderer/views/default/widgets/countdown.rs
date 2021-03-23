use tui::widgets::{Paragraph, Block, Wrap, Borders};
use tui::layout::Alignment;
use crate::utilities::TimeFrame;
use tui::style::{Style, Color};
use tui::text::{Span, Spans};
use crate::runtime::data::launches::structures::Launch;

pub fn render_dynamic(timespan: TimeFrame, launch: Launch) -> Paragraph<'static> {
    let time_highlight = match launch.status.id.clone() {
        None => Style::default().fg(Color::DarkGray),
        Some(status) => {
            match status {
                1 => Style::default().fg(Color::Green),
                2 => Style::default().fg(Color::Yellow),
                3 => Style::default().fg(Color::LightGreen),
                4 => Style::default().fg(Color::Red),
                5 => Style::default().fg(Color::Gray),
                6 => Style::default().fg(Color::LightGreen),
                7 => Style::default().fg(Color::LightYellow),
                8 => Style::default().fg(Color::LightYellow),
                _ => Style::default().fg(Color::DarkGray)
            }
        }
    };

    let mut net = if timespan.days > 0 {
        let mut raw = crate::utilities::digit_map::map_str(format!("{:02}:{:02}:{:02}:{:02}", timespan.days, timespan.hours, timespan.minutes, timespan.seconds).as_str());
        raw[8] = "    Days                   Hours                 Minutes                Seconds   \u{200b}".to_string();

        raw
    } else {
        let mut raw = crate::utilities::digit_map::map_str(format!("{:02}:{:02}:{:02}", timespan.hours, timespan.minutes, timespan.seconds).as_str());
        raw[8] = "    Hours                 Minutes                Seconds   \u{200b}".to_string();

        raw
    };
    net.reverse();
    let mut raw_clock = net.to_vec();
    raw_clock.push("".to_string());
    raw_clock.reverse();

    let final_piece = raw_clock.pop();

    let mut clock = Vec::<Spans>::new();

    for line in raw_clock {
        clock.push(Spans::from(vec![
            Span::styled(line, time_highlight),
        ]))
    }
    clock.push(Spans::from(vec![
        Span::styled(final_piece.unwrap(), Style::default().fg(Color::White)),
    ]));


    Paragraph::new(clock)
        .block(Block::default().title(" Countdown ").borders(Borders::ALL))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: false })
}

pub fn render_blank() -> Paragraph<'static> {
    Paragraph::new(vec![
        "",
        "#####   #####        #####   #####        #####   #####",
        "#   #   #   #   ##   #   #   #   #   ##   #   #   #   #",
        "#   #   #   #   ##   #   #   #   #   ##   #   #   #   #",
        "#   #   #   #        #   #   #   #        #   #   #   #",
        "#   #   #   #   ##   #   #   #   #   ##   #   #   #   #",
        "#   #   #   #   ##   #   #   #   #   ##   #   #   #   #",
        "#####   #####        #####   #####        #####   #####",
        "",
    ].join("\n"))
        .block(Block::default().title(" Countdown ").borders(Borders::ALL))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: false })
}