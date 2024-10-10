echo "Building for Mac"
cargo build --release
echo "Building for Windows"
cargo build --release --target x86_64-pc-windows-gnu
