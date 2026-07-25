#!/bin/bash
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
else
    echo "Error: hyprctl not found."
    exit 1
fi

CENTER_X=$((X_OFF + W / 2))
CENTER_Y=$((Y_OFF + H / 2))

START_X=$((CENTER_X - OFFSET))
START_Y=$((CENTER_Y - OFFSET))

FINAL_X=$((START_X < X_OFF ? X_OFF : START_X))
FINAL_Y=$((START_Y < Y_OFF ? Y_OFF : START_Y))

MAX_W=$(((X_OFF + W) - FINAL_X))
MAX_H=$(((Y_OFF + H) - FINAL_Y))

FINAL_W=$((SIZE > MAX_W ? MAX_W : SIZE))
FINAL_H=$((SIZE > MAX_H ? MAX_H : SIZE))

[ "$FINAL_W" -lt 1 ] && FINAL_W=1
[ "$FINAL_H" -lt 1 ] && FINAL_H=1

grim -g "${FINAL_X},${FINAL_Y} ${FINAL_W}x${FINAL_H}" - |
    magick - -quality 90 "${TARGET_DIR}/${FILE_NAME}.webp"

echo "Saved ${FINAL_W}x${FINAL_H} (clamped) centered shot to: ${TARGET_DIR}/${FILE_NAME}.webp"
