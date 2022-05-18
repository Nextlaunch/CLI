use crate::runtime::data::launches::structures::Launch;
use tui::widgets::{Paragraph, Borders, Block};
use tui::text::{Spans, Span};
use crate::utilities::countdown;
use chrono::Utc;
use webbrowser::BrowserOptions;
use tui::style::{Style, Color};
use crate::languages::LanguagePack;
use crate::runtime::state::State;

pub fn render_list(language: LanguagePack, state: &mut State, launch: Launch) -> Paragraph<'static> {
    let mut updates: Vec<Spans> = vec![];

    let mut update_index = 0;
    // let mut updates_used_space = 0;
    for update in launch.updates.unwrap_or(vec![]) {
        if update_index <= 2 {
            let timespan = countdown(update.created_on.unwrap_or(Utc::now().to_string()));
            let untitled = update.comment.unwrap_or("Comment not found".to_string());

            let timestr = if timespan.days > 0 {
                if timespan.days > 1 || timespan.days == 0 {
                    format!("Updated {} days ago", timespan.days)
                } else {
                    format!("Updated {} day ago", timespan.days)
                }
            } else if timespan.hours > 0 {
                if (timespan.hours > 1 || timespan.hours == 0) && (timespan.minutes > 1 || timespan.minutes == 0) {
                    format!("Updated {} hours {} minutes ago", timespan.hours, timespan.minutes)
                } else if timespan.hours > 1 && timespan.minutes == 0 {
                    format!("Updated {} hours {} minute ago", timespan.hours, timespan.minutes)
                } else if timespan.hours == 1 && timespan.minutes > 1 {
                    format!("Updated {} hour {} minutes ago", timespan.hours, timespan.minutes)
                } else {
                    format!("Updated {} hour {} minute ago", timespan.hours, timespan.minutes)
                }
            } else if timespan.minutes > 0 {
                if (timespan.minutes > 1 || timespan.minutes == 0) && (timespan.seconds > 1 || timespan.seconds == 0) {
                    format!("Updated {} minutes {} seconds ago", timespan.minutes, timespan.seconds)
                } else if timespan.minutes > 1 && timespan.seconds == 1 {
                    format!("Updated {} minutes {} second ago", timespan.minutes, timespan.seconds)
                } else if timespan.minutes == 1 && timespan.seconds > 1 {
                    format!("Updated {} minute {} seconds ago", timespan.minutes, timespan.seconds)
                } else {
                    format!("Updated {} minute {} second ago", timespan.minutes, timespan.seconds)
                }
            } else {
                if timespan.seconds > 1 || timespan.seconds == 0 {
                    format!("Updated {} seconds ago", timespan.seconds)
                } else {
                    format!("Updated {} second ago", timespan.seconds)
                }
            };

            if state.selected_side == 0 && update_index == state.selected_update {
                if state.open_selected && update.info_url.is_some() {
                    let _ = webbrowser::open_browser_with_options(
                        BrowserOptions {
                            url: update.info_url.unwrap(),
                            suppress_output: Some(true),
                            browser: Some(webbrowser::Browser::Default),
                        });
                }
                updates.push(Spans::from(vec![
                    Span::styled(format!(" {}", update.created_by.unwrap_or("Unknown author".to_string())), Style::default().fg(Color::Magenta)),
                    Span::raw(" - "),
                    Span::styled(untitled, Style::default().fg(Color::Cyan))
                ]));
                // } else if side == 1 && update_index == selected_update {
                //     updates.push(Spans::from(vec![
                //         Span::styled(format!(" {}", update.created_by.unwrap_or("Unknown author".to_string())), Style::default().fg(Color::Magenta)),
                //         Span::raw(" - "),
                //         Span::styled(untitled, Style::default().fg(Color::Magenta))
                //     ]));
            } else {
                updates.push(Spans::from(vec![
                    Span::styled(format!(" {}", update.created_by.unwrap_or("Unknown author".to_string())), Style::default().fg(Color::Magenta)),
                    Span::raw(" - "),
                    Span::raw(untitled)
                ]));
            }
            update_index += 1;
            updates.push(Spans::from(vec![
                Span::styled(format!(" {}", timestr), Style::default().fg(Color::DarkGray))
            ]));

            updates.push(Spans::from(vec![
                Span::raw("")
            ]));
        }
    }

    if updates.is_empty() {
        Paragraph::new(" This launch does not have any updates yet.")
            .block(Block::default().title(format!(" {} ", language.titles.updates))
                .borders(Borders::ALL))
    } else {
        Paragraph::new(updates)
            .block(Block::default().title(format!(" {} ", language.titles.updates))
                .borders(Borders::ALL))
    }
}
