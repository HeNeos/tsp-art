mod fourier_epicycle;
mod image;
mod input;
mod stippling;
mod svg_generator;
mod tour_generation;

use crate::fourier_epicycle::fourier::{compute_fourier_series, compute_position};
use crate::image::image_processing::load_and_grayscale;
use crate::input::input::Args;
use crate::stippling::stippling::generate_stippling;
use crate::svg_generator::svg_generator::{
    generate_fourier_svg, generate_svg_stippling, generate_tsp_svg,
};
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

    let output_dir = "frames_out";
    let tour_points: Vec<(f32, f32)> = tour.iter().map(|&i| points[i]).collect();

    let (c_0, epicycles) = compute_fourier_series(&tour_points, 8192);

    let mut trace_points = Vec::new();
    let num_frames = 3600;
    for frame in 0..num_frames {
        let t = frame as f32 / (num_frames - 1) as f32;
        let position = compute_position(c_0, &epicycles, t);
        trace_points.push(position);

        let frame_svg = generate_fourier_svg(
            svg.clone(),
            c_0,
            &epicycles,
            t,
            &points,
            &trace_points,
            &colors,
            &tour,
            &darkness_values,
            args.min_stroke_width,
            args.max_stroke_width,
        );
        let output_path = format!("{}/frame_{:04}.svg", output_dir, frame);
        svg::save(&output_path, &frame_svg).expect("Failed to save SVG frame");
    }

    svg::save(&output_path, &svg).expect("Failed to save SVG");
}
