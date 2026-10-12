#!/bin/bash
TARGET_DIR="$HOME/Documents/GitHub/physics/blog/content/assets"
mkdir -p "$TARGET_DIR"

FILE_NAME="${1:-physics_capture}"
REQ_W="${2:-1600}"

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

[ "$REQ_W" -gt "$W" ] && REQ_W="$W"
REQ_H=$(( REQ_W * 10 / 16 ))
if [ "$REQ_H" -gt "$H" ]; then
    REQ_H="$H"
    REQ_W=$(( REQ_H * 16 / 10 ))
fi
[ "$REQ_W" -lt 16 ] && REQ_W=16
[ "$REQ_H" -lt 10 ] && REQ_H=10
FINAL_X=$(( X_OFF + (W - REQ_W) / 2 ))
FINAL_Y=$(( Y_OFF + (H - REQ_H) / 2 ))
grim -g "${FINAL_X},${FINAL_Y} ${REQ_W}x${REQ_H}" - |
    magick - -quality 90 "${TARGET_DIR}/${FILE_NAME}.webp"

echo "Saved ${REQ_W}x${REQ_H} (16:10) centered shot to: ${TARGET_DIR}/${FILE_NAME}.webp"
