use crossterm::event::{poll, Event, read, KeyCode, KeyModifiers};
use std::time::Duration;
use std::sync::{Arc, Mutex};
use std::process::exit;
use crossterm::{ExecutableCommand};
use crossterm::terminal::{Clear, ClearType};
use crate::runtime::state::State;

pub fn keybinder(state: Arc<Mutex<State>>) {
    loop {
        std::thread::sleep(Duration::from_millis(100));
        match poll(Duration::from_millis(100 as u64)) {
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
                                        } else if state.lock().unwrap().editing_settings {
                                            state.lock().unwrap().editing_settings = false;
                                            state.lock().unwrap().stored_value = vec![];
                                        }
                                    }
                                    KeyCode::Enter => {
                                        if state.lock().unwrap().editing_settings {
                                            state.lock().unwrap().editing_settings = false;
                                            state.lock().unwrap().save_stored = true;
                                        } else if state.lock().unwrap().render_settings {
                                            let frame = state.lock().unwrap().settings_pane.clone();
                                            match frame {
                                                0 => {}
                                                _ => {}
                                            }
                                        } else if !state.lock().unwrap().render_help {
                                            state.lock().unwrap().open_selected = true;
                                        }
                                    }
                                    KeyCode::Up => {
                                        if state.lock().unwrap().render_settings {
                                            state.lock().unwrap().should_clear = true;
                                            if state.lock().unwrap().settings_selected == 0 {
                                                state.lock().unwrap().should_clear = true;
                                                let pane = state.lock().unwrap().settings_pane;

                                                if pane == 1 {
                                                    state.lock().unwrap().settings_selected = 0;
                                                } else {
                                                    state.lock().unwrap().settings_selected = 9;
                                                }
                                            } else {
                                                state.lock().unwrap().settings_selected -= 1;
                                            }
                                        } else {
                                            if state.lock().unwrap().selected_side == 0 {
                                                let limit = state.lock().unwrap().launch_update_count.clone();
                                                let mut current = state.lock().unwrap().selected_update.clone();

                                                if current > 0 {
                                                    if current - 1 >= 0 {
                                                        current -= 1;
                                                    } else {
                                                        current = limit - 1;
                                                    }
                                                } else {
                                                    if current as i8 - 1 == 0 {
                                                        current = 0
                                                    } else {
                                                        current = limit - 2;
                                                    }
                                                }
                                                state.lock().unwrap().selected_update = current;
                                            } else {
                                                let limit = state.lock().unwrap().news_article_count.clone();
                                                let mut current = state.lock().unwrap().selected_article.clone();

                                                if current > 0 {
                                                    if current - 1 >= 0 {
                                                        current -= 1;
                                                    } else {
                                                        current = limit - 1;
                                                    }
                                                } else {
                                                    if current as i8 - 1 == 0 {
                                                        current = 0
                                                    } else {
                                                        current = limit - 1;
                                                    }
                                                }
                                                state.lock().unwrap().selected_article = current;
                                            }
                                        }
                                    }
                                    KeyCode::Down => {
                                        if state.lock().unwrap().render_settings {
                                            state.lock().unwrap().should_clear = true;
                                            state.lock().unwrap().settings_selected += 1;

                                            let pane = state.lock().unwrap().settings_pane;

                                            match pane {
                                                1 => {
                                                    if state.lock().unwrap().settings_selected + 1 > 0 {
                                                        state.lock().unwrap().settings_selected = 0
                                                    }
                                                }
                                                _ => {
                                                    {
                                                        if state.lock().unwrap().settings_selected + 1 > 10 {
                                                            state.lock().unwrap().settings_selected = 0
                                                        }
                                                    }
                                                }
                                            }
                                        } else {
                                            if state.lock().unwrap().selected_side == 0 {
                                                let limit = state.lock().unwrap().launch_update_count.clone();
                                                let mut current = state.lock().unwrap().selected_update.clone();

                                                if current + 1 >= limit-1 {
                                                    current = 0;
                                                } else {
                                                    current += 1;
                                                }
                                                state.lock().unwrap().selected_update = current;
                                            } else {
                                                let limit = state.lock().unwrap().news_article_count.clone();
                                                let mut current = state.lock().unwrap().selected_article.clone();

                                                if current + 1 >= limit {
                                                    current = 0;
                                                } else {
                                                    current += 1;
                                                }
                                                state.lock().unwrap().selected_article = current;
                                            }
                                        }
                                    }
                                    KeyCode::Right => {
                                        if state.lock().unwrap().render_settings {
                                            let tab = state.lock().unwrap().settings_pane.clone() as i8;
                                            state.lock().unwrap().should_clear = true;

                                            if tab + 1 >= 5 {
                                                state.lock().unwrap().settings_pane = 0;
                                            } else {
                                                state.lock().unwrap().settings_pane = (tab + 1) as u8;
                                            }
                                            state.lock().unwrap().settings_selected = 0;
                                        } else {
                                            if !state.lock().unwrap().render_settings {
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
                                    }
                                    KeyCode::Left => {
                                        if state.lock().unwrap().render_settings {
                                            let tab = state.lock().unwrap().settings_pane.clone() as i8;
                                            state.lock().unwrap().should_clear = true;

                                            if tab - 1 >= 0 {
                                                state.lock().unwrap().settings_pane = (tab - 1) as u8;
                                            } else {
                                                state.lock().unwrap().settings_pane = 4;
                                            }
                                            state.lock().unwrap().settings_selected = 0;
                                        } else {
                                            if !state.lock().unwrap().render_settings {
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
                                    }
                                    KeyCode::F(no) => {
                                        match no {
                                            1 => {
                                                if !state.lock().unwrap().render_help {
                                                    state.lock().unwrap().should_clear = true;
                                                    state.lock().unwrap().render_help = true;
                                                    state.lock().unwrap().render_settings = false;
                                                } else {
                                                    state.lock().unwrap().should_clear = true;
                                                    state.lock().unwrap().render_help = false;
                                                    state.lock().unwrap().render_settings = false;
                                                }
                                            }
                                            _ => {}
                                        }
                                    }
                                    KeyCode::Char(c) => {
                                        if state.lock().unwrap().editing_settings {
                                            state.lock().unwrap().stored_value.push(c)
                                        } else {
                                            match c {
                                                // '1' => {
                                                //     state.lock().unwrap().view_screen = 0;
                                                //     state.lock().unwrap().should_clear = true;
                                                // }
                                                // '2' => {
                                                //     state.lock().unwrap().view_screen = 1;
                                                //     state.lock().unwrap().should_clear = true;
                                                // }
                                                '?' => {
                                                    if !state.lock().unwrap().render_help {
                                                        state.lock().unwrap().should_clear = true;
                                                        state.lock().unwrap().render_help = true;
                                                        state.lock().unwrap().render_settings = false;
                                                    } else {
                                                        state.lock().unwrap().should_clear = true;
                                                        state.lock().unwrap().render_help = false;
                                                        state.lock().unwrap().render_settings = false;
                                                    }
                                                }
                                                // 's' => {
                                                //     if !state.lock().unwrap().render_settings {
                                                //         state.lock().unwrap().should_clear = true;
                                                //         state.lock().unwrap().render_help = false;
                                                //         state.lock().unwrap().render_settings = true;
                                                //     } else {
                                                //         state.lock().unwrap().should_clear = true;
                                                //         state.lock().unwrap().render_help = false;
                                                //         state.lock().unwrap().render_settings = false;
                                                //     }
                                                // }
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
}