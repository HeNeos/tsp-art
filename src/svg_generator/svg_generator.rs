use crate::fourier_epicycle::fourier::Epicycle;
use crate::stippling::point::PointColor;
use rustfft::num_complex::Complex;
use svg::Document;
use svg::node::element::{Circle, Group, Line, Path, Polyline};
use voronator::VoronoiDiagram;
use voronator::delaunator::Point;

pub fn generate_svg_stippling(
    mut document: Document,
    points: &[(f32, f32)],
    darkness_values: &[f32],
    colors: &[PointColor],
    min_radius: f32,
    max_radius: f32,
) -> Document {
    let max_darkness = darkness_values.iter().cloned().fold(0.0, f32::max);

    for (i, &(x, y)) in points.iter().enumerate() {
        let normalized_darkness = if max_darkness > 0.0 {
            darkness_values[i] / max_darkness
        } else {
            0.0
        };
        let color: String = format!("rgb({},{},{})", colors[i].r, colors[i].g, colors[i].b);
        let radius: f32 = min_radius + normalized_darkness * (max_radius - min_radius);
        let circle: Circle = Circle::new()
            .set("cx", x)
            .set("cy", y)
            .set("r", radius)
            .set("fill", color);
        document = document.add(circle);
    }
    document
}

pub fn add_voronoi_cells(
    mut document: Document,
    points: &[(f32, f32)],
    width: u32,
    height: u32,
) -> Document {
    let points_f64: Vec<(f64, f64)> = points.iter().map(|&(x, y)| (x as f64, y as f64)).collect();
    let voronoi = VoronoiDiagram::<Point>::from_tuple(
        &(0.0, 0.0),
        &(width as f64, height as f64),
        &points_f64,
    )
    .unwrap();

    let mut voronoi_group = Group::new()
        .set("stroke", "rgba(0, 0, 0, 0.5)")
        .set("stroke-width", 0.5)
        .set("fill", "none");

    for cell in voronoi.cells() {
        if cell.points().is_empty() {
            continue;
        }
        let mut path_data = String::new();
        let cell_points = cell.points();
        if let Some(first_point) = cell_points.first() {
            path_data.push_str(&format!("M {} {}", first_point.x, first_point.y));
            for point in cell_points.iter().skip(1) {
                path_data.push_str(&format!(" L {} {}", point.x, point.y));
            }
            path_data.push_str(" Z");
            let path = Path::new().set("d", path_data);
            voronoi_group = voronoi_group.add(path);
        }
    }

    document = document.add(voronoi_group);
    document
}

pub fn generate_tsp_svg(
    mut document: Document,
    points: &[(f32, f32)],
    tour: &[usize],
    darkness_values: &[f32],
    colors: &[PointColor],
    min_stroke_width: f32,
    max_stroke_width: f32,
    line_color: Option<(u8, u8, u8)>,
) -> Document {
    let max_darkness = darkness_values.iter().cloned().fold(0.0, f32::max);

    let mut path_segments = Vec::new();
    let mut stroke_widths = Vec::new();
    let mut segment_colors = Vec::new();

    for i in 0..tour.len() {
        let current_idx = tour[i];
        let next_idx = tour[(i + 1) % tour.len()];
        let (x1, y1) = points[current_idx];
        let (x2, y2) = points[next_idx];

        let avg_darkness = (darkness_values[current_idx] + darkness_values[next_idx]) / 2.0;
        let normalized_avg_darkness = if max_darkness > 0.0 {
            avg_darkness / max_darkness
        } else {
            0.0
        };
        let stroke_width =
            min_stroke_width + normalized_avg_darkness * (max_stroke_width - min_stroke_width);
        stroke_widths.push(stroke_width);

        let segment_color = match line_color {
            Some(color) => format!("rgb({},{},{})", color.0, color.1, color.2),
            None => {
                let avg_r = (colors[current_idx].r as u16 + colors[next_idx].r as u16) / 2;
                let avg_g = (colors[current_idx].g as u16 + colors[next_idx].g as u16) / 2;
                let avg_b = (colors[current_idx].b as u16 + colors[next_idx].b as u16) / 2;
                format!("rgb({},{},{})", avg_r, avg_g, avg_b)
            }
        };
        segment_colors.push(segment_color);

        path_segments.push((x1, y1, x2, y2));
    }

    for (i, (x1, y1, x2, y2)) in path_segments.iter().enumerate() {
        let line = Line::new()
            .set("x1", *x1)
            .set("y1", *y1)
            .set("x2", *x2)
            .set("y2", *y2)
            .set("stroke", &*segment_colors[i])
            .set("stroke-width", stroke_widths[i])
            .set("stroke-opacity", 0.2)
            .set("stroke-linecap", "round");
        document = document.add(line);
    }

    document
}

pub fn generate_fourier_svg(
    mut document: Document,
    c_0: Complex<f32>,
    epicycles: &[Epicycle],
    time: f32,
    points: &[(f32, f32)],
    trace_points: &[(f32, f32)],
    colors: &[PointColor],
    tour: &[usize],
    darkness_values: &[f32],
    min_stroke_width: f32,
    max_stroke_width: f32,
) -> Document {
    let mut x = c_0.re;
    let mut y = c_0.im;

    let max_radius = epicycles.iter().map(|e| e.radius).fold(0.0, f32::max);

    for epicycle in epicycles {
        let angle = 2.0 * std::f32::consts::PI * epicycle.freq as f32 * time + epicycle.phase;
        let dx = epicycle.radius * angle.cos();
        let dy = epicycle.radius * angle.sin();
        let center_x = x;
        let center_y = y;
        x += dx;
        y += dy;

        let opacity = (0.2 + 0.8 * (epicycle.radius / max_radius)).min(1.0);
        let opacity_str = format!("{:.2}", opacity);

        let circle = Circle::new()
            .set("cx", center_x)
            .set("cy", center_y)
            .set("r", epicycle.radius)
            .set("fill", "none")
            .set("stroke", "gray")
            .set("stroke", format!("rgba(128, 128, 128, {})", opacity_str))
            .set("stroke-width", 2.0 * opacity);
        document = document.add(circle);

        let line = Line::new()
            .set("x1", center_x)
            .set("y1", center_y)
            .set("x2", x)
            .set("y2", y)
            .set("stroke", format!("rgba(0, 0, 255, {})", opacity_str))
            .set("stroke-width", 2.0 * opacity);
        document = document.add(line);
    }

    if !trace_points.is_empty() && trace_points.len() >= 2 {
        let max_darkness = darkness_values.iter().cloned().fold(0.0, f32::max);
        let mut path_segments = Vec::new();

        for i in 0..trace_points.len() - 1 {
            let (x1, y1) = trace_points[i];
            let (x2, y2) = trace_points[i + 1];
            path_segments.push((i, x1, y1, x2, y2));
        }

        for (_idx, x1, y1, x2, y2) in path_segments {
            let (segment_color, stroke_width) = {
                let mut closest_tour_idx = 0;
                let mut min_distance = f32::MAX;

                let segment_midpoint = ((x1 + x2) / 2.0, (y1 + y2) / 2.0);

                for i in 0..tour.len() {
                    let tour_point = points[tour[i]];
                    let dx = segment_midpoint.0 - tour_point.0;
                    let dy = segment_midpoint.1 - tour_point.1;
                    let distance = (dx * dx + dy * dy).sqrt();
                    if distance < min_distance {
                        min_distance = distance;
                        closest_tour_idx = i;
                    }
                }

                let current_idx = tour[closest_tour_idx];
                let next_idx = tour[(closest_tour_idx + 1) % tour.len()];

                let avg_r = (colors[current_idx].r as u16 + colors[next_idx].r as u16) / 2;
                let avg_g = (colors[current_idx].g as u16 + colors[next_idx].g as u16) / 2;
                let avg_b = (colors[current_idx].b as u16 + colors[next_idx].b as u16) / 2;

                let avg_darkness = (darkness_values[current_idx] + darkness_values[next_idx]) / 2.0;
                let normalized_avg_darkness = if max_darkness > 0.0 {
                    avg_darkness / max_darkness
                } else {
                    0.0
                };

                let width = min_stroke_width
                    + normalized_avg_darkness * (max_stroke_width - min_stroke_width);

                (format!("rgb({},{},{})", avg_r, avg_g, avg_b), width)
            };

            let line = Line::new()
                .set("x1", x1)
                .set("y1", y1)
                .set("x2", x2)
                .set("y2", y2)
                .set("stroke", segment_color)
                .set("stroke-width", stroke_width)
                .set("stroke-linecap", "round")
                .set("stroke-opacity", 1.0);

            document = document.add(line);
        }
    }
    document
}
