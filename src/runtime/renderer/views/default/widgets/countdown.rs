use tui::widgets::{Paragraph, Block, Wrap, Borders};
use tui::layout::Alignment;
use tui::style::{Style, Color};
use tui::text::{Span, Spans};
use crate::utilities::TimeFrame;
use crate::languages::LanguagePack;
use crate::runtime::data::launches::structures::Launch;

pub fn render_dynamic(language: &LanguagePack, timespan: TimeFrame, launch: Launch) -> Paragraph<'static> {
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
        raw[8] = generate_language_bar(language, timespan);
        raw
    } else {
        let mut raw = crate::utilities::digit_map::map_str(format!("{:02}:{:02}:{:02}", timespan.hours, timespan.minutes, timespan.seconds).as_str());
        raw[8] = generate_language_bar(language, timespan);
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
        .block(Block::default().title(format!(" {} ", language.title.countdown)).borders(Borders::ALL))
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

fn generate_language_bar(language: &LanguagePack, time: TimeFrame) -> String {
    let mut text = "".to_string();

    let defaults = {
        digit: "#####   #####   ".len(),
        length: "    Hours                 Minutes                Seconds   ".len();
    };

    if time.days > 0 {
        if time.days == 1 {
            text = format!("{}{}  ", text, center(language.time.day, defaults.digit))
        } else {
            text = format!("{}{}  ", text, center(language.time.day_plural, defaults.digit))
        }
    }

    if time.hours == 1 {
        text = format!("{}{}  ", text, center(language.time.hour, defaults.digit))
    } else {
        text = format!("{}{}  ", text, center(language.time.hour_plural, defaults.digit))
    }

    if time.minutes == 1 {
        text = format!("{}{}  ", text, center(language.time.minute, defaults.digit))
    } else {
        text = format!("{}{}  ", text, center(language.time.minute_plural, defaults.digit))
    }

    if time.seconds == 1 {
        text = format!("{}{}  ", text, center(language.time.second, defaults.digit))
    } else {
        text = format!("{}{}  ", text, center(language.time.second_plural, defaults.digit))
    }

    format("{}\u{200b}", text)
}

fn center(word: &str, width: usize) -> &str {
    let mut left = "".to_string();
    let mut right = "".to_string();

    let middle = width/2;

    if middle % 2 == 1 {
        left = "+".to_string();
    }

    let half_word = word.len()/2;

    while left.len() < middle-half_word {
        left = format!("-{}", left);
    }
    while right.len() < middle-half_word {
        right = format!("-{}", right);
    }

    format!("{}{}{}", left, word, right)
}