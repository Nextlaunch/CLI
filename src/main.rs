pub use constants::*;
use std::process::exit;


pub mod management;
pub mod constants;
pub mod utilities;

pub mod sixel;


// TODO: Look further into the Kitty and Sixel graphics rendering protocols
// Kitty: https://sw.kovidgoyal.net/kitty/graphics-protocol.html?highlight=protocol

#[tokio::main]
async fn main() {

    let flags = management::flags::check_flags().await;

    management::flags::process_flags(&flags).await;

    if flags.sixel {
        println!("{}", sixel::CONTENT)
    } else {
        println!("{}", sixel::TEXT)
    }
    println!("\x1b[1;37mWelcome to NextLaunch.\x1b[0m\nA state of the art program, providing advanced access to the world's rocket launches and space news, directly in your terminal");

    exit(0);

    management::runtime::run(flags).await;
}
