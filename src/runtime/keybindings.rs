use crossterm::event::{poll, Event, read, KeyCode, KeyModifiers};
use std::time::Duration;
use std::sync::{Arc, Mutex};
use std::process::exit;
use crossterm::ExecutableCommand;
use crossterm::terminal::{Clear, ClearType};
use crate::runtime::state::State;

pub fn launch_thread(
    state: Arc<Mutex<State>>
)
{
    std::thread::spawn(move || {
        loop {
            match poll(Duration::from_millis(250 as u64)) {
                Ok(is_ready) => {
                    if is_ready {
                        let raw_event = read();
                        if let Ok(event) = raw_event {
                            match event {
                                Event::Key(raw_key) => {
                                    match raw_key.code {
                                        KeyCode::Esc => {
                                            if state.lock().unwrap().render_help {
                                                state.lock().unwrap().render_help = false;
                                                state.lock().unwrap().render_settings = false;
                                                state.lock().unwrap().should_clear = true;
                                            }
                                        }
                                        KeyCode::Enter => {
                                            state.lock().unwrap().open_selected = true;
                                        }
                                        KeyCode::Up => {
                                            if state.lock().unwrap().selected_side == 0 {
                                                let limit = &state.lock().unwrap().launch_update_count;
                                                let mut current = state.lock().unwrap().selected_update.clone();

                                                if current - 1 > 0 {
                                                    current -= 1;
                                                } else {
                                                    current = limit.clone();
                                                }
                                                state.lock().unwrap().selected_update = current;
                                            } else {
                                                let limit = &state.lock().unwrap().news_article_count;
                                                let mut current = state.lock().unwrap().selected_article.clone();

                                                if current - 1 > 0 {
                                                    current -= 1;
                                                } else {
                                                    current = limit.clone();
                                                }
                                                state.lock().unwrap().selected_article = current;
                                            }
                                        }
                                        KeyCode::Down => {
                                            if state.lock().unwrap().selected_side == 0 {
                                                let limit = state.lock().unwrap().launch_update_count.clone();
                                                let mut current = state.lock().unwrap().selected_update.clone();

                                                if current + 1 >= limit {
                                                    current += 1;
                                                } else {
                                                    current = limit.clone();
                                                }
                                                state.lock().unwrap().selected_update = current;
                                            } else {
                                                let limit = state.lock().unwrap().news_article_count.clone();
                                                let mut current = state.lock().unwrap().selected_article.clone();

                                                if current + 1 >= limit {
                                                    current += 1;
                                                } else {
                                                    current = limit.clone();
                                                }
                                                state.lock().unwrap().selected_article = current;
                                            }
                                        }
                                        KeyCode::Right => {
                                            if !&state.lock().unwrap().render_settings {
                                                let mut side = state.lock().unwrap().selected_side.clone();
                                                if side == 0 {
                                                    side = 1;
                                                } else {
                                                    side = 0;
                                                }
                                                state.lock().unwrap().selected_side = side;
                                            } else {
                                                let mut side = state.lock().unwrap().settings_pane.clone();
                                                if side < 5 {
                                                    side += 1;
                                                } else {
                                                    side = 0;
                                                }
                                                state.lock().unwrap().settings_pane = side;
                                            }
                                        }
                                        KeyCode::Left => {
                                            if !&state.lock().unwrap().render_settings {
                                                let mut side = state.lock().unwrap().selected_side.clone();
                                                if side == 0 {
                                                    side = 1;
                                                } else {
                                                    side = 0;
                                                }
                                                state.lock().unwrap().selected_side = side;
                                            } else {
                                                let mut side = state.lock().unwrap().settings_pane.clone();
                                                if side > 0 {
                                                    side -= 1;
                                                } else {
                                                    side = 5;
                                                }
                                                state.lock().unwrap().settings_pane = side;
                                            }
                                        }
                                        KeyCode::F(no) => {
                                            match no {
                                                1 => {
                                                    if !&state.lock().unwrap().render_help {
                                                        state.lock().unwrap().should_clear = true;
                                                        state.lock().unwrap().render_help = true;
                                                    }
                                                }
                                                _ => {}
                                            }
                                        }
                                        KeyCode::Char(c) => {
                                            match c {
                                                '1' => {
                                                    state.lock().unwrap().view_screen = 0;
                                                    state.lock().unwrap().should_clear = true;
                                                }
                                                '2' => {
                                                    state.lock().unwrap().view_screen = 1;
                                                    state.lock().unwrap().should_clear = true;
                                                }
                                                '?' => {
                                                    if !&state.lock().unwrap().render_help {
                                                        state.lock().unwrap().should_clear = true;
                                                        state.lock().unwrap().render_help = true;
                                                    }
                                                }
                                                's' => {
                                                    if !&state.lock().unwrap().render_settings {
                                                        state.lock().unwrap().should_clear = true;
                                                        state.lock().unwrap().render_settings = true;
                                                    } else {
                                                        state.lock().unwrap().should_clear = true;
                                                        state.lock().unwrap().render_settings = false;
                                                    }
                                                }
                                                'c' => {
                                                    if raw_key.modifiers.contains(KeyModifiers::CONTROL) {
                                                        let mut stdout = std::io::stdout();
                                                        let _ = stdout.execute(Clear(ClearType::All));
                                                        println!(" Thank you for using NextLaunch, goodbye.");
                                                        let _ = crossterm::terminal::disable_raw_mode();
                                                        exit(0);
                                                    }
                                                }
                                                'q' => {
                                                    if !raw_key.modifiers.contains(KeyModifiers::CONTROL) {
                                                        let mut stdout = std::io::stdout();
                                                        let _ = stdout.execute(Clear(ClearType::All));
                                                        println!(" Thank you for using NextLaunch, goodbye.");
                                                        let _ = crossterm::terminal::disable_raw_mode();
                                                        exit(0);
                                                    }
                                                }
                                                _ => {}
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }
                Err(_) => {}
            }
        }
    });
}