#!/bin/bash

OWNER="byson94"
REPO="plug-in-c.ewwii"
FILE_PATTERN="plugc-ewwii" 
OUT="$HOME/.local/bin/plugc-ewwii"

RELEASE_JSON=$(curl -s "https://api.github.com/repos/$OWNER/$REPO/releases/latest")
DOWNLOAD_URL=$(echo "$RELEASE_JSON" | grep -o "https://[^\"]*$FILE_PATTERN[^\"]*")

if [ -z "$DOWNLOAD_URL" ]; then
    echo "Error: Could not find asset matching '$FILE_PATTERN'"
    exit 1
fi

TEMP_FILE=$(mktemp)
echo "Downloading: $DOWNLOAD_URL"
curl -L -o -o "$TEMP_FILE" "$DOWNLOAD_URL"

chmod +x "$TEMP_FILE"
mv "$TEMP_FILE" "$OUT"

echo "Installation complete at: $OUT"
