use crate::runtime::data::launches::structures::{Launch, Article};
use crate::languages::LanguagePack;
use crate::runtime::renderer::{render_help_menu, render_qr};
use crate::settings::Config;
use crate::runtime::state::State;

use std::io::Stdout;

use tui::Terminal;
use tui::backend::CrosstermBackend;
use tui::layout::{Layout, Direction, Constraint};
use tui::widgets::Clear;

use chrono::{Utc, DateTime, Local};


mod widgets;


pub fn run(
    language: &LanguagePack,
    out: &mut Terminal<CrosstermBackend<Stdout>>,
    launch_present: bool,
    i: &Option<Launch>,
    news: &Option<Vec<Article>>,
    log: &Vec<(DateTime<Local>, String, u8)>,
    mut state: State,
    _settings: &mut Config,
) {
    let mut news_dimensions = (0, 0);
    let mut update_dimensions = (0, 0);

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
        let right = Layout::default().direction(Direction::Horizontal)
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

        news_dimensions = (right_status[0].width, right_status[0].height);
        update_dimensions = (left[1].width, left[1].height);
    });

    if launch_present {
        let launch = i.clone().unwrap();


        let timespan = crate::utilities::countdown(launch.net.clone().unwrap_or(Utc::now().to_string()));

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

            // Render launch widget ("Launch Info")
            f.render_widget(widgets::launch_info::render_dynamic(&language, launch.clone()), left[0]);

            // Render logs widget ("Logs")
            f.render_widget(widgets::system_logs::render(&language, log), right_status[1]);

            // Render dynamic countdown widget ("Countdown")
            f.render_widget(widgets::countdown::render_dynamic(timespan, launch.clone()), whole[1]);

            // Render dynamic news widget ("News")
            f.render_widget(Clear, right_status[0]);
            if state.show_logo {
                f.render_widget(widgets::logo::render(&mut state, news_dimensions, launch.clone()), right_status[0]);
            } else {
                f.render_widget(widgets::news_articles::render(&mut state, news_dimensions, news.clone().unwrap_or(vec![])), right_status[0]);
            
            }

            // Render dynamic launch update widget ("Updates")
            f.render_widget(Clear, left[1]);
            if state.show_stats { 
                f.render_widget(widgets::lsp_stats::render_list(&mut state, launch.clone()), left[1]);
            } else {
                f.render_widget(widgets::launch_updates::render_list(&mut state, launch.clone()), left[1]);
            }

            if state.render_help {
                render_help_menu(f);
            } else if state.render_qr {
                render_qr(f, format!("https://spacelaunchnow.me/launch/{}", launch.clone().slug.unwrap_or("".to_string())))
            }
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

            f.render_widget(widgets::launch_info::render_missing(), left[0]);
            f.render_widget(Clear, right[1]);
            f.render_widget(widgets::news_articles::render(&mut state, news_dimensions, news.clone().unwrap_or(vec![])), right[1]);
            f.render_widget(widgets::system_logs::render(log), left[1]);
            f.render_widget(widgets::countdown::render_blank(), whole[1]);

            if state.render_help {
                render_help_menu(f);
            } else if state.render_qr {
                render_qr(f, format!("https://spacelaunchnow.me/"))
            }
        });
    }
}

