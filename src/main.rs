use terminal_size::{Width, Height, terminal_size};

pub use constants::*;


pub mod management;
pub mod constants;
pub mod utilities;


#[tokio::main]
async fn main() {
    let flags = management::flags::check_flags().await;

    management::flags::process_flags(&flags).await;

    management::runtime::run(flags).await;
}
