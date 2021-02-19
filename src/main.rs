pub use constants::*;
use std::process::exit;


pub mod management;
pub mod constants;
pub mod utilities;

pub mod sixel;


#[tokio::main]
async fn main() {
    println!("{}\n\n\x1b[1;37mWelcome to NextLaunch.\x1b[0m\nA state of the art program, providing advanced access to the world's rocket launches and space news, directly in your terminal", sixel::CONTENT);

    exit(0);

    let flags = management::flags::check_flags().await;

    management::flags::process_flags(&flags).await;

    management::runtime::run(flags).await;
}
