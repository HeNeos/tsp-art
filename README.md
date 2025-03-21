# TSP Art Generator

🔗 Convert images into stippled art with Traveling Salesman Problem (TSP) optimized tours

<div style="text-align: center;">
 <img src="https://github.com/user-attachments/assets/5916f3bc-4cbc-4065-b2d0-af0732a7f9d9" width="512">
</div>

<div style="text-align: center;">
 <img src="https://github.com/user-attachments/assets/493f2c20-a9e1-4a23-a75a-097f1dbe739f" width="512">
</div>

## Features

  - Dynamic Stippling: Convert images to density-aware point distributions

  - Voronoi Relaxation: Iteratively improve point placement using Voronoi diagrams

  - TSP Optimization:

    - Greedy algorithm for large point sets (>2048 points)

    - Cheapest insertion strategy for smaller configurations

  - Adaptive Rendering:

    - Point radius proportional to local darkness

    - Stroke width based on adjacent point contrast

  - Parallel Processing: Multi-threaded pipeline using Rayon

  - Video Generation: Create animated transformations from image sequences

## Installation

1. **Clone the Repository**:

```bash
git clone https://github.com/HeNeos/tsp_art.git
cd tsp_art
```

2. **Build the Project**:
```bash
cargo build --release
```

## Usage

### Single Image Processing

To generate a TSP art SVG from a single image:

```bash
./target/release/tsp_art --image path/to/image.jpg --output output.svg --points 1000 --iterations 50
```

**Options**:

- --image: Path to the input image (PNG, JPG, JPEG).
- --output: Path for the output SVG (default: output.svg).
- --points: Number of stippling points.
- --iterations: Number of Voronoi iterations (default: 50).
- --min-radius / --max-radius: Min/max radius for stippling points (default: 1.0 / 3.0).
- --min-stroke-width / --max-stroke-width: Min/max stroke width for TSP lines (default: 0.5 / 3.0).

### Batch Processing Frames

To process multiple images in parallel (e.g., for animation):

1. Place your input images in the frames/ directory.
2. Run the run.sh script:

```bash
./scripts/run.sh 1000 [threads]
```

Output SVGs will be saved in `frames_out/`.

### Converting to Video

To convert a sequence of SVGs into a video:

```bash
./scripts/to_video.sh ./frames_out ./pngs [threads]
```

This generates out.mp4 in the current directory at 30 FPS.

## How It Works

1. Image Processing: The input image is resized (max height 720px) and converted to grayscale.

2. Stippling: Points are distributed based on brightness using a weighted centroid algorithm with Voronoi diagrams.

3. TSP Path: Points are connected into a single path using:
  - Cheapest Insertion: For fewer than 2048 points (more accurate).
  - Greedy: For 2048+ points (faster).
  - Optimized with the 2-opt algorithm.

4. SVG Generation: Points are rendered as colored circles, and the TSP path is drawn with varying stroke widths.

## Dependencies

- `clap`: Command-line argument parsing.
- `image`: Image loading and processing.
- `rand`: Random point generation.
- `svg`: SVG file creation.
- `voronator`: Voronoi diagram computation.
- `rayon`: Parallel processing.
- `geo`: Convex hull calculation.
- `kiddo`: KD-tree for efficient nearest-neighbor searches.

See Cargo.toml for version details.

## Contributing

Feel free to open issues or submit pull requests for bug fixes, optimizations, or new features!

## License

This project is licensed under the MIT License. See LICENSE for details.
