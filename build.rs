use dirs_2;
use reqwest;
use std::process::exit;
use std::fs;
use std::io::Write;

const LANG_URL: &str = "https://raw.githubusercontent.com/AltriusRS/NextLaunch/next/languages/languages.zip";
const CONFIG_URL: &str = "https://raw.githubusercontent.com/AltriusRS/NextLaunch/next/nextlaunch.json";
const README_URL: &str = "https://raw.githubusercontent.com/AltriusRS/NextLaunch/next/documents/readme.zip";

fn main() {
    let raw_data_dir = dirs_2::data_dir();
    if let Some(data_path) = raw_data_dir {
        let dp = format!("{}/nextlaunch/", data_path.to_str().unwrap());
        let _ = fs::create_dir(&dp);
        println!("Downloading language files");
        let pack_response = reqwest::blocking::get(LANG_URL);
        println!("Downloading config files");
        let config_response = reqwest::blocking::get(CONFIG_URL);
        println!("Downloading components");
        let readme_response = reqwest::blocking::get(README_URL);

        if let Ok(raw_pack) = pack_response {
            println!("unpacking language files");
            let bytes = raw_pack.bytes();
            if let Ok(raw_bytes) = bytes {
                let opts = fs::OpenOptions::new().read(true).write(true).create(true).open(format!("{}/lang_pack.tmpfile", dp));
                if let Ok(mut file) = opts {
                    let bytes: Vec<u8> = raw_bytes.to_vec();
                    let _ = file.write_all(bytes.as_slice());
                    let _ = file.flush();

                    let mut archive = zip::ZipArchive::new(file).unwrap();

                    if let Err(e) = archive.extract(format!("{}/languages", dp)) {
                        println!("cargo:warning=NextLaunch failed to unpack language files");
                        println!("cargo:warning={:#?}", e);
                        exit(1);
                    }
                } else {
                    println!("cargo:warning=NextLaunch failed to unpack language files");
                    println!("cargo:warning={:#?}", opts.unwrap_err());
                    exit(1);
                }
            } else {
                println!("cargo:warning=NextLaunch failed to unpack language files");
                println!("cargo:warning={:#?}", bytes.unwrap_err());
                exit(1);
            }
        } else {
            println!("cargo:warning=NextLaunch failed to download language files");
            println!("cargo:warning={:#?}", pack_response.unwrap_err());
            exit(1);
        }

        if let Ok(raw_pack) = readme_response {
            println!("unpacking language files");
            let bytes = raw_pack.bytes();
            if let Ok(raw_bytes) = bytes {
                let opts = fs::OpenOptions::new().read(true).write(true).create(true).open(format!("{}/readme_pack.tmpfile", dp));
                if let Ok(mut file) = opts {
                    let bytes: Vec<u8> = raw_bytes.to_vec();
                    let _ = file.write_all(bytes.as_slice());
                    let _ = file.flush();

                    let mut archive = zip::ZipArchive::new(file).unwrap();

                    if let Err(e) = archive.extract(format!("{}/languages", dp)) {
                        println!("cargo:warning=NextLaunch failed to unpack language files");
                        println!("cargo:warning={:#?}", e);
                        exit(1);
                    }
                } else {
                    println!("cargo:warning=NextLaunch failed to unpack language files");
                    println!("cargo:warning={:#?}", opts.unwrap_err());
                    exit(1);
                }
            } else {
                println!("cargo:warning=NextLaunch failed to unpack language files");
                println!("cargo:warning={:#?}", bytes.unwrap_err());
                exit(1);
            }
        } else {
            println!("cargo:warning=NextLaunch failed to download language files");
            println!("cargo:warning={:#?}", pack_response.unwrap_err());
            exit(1);
        }

        if let Ok(raw_config) = config_response {
            println!("unpacking config files");
            let bytes = raw_config.bytes();
            if let Ok(raw_bytes) = bytes {
                let opts = fs::OpenOptions::new().write(true).create(true).open(format!("{}/config.json", dp));
                if let Ok(mut file) = opts {
                    let bytes: Vec<u8> = raw_bytes.to_vec();
                    let _ = file.write_all(bytes.as_slice());
                    let _ = file.flush();
                } else {
                    println!("cargo:warning=NextLaunch failed to unpack config files");
                    println!("cargo:warning={:#?}", opts.unwrap_err());
                    exit(1);
                }
            } else {
                println!("cargo:warning=NextLaunch failed to unpack config files");
                println!("cargo:warning={:#?}", bytes.unwrap_err());
                exit(1);
            }
        } else {
            println!("cargo:warning=NextLaunch failed to download config files");
            println!("cargo:warning={:#?}", config_response.unwrap_err());
            exit(1);
        }

        let _ = fs::remove_file(format!("{}/lang_pack.tmpfile", dp));
        let _ = fs::remove_file(format!("{}/readme_pack.tmpfile", dp));

    } else {
        println!("cargo:warning=NextLaunch failed to determine location to store required files");
        exit(1);
    }

}