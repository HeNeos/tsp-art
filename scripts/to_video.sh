#!/bin/bash
set -eo pipefail

command -v svgexport >/dev/null 2>&1 || { echo >&2 " Required to convert to png (svgexport)"; exit 1; }
command -v ffmpeg >/dev/null 2>&1 || { echo >&2 "FFmpeg required"; exit 1; }
command -v parallel >/dev/null 2>&1 || { 
    echo >&2 "GNU Parallel required. Install with:"
    echo >&2 "  Ubuntu/Debian: sudo apt-get install parallel"
    echo >&2 "  macOS: brew install parallel"
    echo >&2 "  CentOS/RHEL: sudo yum install parallel"
    exit 1; 
}

if [ $# -lt 2 ] || [ $# -gt 3 ]; then
    echo "Usage: $0 <svg_dir> <png_dir> [<threads>]"
    echo "Example: $0 ./svgs ./pngs 8"
    exit 1
fi

SVG_DIR="$1"
PNG_DIR="$2"
OUTPUT_VIDEO="out.mp4"
FRAMERATE="30"
THREADS=${3:-$(nproc || sysctl -n hw.ncpu || echo 4)}

mkdir -p "$PNG_DIR"

TOTAL_FILES=$(find "$SVG_DIR" -maxdepth 1 -type f -name '*.svg' | wc -l)
echo "Found $TOTAL_FILES SVG files to convert"
echo "Using $THREADS parallel jobs"

convert_svg_to_png() {
    local svg_file="$1"
    local png_file="$PNG_DIR/$(basename "${svg_file%.*}").png"
    for ((attempt=1; attempt<=3; attempt++)); do
        if svgexport "$svg_file" "$png_file" 2x 2>/dev/null; then
            echo -n "."
            return 0
        else
            if [ $attempt -lt 3 ]; then
                >&2 echo -n "R"
                sleep 5
            fi
        fi
    done
    >&2 echo -e "\nError: Failed to convert $svg_file after 3 attempts"
    return 1
}
export -f convert_svg_to_png
export PNG_DIR

echo "Converting SVGs to PNGs with white background..."

export PUPPETEER_LAUNCH_TIMEOUT=60000
find "$SVG_DIR" -maxdepth 1 -type f -name '*.svg' | sort | \
    parallel -j "$THREADS" convert_svg_to_png

echo -e "\nAll conversions complete!"

echo -e "\nGenerating video..."
ffmpeg -y -framerate "$FRAMERATE" \
    -pattern_type glob -i "$PNG_DIR/*.png" \
    -c:v libx264 \
    -preset fast \
    -crf 22 \
    -pix_fmt yuv420p \
    -vf "pad=ceil(iw/2)*2:ceil(ih/2)*2" \
    "$OUTPUT_VIDEO"

echo -e "\nVideo created: $OUTPUT_VIDEO"
