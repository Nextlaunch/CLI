pub use constants::*;
use std::process::exit;


pub mod management;
pub mod constants;
pub mod utilities;


#[tokio::main]
async fn main() {
    println!("\x1b[?8452h\n\x1b[Pq\n#0;2;99;99;99#1;2;0;99;0\n#0?o!${FULL}Oo\$#1??!${PART}_-\n\n\n#1!14@\x1b\\");

    exit(0);

    let flags = management::flags::check_flags().await;

    management::flags::process_flags(&flags).await;

    management::runtime::run(flags).await;
}
