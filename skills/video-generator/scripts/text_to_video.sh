#!/bin/bash
# Generate video from text using ffmpeg

TEXT="$1"
OUTPUT="$2"
DURATION="${3:-5}"

if [ -z "$TEXT" ] || [ -z "$OUTPUT" ]; then
    echo "Usage: $0 <text> <output.mp4> [duration]"
    exit 1
fi

TEMP_DIR=$(mktemp -d)
FRAME_DIR="$TEMP_DIR/frames"
mkdir -p "$FRAME_DIR"

# Create a single frame with text
convert -size 1920x1080 xc:black \
    -fill white -pointsize 48 -gravity center \
    -annotate 0 "$TEXT" \
    "$FRAME_DIR/frame.png"

# Generate video from frame
ffmpeg -y -loop 1 -i "$FRAME_DIR/frame.png" \
    -c:v libx264 -t "$DURATION" -pix_fmt yuv420p \
    "$OUTPUT"

rm -rf "$TEMP_DIR"
echo "Generated: $OUTPUT (${DURATION}s)"