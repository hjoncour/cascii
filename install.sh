#!/bin/bash
set -e

# App specific directory in Application Support
APP_SUPPORT_DIR="$HOME/Library/Application Support/casci"
REPO_DIR="$APP_SUPPORT_DIR/repo"

echo "Setting up application directory at $APP_SUPPORT_DIR..."
mkdir -p "$REPO_DIR"

echo "Copying repository to $REPO_DIR..."
rsync -a --delete --exclude='.git' --exclude='target' ./ "$REPO_DIR/"

echo "Building release binary in $REPO_DIR..."
(cd "$REPO_DIR" && cargo build --release)

INSTALL_DIR="/usr/local/bin"

# Install casci binary
BINARY_NAME="casci"
SOURCE_PATH="$REPO_DIR/target/release/$BINARY_NAME"
echo "Installing $BINARY_NAME to $INSTALL_DIR..."
sudo cp "$SOURCE_PATH" "$INSTALL_DIR/$BINARY_NAME"

# Determine shell configuration file
SHELL_CONFIG=""
if [[ "$SHELL" == */zsh ]]; then
    SHELL_CONFIG="$HOME/.zshrc"
elif [[ "$SHELL" == */bash ]]; then
    SHELL_CONFIG="$HOME/.bash_profile"
else
    echo "Unsupported shell: $SHELL. Please add the casci-demo function to your shell configuration file manually."
    exit 1
fi

# Ensure the shell configuration file exists
touch "$SHELL_CONFIG"

echo "Adding 'casci-demo' function to $SHELL_CONFIG..."

# Remove existing casci-demo function to ensure idempotency
sed -i '' '/# casci-demo function start/,/# casci-demo function end/d' "$SHELL_CONFIG"

# Add new casci-demo function
cat <<'EOF' >> "$SHELL_CONFIG"
# casci-demo function start
casci-demo() {
    local REPO_DIR="$HOME/Library/Application Support/casci/repo"
    local PUBLIC_DIR="$REPO_DIR/public"

    if [ "$1" = "go" ]; then
        cd "$PUBLIC_DIR"
        return
    fi

    if [ "$1" = "open" ]; then
        open "$PUBLIC_DIR"
        return
    fi

    if [ -z "$1" ]; then
        echo "Usage: casci-demo <path_to_frames_folder> | go | open"
        return
    fi
    
    local SOURCE_FOLDER="$1"
    if [ ! -d "$SOURCE_FOLDER" ]; then
        echo "Error: '$SOURCE_FOLDER' is not a directory."
        return
    fi

    local PROJECT_NAME=$(basename "$SOURCE_FOLDER")
    local PROJECTS_JSON="$PUBLIC_DIR/projects.json"
    local DEST_DIR="$PUBLIC_DIR/$PROJECT_NAME"

    echo "Copying '$PROJECT_NAME' to $DEST_DIR..."
    rsync -a "$SOURCE_FOLDER/" "$DEST_DIR/"

    echo "Updating projects manifest..."
    local FRAME_COUNT=$(find "$SOURCE_FOLDER" -name 'frame_*.txt' | wc -l | tr -d ' ')

    if [ ! -f "$PROJECTS_JSON" ]; then
        echo "[]" > "$PROJECTS_JSON"
    fi

    local TMP_JSON=$(mktemp)
    jq --arg name "$PROJECT_NAME" --argjson count "$FRAME_COUNT" \
       '(. | map(if .name == $name then .frameCount = $count else . end)) | if any(.name == $name) then . else . + [{"name": $name, "frameCount": $count, "fps": 24}] end' \
       "$PROJECTS_JSON" > "$TMP_JSON" && mv "$TMP_JSON" "$PROJECTS_JSON"

    echo "Starting viewer..."
    (cd "$REPO_DIR" && npm install && npm start)
}
# casci-demo function end
EOF

echo "Installation complete."
echo "You can now use 'casci' and 'casci-demo' commands."
echo "Please run 'source $SHELL_CONFIG' or open a new terminal to use 'casci-demo'."
