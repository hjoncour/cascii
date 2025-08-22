#!/bin/bash
set -e

if [ -z "$1" ]; then
    echo "Usage: casci-demo <path_to_frames_folder>"
    exit 1
fi

SOURCE_FOLDER="$1"
if [ ! -d "$SOURCE_FOLDER" ]; then
    echo "Error: '$SOURCE_FOLDER' is not a directory."
    exit 1
fi

PROJECT_NAME=$(basename "$SOURCE_FOLDER")
REPO_DIR="$HOME/Library/Application Support/casci/repo"

if [ ! -d "$REPO_DIR" ]; then
    echo "Error: casci repo not found at $REPO_DIR"
    echo "Please run install.sh from the casci repository first."
    exit 1
fi

PUBLIC_DIR="$REPO_DIR/public"
PROJECTS_JSON="$PUBLIC_DIR/projects.json"
DEST_DIR="$PUBLIC_DIR/$PROJECT_NAME"

echo "Copying '$PROJECT_NAME' to $DEST_DIR..."
rsync -a "$SOURCE_FOLDER/" "$DEST_DIR/"

echo "Updating projects manifest..."
FRAME_COUNT=$(find "$SOURCE_FOLDER" -name 'frame_*.txt' | wc -l | tr -d ' ')

if ! command -v jq &> /dev/null; then
    echo "jq is not installed, which is required for casci-demo."
    echo "On macOS, you can install it with: brew install jq"
    exit 1
fi

if [ ! -f "$PROJECTS_JSON" ]; then
    echo "[]" > "$PROJECTS_JSON"
fi

TMP_JSON=$(mktemp)
jq --arg name "$PROJECT_NAME" --argjson count "$FRAME_COUNT" \
   '(. | map(if .name == $name then .frameCount = $count else . end)) | if any(.name == $name) then . else . + [{"name": $name, "frameCount": $count, "fps": 24}] end' \
   "$PROJECTS_JSON" > "$TMP_JSON" && mv "$TMP_JSON" "$PROJECTS_JSON"


echo "Starting viewer..."
(cd "$REPO_DIR" && npm install && npm start)
