use crate::utilities::map_weather::map_weather;
use crate::runtime::data::launches::structures::{Launch, LSP, Rocket, RocketConfiguration, LaunchPad, PadLocation, Article};
use crate::runtime::data::launches::update;
use crate::runtime::flags::Flags;

use std::process::exit;
use std::io::{Stdout, stdout};
use std::iter::FromIterator;

use tokio::time::{sleep, Duration, Instant};

use tui::Terminal;
use tui::backend::CrosstermBackend;
use tui::layout::{Layout, Direction, Constraint, Alignment};
use tui::widgets::{Block, Borders, Row, Table, Paragraph, Wrap};
use tui::text::{Text, Span, Spans};
use tui::style::{Style, Color, Modifier};
use tui::buffer::Cell;
use tui::style::Color::Yellow;

use crossterm::terminal::{ClearType, Clear};
use crossterm::ExecutableCommand;
use crossterm::style::Colorize;

use chrono::{Utc, DateTime, Local};

pub fn run(mut out: Terminal<CrosstermBackend<Stdout>>, launch_present: bool, i: &Option<Launch>, news: &Option<Vec<Article>>, log: &Vec<(DateTime<Local>, String, u8)>) {
    let suc = Text::styled("Launch Successful", Style::default().fg(Color::LightGreen));
    let tbd = Text::styled("To Be Determined", Style::default().fg(Color::Cyan));
    let tbc = Text::styled("To Be Confirmed", Style::default().fg(Color::LightYellow));
    let paf = Text::styled("Partial Failure", Style::default().fg(Color::LightYellow));
    let fal = Text::styled("Launch Failure", Style::default().fg(Color::Red));
    let g4l = Text::styled("Go For Launch", Style::default().fg(Color::Green));
    let inf = Text::styled("In Flight", Style::default().fg(Color::LightGreen));
    let hol = Text::styled("On Hold", Style::default().fg(Color::Gray));
    let fetching = Text::raw("Fetching...");

    let mut parsed_logs = vec![];

    for (time, message, level) in log {
        let (lvl, style) = match level {
            0 => ("INFO".to_string(), Style::default().fg(Color::Gray)),
            1 => ("ERROR".to_string(), Style::default().fg(Color::Red)),
            2 => ("WARN".to_string(), Style::default().fg(Color::Yellow)),
            _ => ("INFO".to_string(), Style::default().fg(Color::Gray))
        };

        parsed_logs.push(
            Row::new(
                vec![
                    Span::raw(time.format(" %Y/%b/%d %H:%M").to_string()),
                    Span::styled(lvl, style),
                    Span::raw(message.clone()),
                ]
            )
        )
    }

    let articles = news.clone().unwrap_or(vec![]);

    let mut processed_articles: Vec<Spans> = vec![];
    let mut artindex = 0;
    for article in articles {
        let untitle = article.title.unwrap_or("Unkown Title".to_string());

        let raw_title = if untitle.len() > 60 {
            let (headline, _) = untitle.split_at(57);
            format!("{}...", headline)
        } else {
            untitle
        };

        let title = if artindex == 0 {
            Span::styled(
                format!(" {}\n",
                        raw_title
                ),
                Style::default().fg(Color::LightBlue),
            )
        } else {
            Span::raw(
                format!(" {}\n",
                        raw_title
                )
            )
        };
        artindex += 1;
        processed_articles.push(
            Spans::from(
                vec![
                    title
                ]
            )
        );
        processed_articles.push(
            Spans::from(
                vec![
                    Span::styled(
                        format!("  {}\n",
                                article.newsSite.unwrap_or("Unkown Publisher".to_string()
                                )
                        ),
                        Style::default().fg(
                            Color::Magenta
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

        let net = crate::utilities::digit_map::map_str(format!("{:02}:{:02}:{:02}:{:02}:{:02}:{:02}", timespan.years, timespan.weeks, timespan.days, timespan.hours, timespan.minutes, timespan.seconds).as_str());

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
                        (tbd, Style::default().fg(Color::Cyan))
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

        out.draw(|f| {
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

            // let right_status = Layout::default()
            //     .direction(Direction::Vertical)
            //     .constraints(
            //         [
            //             Constraint::Percentage(75),
            //             Constraint::Percentage(25),
            //         ]
            //             .as_ref(),
            //     )
            //     .split(right[1]);

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
            let launch_table = Table::new(vec![
                Row::new(vec![" ", ""]),
                Row::new(vec![" Name", raw_name.as_str()]),
                Row::new(vec![" Provider".to_string(), lsp.name.unwrap_or("Unknown Provider".to_string())]),
                Row::new(vec![" Vehicle".to_string(), vehicle.name.unwrap_or("Unknown Launch Vehicle".to_string())]),
                Row::new(vec![" Payload", payload]),
                Row::new(vec![" Pad".to_string(), launchpad.name.unwrap_or("Unkown Launchpad".to_string())]),
                Row::new(vec![" Location".to_string(), launchpad.location.name.unwrap_or("Unkown Location".to_string())]),
                Row::new(vec![Text::from(" Status"), status]),
                Row::new(vec![" ", ""]),
            ])
                .widths(&[
                    Constraint::Min(10),
                    Constraint::Min(45)
                ])
                .block(Block::default().title(" Launch Info ").borders(Borders::from_iter(vec![Borders::LEFT, Borders::TOP, Borders::RIGHT])));


            f.render_widget(launch_table, left[0]);

            let weather_icon = map_weather(1003, true);

            // let weather_table = Table::new(vec![
            //     Row::new(vec![" ", ""]),
            //     Row::new(vec![" Description", "Fetching..."]),
            //     Row::new(vec![" Temp (c)", "Fetching..."]),
            //     Row::new(vec![" Feels Like (c)", "Fetching...", weather_icon[0]]),
            //     Row::new(vec![" Wind (KM/H)", "Fetching...", weather_icon[1]]),
            //     Row::new(vec![" Gusts (KM/H)", "Fetching...", weather_icon[2]]),
            //     Row::new(vec![" Direction", "Fetching...", weather_icon[3]]),
            //     Row::new(vec![" Bearing", "Fetching...", weather_icon[4]]),
            //     Row::new(vec![" Humidity", "Fetching..."]),
            //     Row::new(vec![" Cloud Cover", "Fetching..."]),
            //     Row::new(vec![" Visibility (KM)", "Fetching..."]),
            // ])
            //     .widths(&[
            //         Constraint::Percentage(30),
            //         Constraint::Percentage(30),
            //         Constraint::Percentage(30)
            //     ])
            //     .block(Block::default().title(" Launch Site Weather Info ").borders(Borders::from_iter(vec![Borders::LEFT, Borders::TOP, Borders::RIGHT])));
            //
            // f.render_widget(weather_table, left[1]);


            let news = Paragraph::new(processed_articles)
                .block(Block::default().title(" News ").borders(Borders::from_iter(vec![Borders::TOP, Borders::RIGHT])));
            // f.render_widget(news, right_status[0]);
            f.render_widget(news, right[1]);


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

            let countdown = Paragraph::new(
                Text::styled(net.join("\n"), time_highlight)
            )
                .block(Block::default().title(" Countdown ").borders(Borders::ALL))
                .alignment(Alignment::Center)
                .wrap(Wrap { trim: false });
            f.render_widget(countdown, whole[1]);
        });
    } else {
        out.draw(|f| {
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

            // let right_status = Layout::default()
            //     .direction(Direction::Vertical)
            //     .constraints(
            //         [
            //             Constraint::Percentage(75),
            //             Constraint::Percentage(25),
            //         ]
            //             .as_ref(),
            //     )
            //     .split(right[1]);

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

            let launch_table = Table::new(vec![
                Row::new(vec![" ", ""]),
                Row::new(vec![" Name", "Fetching..."]),
                Row::new(vec![" Provider", "Fetching..."]),
                Row::new(vec![" Vehicle", "Fetching..."]),
                Row::new(vec![" Pad", "Fetching..."]),
                Row::new(vec![" Location", "Fetching..."]),
                Row::new(vec![Text::from(" Status"), tbc.clone()]),
                Row::new(vec![" ", ""]),
            ])
                .widths(&[
                    Constraint::Min(10),
                    Constraint::Min(45)
                ])
                .block(Block::default().title(" Launch Info ").borders(Borders::from_iter(vec![Borders::LEFT, Borders::TOP, Borders::RIGHT])));


            f.render_widget(launch_table, left[0]);

            let news = Paragraph::new(processed_articles)
                .block(Block::default().title(" News ").borders(Borders::from_iter(vec![Borders::TOP, Borders::RIGHT])));
            // f.render_widget(news, right_status[0]);
            f.render_widget(news, right[1]);

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
                "#####   #####        #####   #####        #####   #####        #####   #####        #####   #####        #####   #####",
                "#   #   #   #   ##   #   #   #   #   ##   #   #   #   #   ##   #   #   #   #   ##   #   #   #   #   ##   #   #   #   #",
                "#   #   #   #   ##   #   #   #   #   ##   #   #   #   #   ##   #   #   #   #   ##   #   #   #   #   ##   #   #   #   #",
                "#   #   #   #        #   #   #   #        #   #   #   #        #   #   #   #        #   #   #   #        #   #   #   #",
                "#   #   #   #   ##   #   #   #   #   ##   #   #   #   #   ##   #   #   #   #   ##   #   #   #   #   ##   #   #   #   #",
                "#   #   #   #   ##   #   #   #   #   ##   #   #   #   #   ##   #   #   #   #   ##   #   #   #   #   ##   #   #   #   #",
                "#####   #####        #####   #####        #####   #####        #####   #####        #####   #####        #####   #####",
                "",
            ].join("\n"))
                .block(Block::default().title(" Countdown ").borders(Borders::ALL))
                .alignment(Alignment::Center)
                .wrap(Wrap { trim: false });
            f.render_widget(countdown, whole[1]);
        });
    }
}