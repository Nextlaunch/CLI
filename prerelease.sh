# Check zip and unzip are installed
sudo apt install zip unzip

# Compress language packs into ZIP file
rm installer_files/languages.zip
zip -r installer_files/languages.zip languages

# Compress documents folder into ZIP file
rm installer_files/documents.zip
zip -r installer_files/documents.zip documents

# Change working directory to the hash calculator sub-program
cd installer_files/calc_hash

# Run hash calculator to generate hashes and output them to the manifest file
cargo build --release
cd ../../

./installer_files/calc_hash/target/release/calc_hash
