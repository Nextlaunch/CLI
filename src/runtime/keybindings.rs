use crossterm::event::{poll, Event, read, KeyCode, KeyModifiers};
use std::time::Duration;
use std::sync::{Arc, Mutex};
use std::process::exit;
use crossterm::ExecutableCommand;
use crossterm::terminal::{Clear, ClearType};

pub fn launch_thread(
    view_screen2: Arc<Mutex<i32>>,
    selected_article2: Arc<Mutex<i32>>,
    selected_update2: Arc<Mutex<i32>>,
    selected_side2: Arc<Mutex<i32>>,
    should_clear2: Arc<Mutex<bool>>,
    open_selected2: Arc<Mutex<bool>>,
    render_help2: Arc<Mutex<bool>>,
    launch_update_count2: Arc<Mutex<i32>>,
    news_article_count2: Arc<Mutex<i32>>,
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
                                            if *render_help2.lock().unwrap() {
                                                *render_help2.lock().unwrap() = false;
                                                *should_clear2.lock().unwrap() = true;
                                            }
                                        }
                                        KeyCode::Enter => {
                                            *open_selected2.lock().unwrap() = true;
                                        }
                                        KeyCode::Up => {
                                            if *selected_side2.lock().unwrap() == 0 {
                                                let limit = *launch_update_count2.lock().unwrap();
                                                let mut current = *selected_update2.lock().unwrap();

                                                if current - 1 >= 0 {
                                                    current -= 1;
                                                } else {
                                                    current = limit.clone();
                                                }
                                                *selected_update2.lock().unwrap() = current;
                                            } else {
                                                let limit = *news_article_count2.lock().unwrap();
                                                let mut current = *selected_article2.lock().unwrap();

                                                if current - 1 >= 0 {
                                                    current -= 1;
                                                } else {
                                                    current = limit.clone();
                                                }
                                                *selected_article2.lock().unwrap() = current;
                                            }
                                        }
                                        KeyCode::Down => {
                                            if *selected_side2.lock().unwrap() == 0 {
                                                let limit = *launch_update_count2.lock().unwrap();
                                                let mut current = *selected_update2.lock().unwrap();

                                                if current + 1 < limit {
                                                    current += 1;
                                                } else {
                                                    current = 0;
                                                }
                                                *selected_update2.lock().unwrap() = current;
                                            } else {
                                                let limit = *news_article_count2.lock().unwrap();
                                                let mut current = *selected_article2.lock().unwrap();

                                                if current + 1 < limit {
                                                    current += 1;
                                                } else {
                                                    current = 0;
                                                }
                                                *selected_article2.lock().unwrap() = current;
                                            }
                                        }
                                        KeyCode::Left | KeyCode::Right => {
                                            let mut side = *selected_side2.lock().unwrap();
                                            if side == 0 {
                                                side = 1;
                                            } else {
                                                side = 0;
                                            }
                                            *selected_side2.lock().unwrap() = side;
                                        }
                                        KeyCode::F(no) => {
                                            match no {
                                                1 => {
                                                    if !*render_help2.lock().unwrap() {
                                                        *should_clear2.lock().unwrap() = true;
                                                        *render_help2.lock().unwrap() = true;
                                                    }
                                                }
                                                _ => {}
                                            }
                                        }
                                        KeyCode::Char(c) => {
                                            match c {
                                                '1' => {
                                                    *view_screen2.lock().unwrap() = 0;
                                                    *should_clear2.lock().unwrap() = true;
                                                }
                                                '2' => {
                                                    *view_screen2.lock().unwrap() = 1;
                                                    *should_clear2.lock().unwrap() = true;
                                                }
                                                '?' => {
                                                    if !*render_help2.lock().unwrap() {
                                                        *should_clear2.lock().unwrap() = true;
                                                        *render_help2.lock().unwrap() = true;
                                                    }
                                                }
                                                'c' => {
                                                    if raw_key.modifiers.contains(KeyModifiers::CONTROL) {
                                                        let mut stdout = std::io::stdout();
                                                        stdout.execute(Clear(ClearType::All));
                                                        println!("Thank you for using NextLaunch, goodbye.");
                                                        crossterm::terminal::disable_raw_mode();
                                                        exit(0);
                                                    }
                                                }
                                                'q' => {
                                                    if !raw_key.modifiers.contains(KeyModifiers::CONTROL) {
                                                        let mut stdout = std::io::stdout();
                                                        stdout.execute(Clear(ClearType::All));
                                                        println!("Thank you for using NextLaunch, goodbye.");
                                                        crossterm::terminal::disable_raw_mode();
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