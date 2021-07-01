pub use constants::*;
use std::process::exit;

pub mod utilities;
pub mod settings;
pub mod constants;
pub mod runtime;
pub mod languages;

#[tokio::main]
async fn main() {
    let cfg = settings::import();
    exit(1);
    let flags = runtime::flags::check_flags().await;

    runtime::flags::process_flags(&flags).await;

    runtime::launch(flags, cfg).await;
}
