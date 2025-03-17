use crate::stippling::point::PointColor;
use image::{DynamicImage, GenericImageView, GrayImage, Pixel};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use rayon::prelude::*;
use voronator::VoronoiDiagram;
use voronator::delaunator::Point;

/// Checks whether a point (x, y) is inside the polygon defined by vertices.
/// Uses the ray-casting algorithm.
#[inline]
fn point_in_polygon(x: f32, y: f32, polygon: &[(f32, f32)]) -> bool {
    let mut inside: bool = false;
    let n: usize = polygon.len();
    if n < 3 {
        return false;
    }

    let mut j: usize = n - 1;
    for i in 0..n {
        let (xi, yi) = polygon[i];
        let (xj, yj) = polygon[j];

        if ((yi > y) != (yj > y)) && (x < (xj - xi) * (y - yi) / (yj - yi) + xi) {
            inside = !inside;
        }
        j = i;
    }
    inside
}

pub fn generate_stippling(
    gray_image: &GrayImage,
    color_image: &DynamicImage,
    num_points: usize,
    seed: u64,
    iterations: usize,
) -> (Vec<(f32, f32)>, Vec<f32>, Vec<PointColor>) {
    let (width, height) = gray_image.dimensions();
    let width_f: f32 = width as f32;
    let height_f: f32 = height as f32;

    let brightness_map: Vec<Vec<f32>> = (0..height)
        .map(|y| {
            (0..width)
                .map(|x| gray_image.get_pixel(x, y)[0] as f32)
                .collect()
        })
        .collect();

    let mut rng: StdRng = StdRng::seed_from_u64(seed);

    let mut points: Vec<(f32, f32)> = Vec::with_capacity(num_points);
    let threshold: f32 = 24.0;

    let mut bright_pixels: Vec<(u32, u32)> = Vec::new();
    for y in 0..height {
        for x in 0..width {
            let brightness = brightness_map[y as usize][x as usize];
            if brightness > threshold {
                bright_pixels.push((x, y));
            }
        }
    }

    if bright_pixels.is_empty() {
        while points.len() < num_points {
            let x: f32 = rng.random_range(0..width) as f32;
            let y: f32 = rng.random_range(0..height) as f32;
            points.push((x, y));
        }
    } else {
        while points.len() < num_points {
            let idx: usize = rng.random_range(0..bright_pixels.len());
            let (x, y) = bright_pixels[idx];
            points.push((x as f32, y as f32));
        }
    }

    let mut average_weights: Vec<f32> = vec![0.0f32; points.len()];
    let mut max_weight: f32 = 0.0f32;

    for _ in 0..iterations {
        let points_f64: Vec<(f64, f64)> =
            points.iter().map(|&(x, y)| (x as f64, y as f64)).collect();

        let voronoi: VoronoiDiagram<Point> = VoronoiDiagram::<Point>::from_tuple(
            &(0.0, 0.0),
            &(width as f64, height as f64),
            &points_f64,
        )
        .unwrap();
        let cells = voronoi.cells();

        let cell_results: Vec<_> = cells
            .par_iter()
            .enumerate()
            .map(|(i, cell)| {
                if cell.points().is_empty() {
                    return (points[i], 0.0f32, 0);
                }

                let mut min_x = f32::INFINITY;
                let mut max_x = f32::NEG_INFINITY;
                let mut min_y = f32::INFINITY;
                let mut max_y = f32::NEG_INFINITY;

                for point in cell.points() {
                    let (vx, vy) = (point.x as f32, point.y as f32);
                    min_x = min_x.min(vx);
                    max_x = max_x.max(vx);
                    min_y = min_y.min(vy);
                    max_y = max_y.max(vy);
                }

                min_x = min_x.max(0.0);
                max_x = max_x.min(width_f - 1.0);
                min_y = min_y.max(0.0);
                max_y = max_y.min(height_f - 1.0);

                let start_x: u32 = min_x.floor() as u32;
                let end_x: u32 = max_x.ceil() as u32;
                let start_y: u32 = min_y.floor() as u32;
                let end_y: u32 = max_y.ceil() as u32;

                let cell_points: Vec<(f32, f32)> = cell
                    .points()
                    .iter()
                    .map(|p| (p.x as f32, p.y as f32))
                    .collect();

                let mut sum_x: f32 = 0.0f32;
                let mut sum_y: f32 = 0.0f32;
                let mut sum_weight: f32 = 0.0f32;
                let mut count: i32 = 0;

                for x in start_x..=end_x {
                    for y in start_y..=end_y {
                        let xf = x as f32;
                        let yf = y as f32;

                        if point_in_polygon(xf, yf, &cell_points) {
                            let brightness: f32 = brightness_map[y as usize][x as usize];
                            let weight: f32 = 1.0 - brightness / 255.0;
                            sum_x += xf * weight;
                            sum_y += yf * weight;
                            sum_weight += weight;
                            count += 1;
                        }
                    }
                }

                if sum_weight > 0.0 {
                    let new_point: (f32, f32) = (sum_x / sum_weight, sum_y / sum_weight);
                    let avg: f32 = sum_weight / (count as f32);
                    (new_point, avg, count)
                } else {
                    (points[i], 0.0f32, 0)
                }
            })
            .collect();

        let mut new_points: Vec<(f32, f32)> = Vec::with_capacity(points.len());
        let mut new_average_weights: Vec<f32> = vec![0.0f32; points.len()];
        let mut new_max_weight: f32 = 0.0f32;

        for (i, (point, weight, _)) in cell_results.into_iter().enumerate() {
            new_points.push(point);
            new_average_weights[i] = weight;
            if weight > new_max_weight {
                new_max_weight = weight;
            }
        }

        points = new_points;
        average_weights = new_average_weights;
        max_weight = new_max_weight;
    }

    let radii: Vec<f32> = average_weights
        .par_iter()
        .map(|&w| {
            if max_weight > 0.0 {
                w / max_weight
            } else {
                0.0
            }
        })
        .collect();

    let colors: Vec<PointColor> = points
        .par_iter()
        .map(|&(x, y)| {
            let xi = x.clamp(0.0, (width - 1) as f32) as u32;
            let yi = y.clamp(0.0, (height - 1) as f32) as u32;
            let pixel = color_image.get_pixel(xi, yi).to_rgba();
            PointColor {
                r: pixel[0],
                g: pixel[1],
                b: pixel[2],
            }
        })
        .collect();

    (points, radii, colors)
}
