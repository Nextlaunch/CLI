use std::fs::metadata;
use std::io::ErrorKind;

pub use constants::*;

pub mod constants;
pub mod installer;
pub mod languages;
pub mod runtime;
pub mod settings;
pub mod utilities;

#[tokio::main]
async fn main() {
    let raw_data_dir = dirs_2::data_dir();
    if let Some(data_path) = raw_data_dir {
        let dp = format!("{}/nextlaunch/", data_path.to_str().unwrap());
        let x = metadata(&dp);
        match x {
            // Data path exists and program can continue without re-downloading files
            Ok(_) => {
                if !installer::check_integrity() {
                    println!(
                        "Nextlaunch has detected an integrity problem with configuration files."
                    );
                    println!("Attempting to repair files...");
                    installer::install().await;
                    println!("Great! Now that we are all set up, you are ready to use Nextlaunch!");
                }

                launch().await;
            }
            Err(e) => {
                match e.kind() {
                    ErrorKind::PermissionDenied => {
                        println!("Nextlaunch cannot install it's supporting files due to mismatched permissions.");
                        crashlog(e, dp).await;
                    }
                    ErrorKind::BrokenPipe => {
                        println!(
                            "Nextlaunch cannot install it's supporting files due to a pipe error."
                        );
                        crashlog(e, dp).await;
                    }
                    ErrorKind::InvalidInput => {
                        println!("Nextlaunch cannot install it's supporting files due to an invalid input.");
                        crashlog(e, dp).await;
                    }
                    ErrorKind::InvalidData => {
                        println!("Nextlaunch cannot install it's supporting files due to a invalid data.");
                        crashlog(e, dp).await;
                    }
                    ErrorKind::TimedOut => {
                        println!("Nextlaunch cannot install it's supporting files because the operation timed out.");
                        crashlog(e, dp).await;
                    }
                    ErrorKind::Unsupported => {
                        println!("Nextlaunch cannot install it's supporting files due to an unsupported operation.");
                        crashlog(e, dp).await;
                    }
                    ErrorKind::OutOfMemory => {
                        println!("Nextlaunch cannot install it's supporting files because it was unable to allocate enough memory.");
                        crashlog(e, dp).await;
                    }
                    ErrorKind::NotFound => {
                        println!("Welcome to Nextlaunch.");
                        println!("Commencing first-run installation, please wait whilst we set things up...");
                        installer::install().await;
                        println!(
                            "Great! Now that we are all set up, you are ready to use Nextlaunch!"
                        );

                        launch().await;
                    }
                    _ => {
                        println!("Nextlaunch cannot install it's supporting files due to an unknown error.");
                        crashlog(e, dp).await;
                    }
                }
            }
        }
    }
}

async fn launch() {
    let cfg = settings::import();

    let flags = runtime::flags::check_flags().await;

    runtime::flags::process_flags(&flags).await;

    runtime::launch(flags, cfg).await;
}

async fn crashlog(e: std::io::Error, dp: String) {
    println!("Please send the following as a crash report");
    println!("======== REPORT START ========");
    println!("Error type: {:?}", e.kind());
    println!("Data path: {}", dp);
    dbg!(e);
    println!("========= REPORT END =========");
}
