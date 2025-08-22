#!/bin/bash
set -e

# App specific directory in Application Support
APP_SUPPORT_DIR="$HOME/Library/Application Support/casci"
REPO_DIR="$APP_SUPPORT_DIR/repo"

echo "Setting up application directory at $APP_SUPPORT_DIR..."
mkdir -p "$REPO_DIR"

echo "Copying repository to $REPO_DIR..."
# Use rsync to copy, excluding .git and target
rsync -a --delete --exclude='.git' --exclude='target' ./ "$REPO_DIR/"

echo "Building release binary in $REPO_DIR..."
(cd "$REPO_DIR" && cargo build --release)

INSTALL_DIR="/usr/local/bin"

# Install casci binary
BINARY_NAME="casci"
SOURCE_PATH="$REPO_DIR/target/release/$BINARY_NAME"
echo "Installing $BINARY_NAME to $INSTALL_DIR..."
sudo cp "$SOURCE_PATH" "$INSTALL_DIR/$BINARY_NAME"

# Install casci-demo script
DEMO_SCRIPT_NAME="casci-demo.sh"
DEMO_SOURCE_PATH="$REPO_DIR/$DEMO_SCRIPT_NAME"
echo "Installing $DEMO_SCRIPT_NAME to $INSTALL_DIR/casci-demo..."
sudo cp "$DEMO_SOURCE_PATH" "$INSTALL_DIR/casci-demo"
sudo chmod +x "$INSTALL_DIR/casci-demo"


echo "Installation complete."
echo "You can now use 'casci' and 'casci-demo' commands."
