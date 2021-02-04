use crate::management::flags::Flags;
use crate::management::data::launches::{LaunchAPI, LaunchAPIop};
use std::time::{Instant, Duration};
use tokio::time::sleep;
use crate::utilities::process_seconds;

pub async fn run(f: Flags) {
    let (
        (mut s_launch, mut r_launch),
        (mut s_telem, mut r_telem),
        (mut s_render, mut r_render)
    ) = super::threads::spawn_threads(f.clone()).await;

    let mut last_launch_check = Instant::now();

    s_launch.send(LaunchAPI::new(LaunchAPIop::READ));
    let launch_options = r_launch.recv().await;

    loop {
        let incoming = r_launch.try_recv();

        if let Ok(launch_payload) = incoming {
            match launch_payload.op {
                LaunchAPIop::FETCH => {}
                LaunchAPIop::UPDATE => {}
                LaunchAPIop::CACHE => {}
                LaunchAPIop::READ => {}
                LaunchAPIop::RESPONSE => {}
                LaunchAPIop::ERROR => {}
                LaunchAPIop::HALT => {
                    sleep(Duration::from_secs(5 * 60)).await;
                }
            }
        } else {
            if last_launch_check.elapsed().as_secs() >= 15 * 60 {} else {
                // println!("{}", process_seconds(last_launch_check.elapsed().as_secs()))
            }
        }
        sleep(Duration::from_millis(500)).await;
    }
}