use super::tour_strategy::{CheapestInsertionStrategy, TourStrategy};
use super::utils::{distance, two_opt};
use geo::Point;
use std::collections::HashSet;

impl TourStrategy for CheapestInsertionStrategy {
    fn build_tour(&self, points: &Vec<(f32, f32)>, hull_points: &Vec<Point<f32>>) -> Vec<usize> {
        let mut hull_indices: Vec<usize> = Vec::new();
        for hp in hull_points {
            if let Some(pos) = points.iter().position(|&(x, y)| {
                (x - hp.x()).abs() < std::f32::EPSILON && (y - hp.y()).abs() < std::f32::EPSILON
            }) {
                hull_indices.push(pos);
            }
        }

        let mut tour: Vec<usize> = hull_indices.clone();
        let mut in_tour: HashSet<usize> = tour.iter().copied().collect();

        let mut remaining: Vec<usize> =
            (0..points.len()).filter(|i| !in_tour.contains(i)).collect();

        while !remaining.is_empty() {
            let mut best_candidate: Option<usize> = None;
            let mut best_increase: f32 = std::f32::INFINITY;
            let mut best_insert_position: usize = 0;
            let mut best_candidate_idx_in_remaining: usize = 0;

            for (r_idx, &candidate) in remaining.iter().enumerate() {
                for i in 0..tour.len() {
                    let j: usize = (i + 1) % tour.len();
                    let cost: f32 = distance(points[tour[i]], points[candidate])
                        + distance(points[candidate], points[tour[j]])
                        - distance(points[tour[i]], points[tour[j]]);
                    if cost < best_increase {
                        best_increase = cost;
                        best_candidate = Some(candidate);
                        best_insert_position = j;
                        best_candidate_idx_in_remaining = r_idx;
                    }
                }
            }
            if let Some(candidate) = best_candidate {
                tour.insert(best_insert_position, candidate);
                in_tour.insert(candidate);
                remaining.remove(best_candidate_idx_in_remaining);
            } else {
                break;
            }
        }

        two_opt(&points, &mut tour);
        tour
    }
}
