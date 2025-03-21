use crate::stippling::point::PointColor;
use svg::Document;
use svg::node::element::{Circle, Group, Line, Path};
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
            .set("stroke-linecap", "round");
        document = document.add(line);
    }

    document
}
