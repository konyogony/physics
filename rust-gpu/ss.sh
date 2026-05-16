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
else
    echo "Error: hyprctl not found."
    exit 1
fi

REQ_X=$((X_OFF + (W / 2) - OFFSET))
REQ_Y=$((Y_OFF + (H / 2) - OFFSET))
REQ_W=$SIZE
REQ_H=$SIZE

MON_RIGHT=$((X_OFF + W))
MON_BOTTOM=$((Y_OFF + H))

FINAL_X=$((REQ_X < X_OFF ? X_OFF : REQ_X))
FINAL_Y=$((REQ_Y < Y_OFF ? Y_OFF : REQ_Y))

END_X=$(((REQ_X + REQ_W) > MON_RIGHT ? MON_RIGHT : (REQ_X + REQ_W)))
END_Y=$(((REQ_Y + REQ_H) > MON_BOTTOM ? MON_BOTTOM : (REQ_Y + RE_H)))

FINAL_W=$((END_X - FINAL_X))
FINAL_H=$((END_Y - FINAL_Y))

if [ "$FINAL_W" -le 0 ] || [ "$FINAL_H" -le 0 ]; then
    echo "Error: Requested size is outside the monitor bounds."
    exit 1
fi

grim -g "${FINAL_X},${FINAL_Y} ${FINAL_W}x${FINAL_H}" - |
    magick - -quality 90 "${TARGET_DIR}/${FILE_NAME}.webp"

echo "Saved ${FINAL_W}x${FINAL_H} (clipped) centered shot to: ${TARGET_DIR}/${FILE_NAME}.webp"

# Fully disclosure: AI Generated script. Do NOT have time for this.
