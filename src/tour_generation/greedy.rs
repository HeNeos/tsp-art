use super::tour_strategy::{GreedyStrategy, TourStrategy};
use super::utils::{distance, two_opt};
use geo::Point;
use kiddo::{KdTree, SquaredEuclidean};

impl TourStrategy for GreedyStrategy {
    fn build_tour(&self, points: &Vec<(f32, f32)>, hull_points: &Vec<Point<f32>>) -> Vec<usize> {
        let mut hull_indices: Vec<usize> = hull_points
            .into_iter()
            .map(|p| {
                points
                    .iter()
                    .position(|&pt| pt.0 == p.x() && pt.1 == p.y())
                    .unwrap()
            })
            .collect();

        if hull_indices.len() > 1 && hull_indices.first() == hull_indices.last() {
            hull_indices.pop();
        }
        let mut tour: Vec<usize> = hull_indices.clone();
        let mut in_tour: Vec<bool> = vec![false; points.len()];
        for &idx in &tour {
            in_tour[idx] = true;
        }

        let mut kdtree = KdTree::<f32, 2>::with_capacity(points.len());
        for (idx, &(x, y)) in points.iter().enumerate() {
            if !in_tour[idx] {
                kdtree.add(&[x, y], idx as u64);
            }
        }

        while tour.len() < points.len() {
            let mut best_increase: f32 = f32::INFINITY;
            let mut best_p = None;
            let mut best_k = None;

            let step: usize = (tour.len() >> 8).max(1).min(8);
            let max_neighbors: usize = if step > 4 { 8 } else { 4 };
            for k in (0..tour.len()).step_by(step) {
                let next_k: usize = (k + 1) % tour.len();
                let i: usize = tour[k];
                let j: usize = tour[next_k];
                let p1: (f32, f32) = points[i];
                let p2: (f32, f32) = points[j];

                let mid_x: f32 = (p1.0 + p2.0) / 2.0;
                let mid_y: f32 = (p1.1 + p2.1) / 2.0;

                let neighbors =
                    kdtree.nearest_n::<SquaredEuclidean>(&[mid_x, mid_y], max_neighbors);
                for neighbor in neighbors {
                    let p: usize = neighbor.item as usize;
                    if in_tour[p] {
                        continue;
                    }

                    let increase: f32 = distance(points[i], points[p])
                        + distance(points[p], points[j])
                        - distance(points[i], points[j]);

                    if increase < best_increase {
                        best_increase = increase;
                        best_p = Some(p);
                        best_k = Some(k);
                    }
                }
            }

            if let (Some(p), Some(k)) = (best_p, best_k) {
                let next_k: usize = (k + 1) % tour.len();

                if next_k == 0 {
                    tour.push(p);
                } else {
                    tour.insert(next_k, p);
                }

                in_tour[p] = true;

                let neighbors =
                    kdtree.nearest_n::<SquaredEuclidean>(&[points[p].0, points[p].1], 1);
                for neighbor in neighbors.iter() {
                    let idx = neighbor.item as usize;
                    if idx == p {
                        kdtree.remove(&[points[p].0, points[p].1], p as u64);
                        break;
                    }
                }
            } else {
                let remaining: Vec<usize> =
                    (0..points.len()).filter(|&idx| !in_tour[idx]).collect();

                if !remaining.is_empty() {
                    let p: usize = remaining[0];
                    tour.push(p);
                    in_tour[p] = true;

                    kdtree.remove(&[points[p].0, points[p].1], p as u64);
                } else {
                    break;
                }
            }
            if tour.len() % 32 == 0 {
                two_opt(&points, &mut tour);
            }
        }

        two_opt(&points, &mut tour);
        tour
    }
}
