

echo "Building NextLaunch for Linux"
{
  cargo build --release
} &>target/release/build.log
echo "Building NextLaunch for Windows"
{
  cargo build --release --target x86_64-pc-windows-gnu
} &>target/x86_64-pc-windows-gnu/release/build.log
echo "Building NextLaunch for Mac OS"
{
  cargo build --release --target x86_64-apple-darwin
} &>target/x86_64-apple-darwin/release/build.log
