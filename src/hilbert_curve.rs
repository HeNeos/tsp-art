/// Computes the Hilbert curve distance for a point (x, y) in a grid of size n x n.
/// Based on the algorithm to convert (x, y) to a distance d along the curve.
fn hilbert_distance(mut x: u32, mut y: u32, n: u32) -> u32 {
    let mut d = 0;
    let mut s = n / 2;
    while s > 0 {
        let rx = (x & s) > 0;
        let ry = (y & s) > 0;
        d += s * s * ((3 * rx as u32) ^ ry as u32);
        if !ry {
            if rx {
                x = n - 1 - x;
                y = n - 1 - y;
            }
            std::mem::swap(&mut x, &mut y);
        }
        s /= 2;
    }
    d
}

/// Orders points using the Hilbert curve to ensure a non-crossing path.
pub fn order_points_by_hilbert(points: &[(f32, f32)], width: f32, height: f32) -> Vec<(f32, f32)> {
    // Find the maximum dimension and use it as the grid size (must be power of 2)
    let max_dim = width.max(height).ceil() as u32;
    let n = max_dim.next_power_of_two();
    let scale_x = n as f32 / width;
    let scale_y = n as f32 / height;

    // Map points to Hilbert distances
    let mut indexed_points: Vec<(u32, (f32, f32))> = points
        .iter()
        .map(|&(x, y)| {
            let ix = (x * scale_x).min(n as f32 - 1.0) as u32;
            let iy = (y * scale_y).min(n as f32 - 1.0) as u32;
            (hilbert_distance(ix, iy, n), (x, y))
        })
        .collect();

    // Sort by Hilbert distance
    indexed_points.sort_by_key(|&(dist, _)| dist);

    // Return ordered points
    indexed_points.into_iter().map(|(_, point)| point).collect()
}
