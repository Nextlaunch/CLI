echo "Installing necessary toolchains"

rustup target add x86_64-unknown-linux-gnu
rustup target add x86_64-pc-windows-gnu
rustup target add x86_64-apple-darwin

mkdir target/
mkdir target/x86_64-unknown-linux-gnu/
mkdir target/x86_64-pc-windows-gnu/
mkdir target/x86_64-apple-darwin/
mkdir target/x86_64-unknown-linux-gnu/release/
mkdir target/x86_64-pc-windows-gnu/release/
mkdir target/x86_64-apple-darwin/release/

echo "Building NextLaunch for Linux"
cargo build --release --target x86_64-unknown-linux-gnu || tee target/x86_64-unknown-linux-gnu/release/build.log

pause

echo "Building NextLaunch for Windows"
cargo build --release --target x86_64-pc-windows-gnu || tee target/x86_64-pc-windows-gnu/release/build.log

pause

echo "Building NextLaunch for Mac OS"
cargo build --release --target x86_64-apple-darwin || tee target/x86_64-apple-darwin/release/build.log

pause

clear

echo "Targets compiled:"
echo "x86_64-unknown-linux-gnu"
echo "x86_64-pc-windows-gnu"
echo "x86_64-apple-darwin"
echo ""
echo "Releasing to Crates.io"
cargo publish

clear

echo "Targets compiled:"
echo "x86_64-unknown-linux-gnu"
echo "x86_64-pc-windows-gnu"
echo "x86_64-apple-darwin"
echo "Released to Crates.io"

echo "Build script complete"