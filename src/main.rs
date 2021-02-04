use terminal_size::{Width, Height, terminal_size};

pub mod management;

pub const VERSION: &str = "1.0.0 (Ashes Arisen)";
pub const NAME: &str = "Nextlaunch";
pub const AUTHOR: &str = "Thomas B. <tom.b.2k2@gmail.com>";
pub const DESCRIPTION: &str = "Watch a countdown until the next rocket launch, live in your terminal!";


#[tokio::main]
async fn main() {
    let flags = management::flags::check_flags().await;

    management::flags::process_flags(&flags).await;

    management::runtime::run().await;
}
