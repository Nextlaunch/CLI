pub use structures::Launch;
use reqwest::Client;

mod structures;

const DATA_SOURCE: &str = "https://lldev.thespacedevs.com/2.1.0/launch/upcoming/?format=json&mode=detailed&limit=5"

pub async fn update(c: &Client) {

}