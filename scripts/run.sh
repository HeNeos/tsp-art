#!/bin/bash
set -e # Exit on error

if [ $# -lt 1 ] || [ $# -gt 2 ]; then
    echo "Usage: $0 <num_points> [<threads>]"
    exit 1
fi

INPUT_DIR="./frames"
OUTPUT_DIR="./frames_out"
NUM_POINTS="$1"
THREADS=${2:-$(nproc)}

mkdir -p "$OUTPUT_DIR"

if ! command -v parallel &>/dev/null; then
    echo "GNU Parallel is not installed. Please install it first:"
    echo "  Ubuntu/Debian: sudo apt-get install parallel"
    echo "  macOS: brew install parallel"
    echo "  CentOS/RHEL: sudo yum install parallel"
    exit 1
fi

TOTAL_FRAMES=$(find "$INPUT_DIR" -maxdepth 1 -type f \( -iname "*.png" -o -iname "*.jpg" -o -iname "*.jpeg" \) | wc -l)

echo "Starting parallel frame processing..."
echo "Input: $INPUT_DIR"
echo "Output: $OUTPUT_DIR"
echo "Total frames: $TOTAL_FRAMES"
echo "Using $THREADS parallel jobs"

process_frame() {
    local input_file="$1"
    local base_name=$(basename "$input_file" | cut -f 1 -d '.')
    local output_file="$OUTPUT_DIR/$base_name.svg"

    ./target/release/tsp_art \
        --image "$input_file" \
        --output "$output_file" \
        --min-radius "1.0" \
        --max-radius "4.0" \
        --points $NUM_POINTS \
        --iterations 100

    echo -n "."
}
export -f process_frame
export NUM_POINTS
export OUTPUT_DIR

find "$INPUT_DIR" -maxdepth 1 -type f \( -iname "*.png" -o -iname "*.jpg" -o -iname "*.jpeg" \) | sort |
    parallel -j $THREADS process_frame

echo ""
echo "Processing completed!"
