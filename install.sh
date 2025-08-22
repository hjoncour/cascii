#!/usr/bin/env bash
#
# Builds and installs `casci` to /usr/local/bin
#

set -e

# Change to the script's directory
cd "$(dirname "$0")"

echo "Building release binary..."
cargo build --release

echo "Installing casci to /usr/local/bin..."
sudo cp ./target/release/casci /usr/local/bin/casci
sudo chmod +x /usr/local/bin/casci

echo "Installation complete!"
echo "You can now run 'casci' from anywhere."
