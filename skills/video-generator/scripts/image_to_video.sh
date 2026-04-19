#!/bin/bash
# Convert image to video using ffmpeg

INPUT="$1"
OUTPUT="$2"
DURATION="${3:-5}"

if [ -z "$INPUT" ] || [ -z "$OUTPUT" ]; then
    echo "Usage: $0 <input.jpg> <output.mp4> [duration]"
    exit 1
fi

if [ ! -f "$INPUT" ]; then
    echo "Error: Input file not found: $INPUT"
    exit 1
fi

# Generate video from image
ffmpeg -y -loop 1 -i "$INPUT" \
    -c:v libx264 -t "$DURATION" -pix_fmt yuv420p \
    -vf "scale=1920:1080:force_original_aspect_ratio=decrease,pad=1920:1080:(ow-iw)/2:(oh-ih)/2" \
    "$OUTPUT"

echo "Generated: $OUTPUT (${DURATION}s)"