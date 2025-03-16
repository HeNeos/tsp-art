use super::tour_strategy::{GreedyStrategy, TourStrategy};
use super::utils::{distance, two_opt};
use geo::Point;
use kdtree::KdTree;
use kdtree::distance::squared_euclidean;

impl TourStrategy for GreedyStrategy {
    fn build_tour(&self, points: &Vec<(f32, f32)>, hull_points: &Vec<Point<f32>>) -> Vec<usize> {
        let hull_indices: Vec<usize> = hull_points
            .into_iter()
            .map(|p| {
                points
                    .iter()
                    .position(|&pt| pt.0 == p.x() && pt.1 == p.y())
                    .unwrap()
            })
            .collect();

        let mut tour: Vec<usize> = hull_indices.clone();
        let mut in_tour = vec![false; points.len()];
        for &idx in &tour {
            in_tour[idx] = true;
        }

        let mut kdtree = KdTree::new(2);
        for (idx, &(x, y)) in points.iter().enumerate() {
            if !in_tour[idx] {
                kdtree.add([x, y], idx).unwrap();
            }
        }

        while tour.len() < points.len() {
            let mut best_increase = f32::INFINITY;
            let mut best_p = None;
            let mut best_k = None;

            let step = (tour.len() / 500).max(1).min(10);
            for k in (0..tour.len()).step_by(step) {
                let next_k = (k + 1) % tour.len();
                let i = tour[k];
                let j = tour[next_k];
                let p1 = points[i];
                let p2 = points[j];

                let mid_x = (p1.0 + p2.0) / 2.0;
                let mid_y = (p1.1 + p2.1) / 2.0;

                if let Ok(neighbors) = kdtree.nearest(&[mid_x, mid_y], 10, &squared_euclidean) {
                    for (_, &p) in neighbors {
                        if in_tour[p] {
                            continue;
                        }

                        let increase = distance(points[i], points[p])
                            + distance(points[p], points[j])
                            - distance(points[i], points[j]);

                        if increase < best_increase {
                            best_increase = increase;
                            best_p = Some(p);
                            best_k = Some(k);
                        }
                    }
                }
            }

            if let (Some(p), Some(k)) = (best_p, best_k) {
                let next_k = (k + 1) % tour.len();

                if next_k == 0 {
                    tour.push(p);
                } else {
                    tour.insert(next_k, p);
                }

                in_tour[p] = true;

                if let Ok(neighbors) =
                    kdtree.nearest(&[points[p].0, points[p].1], 1, &squared_euclidean)
                {
                    for &(_, &idx) in neighbors.iter() {
                        if idx == p {
                            kdtree.remove(&[points[p].0, points[p].1], &p).ok();
                            break;
                        }
                    }
                }
            } else {
                let remaining: Vec<usize> =
                    (0..points.len()).filter(|&idx| !in_tour[idx]).collect();

                if !remaining.is_empty() {
                    let p = remaining[0];
                    let k = tour.len() - 1;
                    tour.push(p);
                    in_tour[p] = true;

                    kdtree.remove(&[points[p].0, points[p].1], &p).ok();
                } else {
                    break;
                }
            }

            if tour.len() % 1000 == 0 {
                two_opt(&points, &mut tour);
            }
        }

        two_opt(&points, &mut tour);
        tour
    }
}
