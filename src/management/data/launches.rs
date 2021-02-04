pub use structures::{LaunchAPI, LaunchAPIop, LaunchCache};


use reqwest::Client;
use tokio::fs::File;

mod implementors;
mod structures;

pub async fn update(c: &Client) {}

pub async fn process_incoming(p: LaunchAPI, cache_file: &mut File, cache: &mut LaunchCache) {
    match p.op {
        LaunchAPIop::FETCH => {
            dbg!(cache);
        }
        LaunchAPIop::UPDATE => {
            dbg!(cache);
        }
        LaunchAPIop::CACHE => {
            dbg!(cache);
        }
        LaunchAPIop::READ => {
            dbg!(cache);
        }
        _ => {}
    }
}