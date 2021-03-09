use crate::utilities::map_weather::map_weather;
use crate::runtime::data::launches::structures::{Launch, LSP, Rocket, RocketConfiguration, LaunchPad, PadLocation, Article};
use crate::runtime::data::launches::update;
use crate::utilities::countdown;
use crate::runtime::flags::Flags;

use std::process::exit;
use std::io::{Stdout, stdout};
use std::iter::FromIterator;

use tokio::time::{sleep, Duration, Instant};

use tui::Terminal;
use tui::backend::CrosstermBackend;
use tui::layout::{Layout, Direction, Constraint, Alignment};
use tui::widgets::{Block, Borders, Row, Table, Paragraph, Wrap, Clear as Blank};
use tui::text::{Text, Span, Spans};
use tui::style::{Style, Color, Modifier};
use tui::buffer::Cell;
use tui::style::Color::Yellow;

use crossterm::terminal::{ClearType, Clear};
use crossterm::ExecutableCommand;
use crossterm::style::Colorize;

use chrono::{Utc, DateTime, Local};
use webbrowser::open;
use crate::languages::LanguagePack;

pub fn run(
    language: &LanguagePack,
    mut out: Terminal<CrosstermBackend<Stdout>>,
    launch_present: bool,
    i: &Option<Launch>,
    news: &Option<Vec<Article>>,
    log: &Vec<(DateTime<Local>, String, u8)>,
    side: i32,
    selected_article: i32,
    selected_update: i32,
    mut should_open: bool,
) {
    let suc = Text::styled("Launch Successful", Style::default().fg(Color::LightGreen));
    let tbd = Text::styled("To Be Determined", Style::default().fg(Color::Yellow));
    let tbc = Text::styled("To Be Confirmed", Style::default().fg(Color::LightYellow));
    let paf = Text::styled("Partial Failure", Style::default().fg(Color::LightYellow));
    let fal = Text::styled("Launch Failure", Style::default().fg(Color::Red));
    let g4l = Text::styled("Go For Launch", Style::default().fg(Color::Green));
    let inf = Text::styled("In Flight", Style::default().fg(Color::LightGreen));
    let hol = Text::styled("On Hold", Style::default().fg(Color::Gray));
    let fetching = Text::raw("Fetching...");

    let mut parsed_logs = vec![];

    let mut unprocessed = log.clone();
    unprocessed.reverse();
    for (time, message, level) in unprocessed {
        let (lvl, style) = match level {
            0 => ("INFO".to_string(), Style::default().fg(Color::Gray)),
            1 => ("ERROR".to_string(), Style::default().fg(Color::Red)),
            2 => ("WARN".to_string(), Style::default().fg(Color::Yellow)),
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

    let articles = news.clone().unwrap_or(vec![]);

    let mut processed_articles: Vec<Spans> = vec![];
    let mut artindex = 0;
    for article in articles {
        let untitle = article.title.unwrap_or("Unkown Title".to_string());

        let mut headlines: Vec<String> = vec![
            String::new()
        ];
        let mut index = 0;
        let mut line_total = 0;
        let words: Vec<&str> = untitle.split(' ').collect();

        for word in words {
            if line_total + word.len() + 1 <= 50 {
                headlines[index] = format!("{} {}", headlines[index], word);
                line_total += word.len();
            } else {
                index += 1;
                line_total = word.len() + 1;
                headlines.push(format!(" {}", word));
            }
        }

        for headline in headlines {
            if artindex == selected_article && side == 1 {
                if should_open {
                    let _ = webbrowser::open(article.url.clone().unwrap().as_str());
                    should_open = false;
                }
                processed_articles.push(
                    Spans::from(vec![
                        Span::styled(headline, Style::default().fg(Color::Cyan))
                    ])
                )
            } else if artindex == selected_article && side != 1 {
                processed_articles.push(
                    Spans::from(vec![
                        Span::styled(headline, Style::default().fg(Color::Magenta))
                    ])
                );
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

        let timestr = if timespan.weeks > 0 {
            if timespan.weeks > 1 || timespan.weeks == 0 {
                format!("Published {} weeks ago", timespan.weeks)
            } else {
                format!("Published {} week ago", timespan.weeks)
            }
        } else if timespan.days > 0 {
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
    }

    if launch_present {
        let launch = i.clone().unwrap();

        let raw_name = launch.name.unwrap_or("Unknown Launch | Unknown Payload".to_string());

        let pieces: Vec<&str> = raw_name.split(" | ").collect();

        let name = *pieces.first().unwrap_or(&"Unknown Launch");
        let payload = *pieces.last().unwrap_or(&"Unknown Payload");

        let timespan = crate::utilities::countdown(launch.net.unwrap_or(Utc::now().to_string()));

        let mut net = if timespan.years > 0 {
            let mut raw = crate::utilities::digit_map::map_str(format!("{:02}:{:02}:{:02}:{:02}:{:02}:{:02}", timespan.years, timespan.weeks, timespan.days, timespan.hours, timespan.minutes, timespan.seconds).as_str());
            raw[8] = "    Years                  Weeks                  Days                   Hours                 Minutes                Seconds   \u{200b}".to_string();

            raw
        } else if timespan.weeks > 0 {
            let mut raw = crate::utilities::digit_map::map_str(format!("{:02}:{:02}:{:02}:{:02}:{:02}", timespan.weeks, timespan.days, timespan.hours, timespan.minutes, timespan.seconds).as_str());
            raw[8] = "    Weeks                  Days                   Hours                 Minutes                Seconds   \u{200b}".to_string();

            raw
        } else if timespan.days > 0 {
            let mut raw = crate::utilities::digit_map::map_str(format!("{:02}:{:02}:{:02}:{:02}", timespan.days, timespan.hours, timespan.minutes, timespan.seconds).as_str());
            raw[8] = "    Days                   Hours                 Minutes                Seconds   \u{200b}".to_string();

            raw
        } else {
            let mut raw = crate::utilities::digit_map::map_str(format!("{:02}:{:02}:{:02}", timespan.hours, timespan.minutes, timespan.seconds).as_str());
            raw[8] = "    Hours                 Minutes                Seconds   \u{200b}".to_string();

            raw
        };

        let vehicle = launch.rocket.clone().unwrap_or(Rocket {
            id: None,
            configuration: None,
        }).configuration
            .unwrap_or(RocketConfiguration {
                id: None,
                name: None,
                description: None,
                family: None,
                full_name: None,
                manufacturer: None,
                variant: None,
                alias: None,
                min_stage: None,
                max_stage: None,
                length: None,
                diameter: None,
                maiden_flight: None,
                launch_mass: None,
                leo_capacity: None,
                gto_capacity: None,
                to_thrust: None,
                apogee: None,
                vehicle_range: None,
                total_launch_count: None,
                consecutive_successful_launches: None,
                successful_launches: None,
                failed_launches: None,
                pending_launches: None,
            });

        let lsp = launch.launch_service_provider.clone().unwrap_or(LSP {
            id: None,
            name: None,
            features: None,
            org: None,
            country_code: None,
            abbrev: None,
            description: None,
            administrator: None,
            founding_year: None,
            launchers: None,
            spacecraft: None,
            launch_library_url: None,
            total_launch_count: None,
            consecutive_successful_launches: None,
            successful_launches: None,
            failed_launches: None,
            pending_launches: None,
            consecutive_successful_landings: None,
            successful_landings: None,
            failed_landings: None,
            attempted_landings: None,
        });

        let launchpad = launch.pad.unwrap_or(LaunchPad {
            id: None,
            agency_id: None,
            name: None,
            latitude: None,
            longitude: None,
            location: PadLocation {
                id: None,
                name: None,
                country_code: None,
                total_launch_count: None,
                total_landing_count: None,
            },
            total_launch_count: None,
        });

        let (status, time_highlight) = match launch.status.id.clone() {
            None => (fetching, Style::default().fg(Color::DarkGray)),
            Some(status) => {
                match status {
                    1 => {
                        (g4l, Style::default().fg(Color::Green))
                    }
                    2 => {
                        (tbd, Style::default().fg(Color::Yellow))
                    }
                    3 => {
                        (suc, Style::default().fg(Color::LightGreen))
                    }
                    4 => {
                        (fal, Style::default().fg(Color::Red))
                    }
                    5 => {
                        (hol, Style::default().fg(Color::Gray))
                    }
                    6 => {
                        (inf, Style::default().fg(Color::LightGreen))
                    }
                    7 => {
                        (paf, Style::default().fg(Color::LightYellow))
                    }
                    8 => {
                        (tbc, Style::default().fg(Color::LightYellow))
                    }
                    _ => {
                        (fetching, Style::default().fg(Color::DarkGray))
                    }
                }
            }
        };

        let mut updates: Vec<Spans> = vec![];

        let mut update_index = 0;

        for update in launch.updates.unwrap_or(vec![]) {
            let timespan = countdown(update.created_on.unwrap_or(Utc::now().to_string()));
            let untitle = update.comment.unwrap_or("Comment not found".to_string());

            let timestr = if timespan.weeks > 0 {
                if timespan.weeks > 1 || timespan.weeks == 0 {
                    format!("Updated {} weeks ago", timespan.weeks)
                } else {
                    format!("Updated {} week ago", timespan.weeks)
                }
            } else if timespan.days > 0 {
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

            if side == 0 && update_index == selected_update {
                if should_open && update.info_url.is_some() {
                    open(update.info_url.unwrap().as_str());
                }
                updates.push(Spans::from(vec![
                    Span::styled(format!(" {}", update.created_by.unwrap_or("Unknown author".to_string())), Style::default().fg(Color::Magenta)),
                    Span::raw(" - "),
                    Span::styled(untitle, Style::default().fg(Color::Cyan))
                ]));
            } else if side == 1 && update_index == selected_update {
                updates.push(Spans::from(vec![
                    Span::styled(format!(" {}", update.created_by.unwrap_or("Unknown author".to_string())), Style::default().fg(Color::Magenta)),
                    Span::raw(" - "),
                    Span::styled(untitle, Style::default().fg(Color::Magenta))
                ]));
            } else {
                updates.push(Spans::from(vec![
                    Span::styled(format!(" {}", update.created_by.unwrap_or("Unknown author".to_string())), Style::default().fg(Color::Magenta)),
                    Span::raw(" - "),
                    Span::raw(untitle)
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

        let _ = out.draw(|f| {
            let whole = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Ratio(8, 12),
                        Constraint::Min(10),
                    ]
                        .as_ref(),
                )
                .split(f.size());

            let right = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(
                    [
                        Constraint::Percentage(50),
                        Constraint::Percentage(50),
                    ]
                        .as_ref(),
                )
                .split(whole[0]);

            let right_status = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Percentage(75),
                        Constraint::Percentage(25),
                    ]
                        .as_ref(),
                )
                .split(right[1]);

            let left = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Percentage(50),
                        Constraint::Percentage(50),
                    ]
                        .as_ref(),
                )
                .split(right[0]);
            let launch_table = Table::new(vec![
                Row::new(vec![Text::from(" Name"), Text::styled(raw_name.as_str(), Style::default().add_modifier(Modifier::UNDERLINED))]),
                Row::new(vec![" Provider".to_string(), lsp.name.unwrap_or("Unknown Provider".to_string())]),
                Row::new(vec![" Vehicle".to_string(), vehicle.name.unwrap_or("Unknown Launch Vehicle".to_string())]),
                Row::new(vec![" Mission", payload]),
                Row::new(vec![" Pad".to_string(), launchpad.name.unwrap_or("Unkown Launchpad".to_string())]),
                Row::new(vec![" Location".to_string(), launchpad.location.name.unwrap_or("Unkown Location".to_string())]),
                Row::new(vec![Text::from(" Status"), status]),
            ])
                .widths(&[
                    Constraint::Min(10),
                    Constraint::Min(45)
                ])
                .block(Block::default().title(" Launch Info ").borders(Borders::ALL));

            f.render_widget(launch_table, left[0]);


            let news = Paragraph::new(processed_articles)
                .block(Block::default().title(" News ").borders(Borders::ALL));
            // f.render_widget(news, right_status[0]);
            f.render_widget(Blank, right_status[0]);
            f.render_widget(news, right_status[0]);


           if updates.is_empty() {
               let update_list = Paragraph::new(" This launch does not have any updates yet.")
                   .block(Block::default().title(" Updates ")
                       .borders(Borders::ALL));
               f.render_widget(update_list, left[1]);
           } else {
               let update_list = Paragraph::new(updates)
                   .block(Block::default().title(" Updates ")
                       .borders(Borders::ALL));
               f.render_widget(update_list, left[1]);
           }

            let log_list = Table::new(parsed_logs)
                .widths(&[
                    Constraint::Min(19),
                    Constraint::Min(6),
                    Constraint::Min(30)
                ])
                .block(Block::default().title(" Logs ")
                    .borders(Borders::ALL));
            // f.render_widget(log_list, right_status[1]);
            f.render_widget(log_list, right_status[1]);

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


            let countdown = Paragraph::new(clock)
                .block(Block::default().title(" Countdown ").borders(Borders::ALL))
                .alignment(Alignment::Center)
                .wrap(Wrap { trim: false });
            f.render_widget(countdown, whole[1]);
        });
    } else {
        let _ = out.draw(|f| {
            let whole = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Ratio(7, 10),
                        Constraint::Min(9),
                    ]
                        .as_ref(),
                )
                .split(f.size());

            let right = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(
                    [
                        Constraint::Percentage(50),
                        Constraint::Percentage(50),
                    ]
                        .as_ref(),
                )
                .split(whole[0]);

            let left = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Max(10),
                        Constraint::Max(12),
                    ]
                        .as_ref(),
                )
                .split(right[0]);

            let launch_table = Paragraph::new(Text::raw(" Unfortunately, there is not a launch currently available.\n Please check the logs."))
                .block(Block::default().title(" Launch Info ").borders(Borders::from_iter(vec![Borders::LEFT, Borders::TOP, Borders::RIGHT])));

            f.render_widget(launch_table, left[0]);


            if processed_articles.is_empty() {
                let news = Paragraph::new(Text::raw(" There is no available news articles to display.\n Please check the logs."))
                    .block(Block::default().title(" News ").borders(Borders::from_iter(vec![Borders::TOP, Borders::RIGHT])));
                // f.render_widget(news, right_status[0]);
                f.render_widget(Blank, right[1]);
                f.render_widget(news, right[1]);
            } else {
                let news = Paragraph::new(processed_articles)
                    .block(Block::default().title(" News ").borders(Borders::from_iter(vec![Borders::TOP, Borders::RIGHT])));
                // f.render_widget(news, right_status[0]);
                f.render_widget(Blank, right[1]);
                f.render_widget(news, right[1]);
            }


            let log_list = Table::new(parsed_logs)
                .widths(&[
                    Constraint::Min(19),
                    Constraint::Min(6),
                    Constraint::Min(30)
                ])
                .block(Block::default().title(" Logs ")
                    .borders(Borders::from_iter(vec![Borders::LEFT, Borders::TOP, Borders::RIGHT])));
            // f.render_widget(log_list, right_status[1]);
            f.render_widget(log_list, left[1]);


            let countdown = Paragraph::new(vec![
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
                .wrap(Wrap { trim: false });
            f.render_widget(countdown, whole[1]);
        });
    }
}