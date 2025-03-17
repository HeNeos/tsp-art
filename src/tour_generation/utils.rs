use rayon::prelude::*;

pub fn distance(p1: (f32, f32), p2: (f32, f32)) -> f32 {
    let dx = p2.0 - p1.0;
    let dy = p2.1 - p1.1;
    dx * dx + dy * dy
}

pub fn two_opt(points: &[(f32, f32)], tour: &mut Vec<usize>) {
    let n = tour.len();
    let mut improved = true;
    let max_iterations = 5;
    let mut iteration = 0;

    while improved && iteration < max_iterations {
        improved = false;
        iteration += 1;

        let chunk_size = n.max(100) / rayon::current_num_threads();
        let improvements: Vec<Option<(usize, usize, f32)>> = (0..n)
            .step_by(chunk_size)
            .collect::<Vec<_>>()
            .par_iter()
            .map(|&chunk_start| {
                let chunk_end = (chunk_start + chunk_size).min(n - 1);
                let mut best_improvement = None;

                for i in chunk_start..chunk_end {
                    if i >= n - 1 {
                        break;
                    }

                    for j in i + 2..n {
                        if i == 0 && j == n - 1 {
                            continue;
                        }

                        let old_cost = distance(points[tour[i]], points[tour[i + 1]])
                            + distance(points[tour[j]], points[tour[(j + 1) % n]]);
                        let new_cost = distance(points[tour[i]], points[tour[j]])
                            + distance(points[tour[i + 1]], points[tour[(j + 1) % n]]);

                        let improvement = old_cost - new_cost;
                        if improvement > 0.0 {
                            match best_improvement {
                                None => best_improvement = Some((i, j, improvement)),
                                Some((_, _, best_imp)) if improvement > best_imp => {
                                    best_improvement = Some((i, j, improvement));
                                }
                                _ => {}
                            }
                        }
                    }
                }

                best_improvement
            })
            .collect();

        if let Some((i, j, _)) = improvements
            .into_iter()
            .flatten()
            .max_by(|a, b| a.2.partial_cmp(&b.2).unwrap_or(std::cmp::Ordering::Equal))
        {
            tour[i + 1..=j].reverse();
            improved = true;
        }
    }
}
