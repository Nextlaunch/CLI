use std::fs::{read, write};
use b2sum_rs::Blake2bSum;


fn main() {
    let docs = calc_hash("./installer_files/documents.zip");
    let lang = calc_hash("./installer_files/languages.zip");
    let conf = calc_hash("./installer_files/nextlaunch.json");
    println!("Hash for documents.zip:   '{}'", docs);
    println!("Hash for languages.zip:   '{}'", lang);
    println!("Hash for nextlaunch.json: '{}'", conf);
    let contents = format!("{{\"hashes\":{{\"languages\":\"{}\",\"config\":\"{}\",\"documents\":\"{}\"}}}}", lang, conf, docs);
    let _ = write("./installer_files/manifest.json", contents);
}


fn calc_hash(path: &str)-> String {
    let file = read(path);
    if let Ok(_) = file {
        let hasher = Blake2bSum::new(64);
        let hash = hasher.read(path);
        hash
    } else {
        println!("Unable to generate hash for {}", path);
        println!("{:?}", file.unwrap_err().kind());
        String::new()
    }
}
