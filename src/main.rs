pub use constants::*;
use std::process::exit;


pub mod management;
pub mod constants;
pub mod utilities;

pub mod sixel;


#[tokio::main]
async fn main() {
    print!("{}", sixel::CONTENT);

    exit(0);

    let flags = management::flags::check_flags().await;

    management::flags::process_flags(&flags).await;

    management::runtime::run(flags).await;
}
