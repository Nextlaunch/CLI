pub use constants::*;

pub mod utilities;
pub mod constants;
pub mod runtime;
pub mod languages;

#[tokio::main]
async fn main() {

    let flags = runtime::flags::check_flags().await;

    runtime::flags::process_flags(&flags).await;

    runtime::launch(flags).await;
}
