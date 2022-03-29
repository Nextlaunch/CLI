use dirs_2;
use reqwest;
use zip;
use std::process::exit;
use std::fs;
use std::io::Write;
// use crate::{languages::select_language, settings::import};

mod structures;

const LANG_URL: &str = "https://raw.githubusercontent.com/nextlaunch/cli/main/installer_files/languages.zip";
const CONFIG_URL: &str = "https://raw.githubusercontent.com/nextlaunch/cli/main/installer_files/nextlaunch.json";
const README_URL: &str = "https://raw.githubusercontent.com/nextlaunch/cli/main/installer_files/documents.zip";

pub async fn install() {
    let raw_data_dir = dirs_2::data_dir();
    if let Some(data_path) = raw_data_dir {
        let dp = format!("{}/nextlaunch/", data_path.to_str().unwrap());
        let _ = fs::create_dir(&dp);
        println!("Downloading language files...");
        let pack_response = reqwest::get(LANG_URL).await;
        println!("Downloading config files...");
        let config_response = reqwest::get(CONFIG_URL).await;
        println!("Downloading components...");
        let readme_response = reqwest::get(README_URL).await;

        if let Ok(raw_pack) = pack_response {
            println!("unpacking language files...");
            let bytes = raw_pack.bytes().await;
            if let Ok(raw_bytes) = bytes {
                let opts = fs::OpenOptions::new().read(true).write(true).create(true).open(format!("{}/lang_pack.tmpfile", dp));
                if let Ok(mut file) = opts {
                    let bytes: Vec<u8> = raw_bytes.to_vec();
                    let _ = file.write_all(bytes.as_slice());
                    let _ = file.flush();

                    let mut archive = zip::ZipArchive::new(file).unwrap();

                    if let Err(e) = archive.extract(format!("{}/languages", dp)) {
                        println!("NextLaunch failed to unpack language files.");
                        println!("{:#?}", e);
                        exit(1);
                    }
                } else {
                    println!("NextLaunch failed to unpack language files.");
                    println!("{:#?}", opts.unwrap_err());
                    exit(1);
                }
            } else {
                println!("NextLaunch failed to unpack language files.");
                println!("{:#?}", bytes.unwrap_err());
                exit(1);
            }
        } else {
            println!("NextLaunch failed to download language files.");
            println!("{:#?}", pack_response.unwrap_err());
            exit(1);
        }

        if let Ok(raw_pack) = readme_response {
            println!("unpacking language files...");
            let bytes = raw_pack.bytes().await;
            if let Ok(raw_bytes) = bytes {
                let opts = fs::OpenOptions::new().read(true).write(true).create(true).open(format!("{}/readme_pack.tmpfile", dp));
                if let Ok(mut file) = opts {
                    let bytes: Vec<u8> = raw_bytes.to_vec();
                    let _ = file.write_all(bytes.as_slice());
                    let _ = file.flush();

                    let mut archive = zip::ZipArchive::new(file).unwrap();

                    if let Err(e) = archive.extract(format!("{}/readme", dp)) {
                        println!("NextLaunch failed to unpack language files.");
                        println!("{:#?}", e);
                        exit(1);
                    }
                } else {
                    println!("NextLaunch failed to unpack language files.");
                    println!("{:#?}", opts.unwrap_err());
                    exit(1);
                }
            } else {
                println!("NextLaunch failed to unpack language files.");
                println!("{:#?}", bytes.unwrap_err());
                exit(1);
            }
        } else {
            println!("NextLaunch failed to download language files.");
            println!("{:#?}", readme_response.unwrap_err());
            exit(1);
        }

        if let Ok(raw_config) = config_response {
            println!("unpacking config files.");
            let bytes = raw_config.bytes().await;
            if let Ok(raw_bytes) = bytes {
                let opts = fs::OpenOptions::new().write(true).create(true).open(format!("{}/config.json", dp));
                if let Ok(mut file) = opts {
                    let bytes: Vec<u8> = raw_bytes.to_vec();
                    let _ = file.write_all(bytes.as_slice());
                    let _ = file.flush();
                } else {
                    println!("NextLaunch failed to unpack config files.");
                    println!("{:#?}", opts.unwrap_err());
                    exit(1);
                }
            } else {
                println!("NextLaunch failed to unpack config files.");
                println!("{:#?}", bytes.unwrap_err());
                exit(1);
            }
        } else {
            println!("NextLaunch failed to download config files.");
            println!("{:#?}", config_response.unwrap_err());
            exit(1);
        }

        let _ = fs::remove_file(format!("{}/lang_pack.tmpfile", dp));
        let _ = fs::remove_file(format!("{}/readme_pack.tmpfile", dp));

    } else {
        println!("NextLaunch failed to determine location to store required files.");
        exit(1);
    }

}


pub fn check_integrity() -> bool {

    false
}
