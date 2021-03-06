pub use constants::*;




pub mod utilities;
pub mod constants;
pub mod runtime;

// TODO: Look further into the Kitty and Sixel graphics rendering protocols
// Kitty: https://sw.kovidgoyal.net/kitty/graphics-protocol.html?highlight=protocol

#[tokio::main]
async fn main() {

    let flags = runtime::flags::check_flags().await;

    runtime::flags::process_flags(&flags).await;

    // println!("{}", utilities::digit_map::map_str("00:00:00:00:00:00").join("\n"));
    // println!("{}", utilities::digit_map::map_str("00:00:00:01:00:00").join("\n"));
    // println!("{}", utilities::digit_map::map_str("00:00:00:02:00:00").join("\n"));
    // println!("{}", utilities::digit_map::map_str("00:00:00:03:00:00").join("\n"));
    // println!("{}", utilities::digit_map::map_str("00:00:00:04:00:00").join("\n"));
    // println!("{}", utilities::digit_map::map_str("00:00:00:05:00:00").join("\n"));
    // println!("{}", utilities::digit_map::map_str("00:00:00:06:00:00").join("\n"));
    // println!("{}", utilities::digit_map::map_str("00:00:00:07:00:00").join("\n"));
    // println!("{}", utilities::digit_map::map_str("00:00:00:08:00:00").join("\n"));
    // println!("{}", utilities::digit_map::map_str("00:00:00:09:00:00").join("\n"));

    runtime::launch(flags).await;
}
