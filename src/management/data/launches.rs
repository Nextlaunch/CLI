pub use structures::{LaunchAPI, LaunchAPIop};


use reqwest::Client;

mod implementors;
mod structures;

pub async fn update(c: &Client) {

}

pub async fn process_incoming(p: LaunchAPI) {
    dbg!(p);
}