pub use structures::{LaunchAPI, LaunchAPIop, LaunchCache};


use reqwest::Client;
use tokio::fs::File;

mod implementors;
mod structures;

pub async fn update(c: &Client) {}

pub async fn process_incoming(p: LaunchAPI, cache: &mut File) {
    dbg!(p);
}