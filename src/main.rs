mod fourier_epicycle;
mod image;
mod input;
mod save_svg;
mod stippling;
mod svg_generator;
mod tour_generation;

use crate::fourier_epicycle::fourier::{compute_fourier_series, compute_position};
use crate::image::image_processing::load_and_grayscale;
use crate::input::input::Args;
use crate::save_svg::save::save_batch;
use crate::stippling::stippling::generate_stippling;
use crate::svg_generator::svg_generator::{
    add_voronoi_cells, generate_fourier_svg, generate_svg_stippling, generate_tsp_svg,
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
    let iterations: usize = args.iterations;
    let seed = 42;

    let (image, grayscale_image) = load_and_grayscale(&image_path);
    let (width, height) = grayscale_image.dimensions();

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

    if args.voronoid_cells {
        svg = add_voronoi_cells(svg, &points, width, height);
    }

    if args.tour || args.fourier_epicycles {
        let geo_points: Vec<Point<f32>> = points.iter().map(|&(x, y)| Point::new(x, y)).collect();
        let multi_point = MultiPoint::from(geo_points);
        let hull = multi_point.convex_hull();
        let hull_points: Vec<Point<f32>> = hull.exterior().points().collect();

        let tour = if args.points > 2048 {
            let tsp = Tour::new(GreedyStrategy);
            tsp.tour(&points, &hull_points)
        } else {
            let tsp = Tour::new(CheapestInsertionStrategy);
            tsp.tour(&points, &hull_points)
        };

        if args.tour {
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
        }

        if args.fourier_epicycles {
            let output_dir = "frames_out";
            let tour_points: Vec<(f32, f32)> = tour.iter().map(|&i| points[i]).collect();

            let (c_0, epicycles) = compute_fourier_series(&tour_points, num_points);

            let mut trace_points = Vec::new();
            let num_frames = num_points * 6;
            let mut svg_documents: Vec<Document> = Vec::new();
            let mut frames: Vec<usize> = Vec::new();
            let mut path_data: Option<String> = None;
            for frame in 0..num_frames {
                if frame % 100 == 0 {
                    println!("Frame {}", frame);
                }
                let t = frame as f32 / (num_frames - 1) as f32;
                let position = compute_position(c_0, &epicycles, t);
                trace_points.push(position);

                let (current_svg, new_path_data) = generate_fourier_svg(
                    svg.clone(),
                    c_0,
                    &epicycles,
                    t,
                    &trace_points,
                    args.max_stroke_width,
                    path_data.as_deref(),
                );
                path_data = Some(new_path_data);
                svg_documents.push(current_svg);
                frames.push(frame);
                save_batch(&mut svg_documents, &mut frames, false, output_dir);
            }
            save_batch(&mut svg_documents, &mut frames, true, output_dir);
        }
    }

    svg::save(&output_path, &svg).expect("Failed to save SVG");
}
