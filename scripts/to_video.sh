#!/bin/bash
# Video Generation Pipeline: SVG→PNG→MP4 (Parallelized Version)
# Usage: ./generate_video_parallel.sh <svg_dir> <png_dir> [<threads>]
set -eo pipefail

# Check dependencies
command -v convert >/dev/null 2>&1 || { echo >&2 "ImageMagick required (convert)"; exit 1; }
command -v ffmpeg >/dev/null 2>&1 || { echo >&2 "FFmpeg required"; exit 1; }
command -v parallel >/dev/null 2>&1 || { 
    echo >&2 "GNU Parallel required. Install with:"
    echo >&2 "  Ubuntu/Debian: sudo apt-get install parallel"
    echo >&2 "  macOS: brew install parallel"
    echo >&2 "  CentOS/RHEL: sudo yum install parallel"
    exit 1; 
}

# Validate arguments
if [ $# -lt 2 ] || [ $# -gt 3 ]; then
    echo "Usage: $0 <svg_dir> <png_dir> [<threads>]"
    echo "Example: $0 ./svgs ./pngs 8"
    exit 1
fi

SVG_DIR="$1"
PNG_DIR="$2"
OUTPUT_VIDEO="out.mp4"
FRAMERATE="30"
# Use number of CPU cores if threads not specified
THREADS=${3:-$(nproc || sysctl -n hw.ncpu || echo 4)}

# Create PNG directory
mkdir -p "$PNG_DIR"

# Count total SVG files
TOTAL_FILES=$(find "$SVG_DIR" -maxdepth 1 -type f -name '*.svg' | wc -l)
echo "Found $TOTAL_FILES SVG files to convert"
echo "Using $THREADS parallel jobs"

# Convert function for parallel processing
convert_svg_to_png() {
    local svg_file="$1"
    local png_file="$PNG_DIR/$(basename "${svg_file%.*}").png"
    
    # Convert SVG to PNG with white background
    convert -background white -flatten -density 300 "$svg_file" "$png_file"
    
    # Print progress indicator
    echo -n "."
}
export -f convert_svg_to_png
export PNG_DIR

echo "Converting SVGs to PNGs with white background..."

# Run conversions in parallel
find "$SVG_DIR" -maxdepth 1 -type f -name '*.svg' | sort | \
    parallel -j "$THREADS" convert_svg_to_png

echo -e "\nAll conversions complete!"

echo -e "\nGenerating video..."
ffmpeg -y -framerate "$FRAMERATE" \
    -pattern_type glob -i "$PNG_DIR/*.png" \
    -c:v libx264 \
    -preset slow \
    -crf 20 \
    -pix_fmt yuv420p \
    -vf "pad=ceil(iw/2)*2:ceil(ih/2)*2" \
    "$OUTPUT_VIDEO"

echo -e "\nVideo created: $OUTPUT_VIDEO"
