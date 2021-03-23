use tui::widgets::{Paragraph, Block, Borders};
use crate::runtime::data::launches::structures::Article;
use tui::text::{Spans, Span, Text};
use webbrowser::BrowserOptions;
use tui::style::{Style, Color, Modifier};
use std::iter::FromIterator;
use crate::runtime::state::State;
use chrono::Utc;

pub fn render(state: &mut State, news_dimensions: (u16, u16), articles: Vec<Article>) -> Paragraph<'static> {


    // let mut news_lines_used = 0;
    let mut processed_articles: Vec<Spans> = vec![];
    let mut artindex = 0;
    for article in articles {
        // if news_lines_used < news_dimensions.1 {
        let untitle = article.title.unwrap_or("Unkown Title".to_string());
        let mut headlines: Vec<String> = vec![
            String::new()
        ];
        let mut line_total = 0;
        let mut index = 0;
        let words: Vec<&str> = untitle.split(' ').collect();

        for word in words {
            // println!("{} chars / {} chars", (line_total + word.len())+ 1, news_dimensions.0/2);
            if (line_total + word.len()) + 1 < (news_dimensions.0 as usize) - 15 {
                headlines[index] = format!("{} {}", headlines[index], word);
                line_total += word.len();
            } else {
                index += 1;
                line_total = word.len() + 1;
                headlines.push(format!(" {}", word));
            }
        }
        // exit(1);

        for headline in headlines {
            if artindex == state.selected_article && state.selected_side == 1 {
                if state.open_selected {
                    let _ = webbrowser::open_browser_with_options(
                        BrowserOptions {
                            url: article.url.clone().unwrap(),
                            suppress_output: Some(true),
                            browser: Some(webbrowser::Browser::Default),
                        });
                    state.open_selected = false;
                }
                processed_articles.push(
                    Spans::from(vec![
                        Span::styled(headline, Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
                    ])
                )
                // } else if artindex == selected_article && side != 1 {
                //     processed_articles.push(
                //         Spans::from(vec![
                //             Span::styled(headline, Style::default().fg(Color::Magenta).add_modifier(Modifier::ITALIC))
                //         ])
                //     );
            } else {
                processed_articles.push(
                    Spans::from(vec![
                        Span::raw(headline)
                    ])
                )
            }
        }
        artindex += 1;


        let timespan = crate::utilities::countdown_news(article.published_at.unwrap_or(Utc::now().to_string()));

        let timestr = if timespan.days > 0 {
            if timespan.days > 1 || timespan.days == 0 {
                format!("Published {} days ago", timespan.days)
            } else {
                format!("Published {} day ago", timespan.days)
            }
        } else if timespan.hours > 0 {
            if (timespan.hours > 1 || timespan.hours == 0) && (timespan.minutes > 1 || timespan.minutes == 0) {
                format!("Published {} hours {} minutes ago", timespan.hours, timespan.minutes)
            } else if timespan.hours > 1 && timespan.minutes == 0 {
                format!("Published {} hours {} minute ago", timespan.hours, timespan.minutes)
            } else if timespan.hours == 1 && timespan.minutes > 1 {
                format!("Published {} hour {} minutes ago", timespan.hours, timespan.minutes)
            } else {
                format!("Published {} hour {} minute ago", timespan.hours, timespan.minutes)
            }
        } else if timespan.minutes > 0 {
            if (timespan.minutes > 1 || timespan.minutes == 0) && (timespan.seconds > 1 || timespan.seconds == 0) {
                format!("Published {} minutes {} seconds ago", timespan.minutes, timespan.seconds)
            } else if timespan.minutes > 1 && timespan.seconds == 1 {
                format!("Published {} minutes {} second ago", timespan.minutes, timespan.seconds)
            } else if timespan.minutes == 1 && timespan.seconds > 1 {
                format!("Published {} minute {} seconds ago", timespan.minutes, timespan.seconds)
            } else {
                format!("Published {} minute {} second ago", timespan.minutes, timespan.seconds)
            }
        } else {
            if timespan.seconds > 1 || timespan.seconds == 0 {
                format!("Published {} seconds ago", timespan.seconds)
            } else {
                format!("Published {} second ago", timespan.seconds)
            }
        };

        processed_articles.push(
            Spans::from(
                vec![
                    Span::styled(
                        format!("  {}",
                                article.news_site.unwrap_or("Unkown Publisher".to_string())
                        ),
                        Style::default().fg(
                            Color::Magenta
                        ),
                    ),
                    Span::styled(
                        format!("  -  "),
                        Style::default().fg(
                            Color::Reset
                        ),
                    ),
                    Span::styled(
                        format!("{}                                                                                       \u{200b}\n",
                                timestr
                        ),
                        Style::default().fg(
                            Color::DarkGray
                        ),
                    )
                ]
            )
        );
        processed_articles.push(
            Spans::from(
                vec![
                    Span::raw(
                        ""
                    )
                ]
            )
        );
        // }
    }

    return if processed_articles.is_empty() {
        Paragraph::new(Text::raw(" There is no available news articles to display.\n Please check the logs."))
            .block(Block::default().title(" News ").borders(Borders::from_iter(vec![Borders::TOP, Borders::RIGHT])))
    } else {
        Paragraph::new(processed_articles)
            .block(Block::default().title(" News ").borders(Borders::from_iter(vec![Borders::TOP, Borders::RIGHT])))
    };
}