#!/bin/bash
# A personal script to take exact screenshots.
TARGET_DIR="$HOME/Documents/GitHub/math/blog/content/assets"
mkdir -p "$TARGET_DIR"

FILE_NAME="${1:-physics_capture}"
SIZE="${2:-100}"
OFFSET=$((SIZE / 2))
echo -n "Capturing in 3..."
sleep 1
echo -n "2..."
sleep 1
echo "1!"
sleep 1

if command -v hyprctl >/dev/null; then
    MONITOR_INFO=$(hyprctl monitors -j | jq -r '.[] | select(.focused == true)')
    W=$(echo "$MONITOR_INFO" | jq -r '.width')
    H=$(echo "$MONITOR_INFO" | jq -r '.height')
    X_OFF=$(echo "$MONITOR_INFO" | jq -r '.x')
    Y_OFF=$(echo "$MONITOR_INFO" | jq -r '.y')
fi

X=$((X_OFF + (W / 2) - OFFSET))
Y=$((Y_OFF + (H / 2) - OFFSET))

grim -g "${X},${Y} ${SIZE}x${SIZE}" - |
    magick - -quality 90 "${TARGET_DIR}/${FILE_NAME}.webp"

echo "Saved ${SIZE}x${SIZE} centered shot to: ${TARGET_DIR}/${FILE_NAME}.webp"
