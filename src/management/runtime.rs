use crate::management::flags::Flags;
use crate::management::data::launches::{LaunchAPI, LaunchAPIop};

use std::time::{Instant, Duration};

use tokio::time::sleep;
use crate::management::data::RenderFrame;
use crate::utilities::process_seconds;


pub async fn run(f: Flags) {
    let (
        (s_launch, mut r_launch),
        (_s_telem, mut _r_telem),
        (s_render, mut _r_render)
    ) = super::threads::spawn_threads(f.clone()).await;

    let mut last_launch_check = Instant::now();

    s_launch.send(LaunchAPI::new(LaunchAPIop::READ));
    let mut launch_options = r_launch.recv().await.unwrap_or(LaunchAPI::new(LaunchAPIop::ERROR("Timed out waiting for response from the launch monitor".to_string())));
    s_render.send(RenderFrame {
        view: 0,
        launch_refresh: last_launch_check.clone(),
        launch: None,
        telemetry: None,
        error: None,
    });

    let mut idx: u8 = 0;

    loop {
        let incoming = r_launch.try_recv();

        if let Ok(launch_payload) = incoming {
            match launch_payload.op {
                LaunchAPIop::FETCH => {}
                LaunchAPIop::UPDATE => {}
                LaunchAPIop::CACHE => {}
                LaunchAPIop::READ => {}
                LaunchAPIop::RESPONSE => {}
                LaunchAPIop::ERROR(reason) => {
                    s_render.send(RenderFrame {
                        view: 0,
                        launch_refresh: last_launch_check.clone(),
                        launch: None,
                        telemetry: None,
                        error: Some(reason),
                    });
                }
                LaunchAPIop::HALT => {
                    sleep(Duration::from_secs(5 * 60)).await;
                }
            }
        } else {
            if idx == 1 {
                s_render.send(RenderFrame {
                    view: 0,
                    launch_refresh: last_launch_check.clone(),
                    launch: None,
                    telemetry: None,
                    error: None,
                });
                idx = 0;
            } else {
                idx += 1;
            }
            // if last_launch_check.elapsed().as_secs() >= 15 * 60 {} else {
            //     println!("\x1b[1A{}", process_seconds(last_launch_check.elapsed().as_secs()));
            // }
        }
        sleep(Duration::from_millis(500)).await;
    }
}
