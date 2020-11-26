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

{
  cargo build --release --target x86_64-unknown-linux-gnu
} &> target/x86_64-unknown-linux-gnu/release/build.log

cat target/x86_64-unknown-linux-gnu/release/build.log

echo "Building NextLaunch for Windows"

{
  cargo build --release --target x86_64-pc-windows-gnu
} &>target/x86_64-pc-windows-gnu/release/build.log

cat target/x86_64-pc-windows-gnu/release/build.log

echo "Building NextLaunch for Mac OS"

{
  cargo build --release --target x86_64-apple-darwin
} &>target/x86_64-apple-darwin/release/build.log

cat target/x86_64-apple-darwin/release/build.log

echo "Build script complete"