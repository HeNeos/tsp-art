mod image;
mod input;
mod stippling;
mod svg_generator;
mod tour_generation;

use crate::image::image_processing::load_and_grayscale;
use crate::input::input::Args;
use crate::stippling::stippling::generate_stippling;
use crate::svg_generator::svg_generator::{generate_svg_stippling, generate_tsp_svg};
use crate::tour_generation::tour_strategy::{CheapestInsertionStrategy, GreedyStrategy, Tour};
use clap::Parser;
use geo::{ConvexHull, MultiPoint, Point};
use svg::Document;

fn main() {
    let args = Args::parse();

    let image_path = args.image;
    let output_path = args.output;
    let num_points: usize = args.points;
    let iterations = args.iterations;
    let seed = 42; // Fixed seed for reproducibility

    let (image, grayscale_image) = load_and_grayscale(&image_path);
    let (width, height) = grayscale_image.dimensions();

    // println!("Generating {} stippling points...", num_points);
    let (points, darkness_values, colors) =
        generate_stippling(&grayscale_image, &image, num_points, seed, iterations);

    let mut svg = Document::new()
        .set("viewBox", (0, 0, width, height))
        .set("width", width)
        .set("height", height)
        .set("style", "background-color: white");

    svg = generate_svg_stippling(
        svg,
        &points,
        &darkness_values,
        &colors,
        args.min_radius,
        args.max_radius,
    );

    let geo_points: Vec<Point<f32>> = points.iter().map(|&(x, y)| Point::new(x, y)).collect();
    let multi_point = MultiPoint::from(geo_points);
    let hull = multi_point.convex_hull(); // Returns a Polygon<f32>
    let hull_points: Vec<Point<f32>> = hull.exterior().points().collect();

    let tour = if args.points > 2048 {
        let tsp = Tour::new(GreedyStrategy);
        tsp.tour(&points, &hull_points)
    } else {
        let tsp = Tour::new(CheapestInsertionStrategy);
        tsp.tour(&points, &hull_points)
    };

    svg = generate_tsp_svg(
        svg,
        &points,
        &tour,
        &darkness_values,
        &colors,
        args.min_stroke_width,
        args.max_stroke_width,
        None,
    );
    svg::save(&output_path, &svg).expect("Failed to save SVG");
}
