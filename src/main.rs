use terminal_size::{Width, Height, terminal_size};

pub use constants::*;


pub mod management;
pub mod constants;
pub mod utilities;


#[tokio::main]
async fn main() {
    println!(r#"
{}BLACK
{}RED
{}GREEN
{}YELLOW
{}BLUE
{}MAGENTA
{}CYAN
{}WHITE"#, FG_BLACK, FG_RED, FG_GREEN, FG_YELLOW, FG_BLUE, FG_MAGENTA, FG_CYAN, FG_WHITE);

    println!(r#"
{}BLACK{}
{}RED{}
{}GREEN{}
{}YELLOW{}
{}BLUE{}
{}MAGENTA{}
{}CYAN{}
{}WHITE{}"#, BG_BLACK, RESET, BG_RED, RESET, BG_GREEN, RESET, BG_YELLOW, RESET, BG_BLUE, RESET, BG_MAGENTA, RESET, BG_CYAN, RESET, BG_WHITE, RESET);

    let flags = management::flags::check_flags().await;

    management::flags::process_flags(&flags).await;

    management::runtime::run(flags).await;
}
