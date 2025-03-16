pub fn distance(p1: (f32, f32), p2: (f32, f32)) -> f32 {
    let dx = p2.0 - p1.0;
    let dy = p2.1 - p1.1;
    (dx * dx + dy * dy).sqrt()
}

pub fn two_opt(points: &[(f32, f32)], tour: &mut Vec<usize>) {
    let n = tour.len();
    let mut improved = true;
    let max_iterations = 5;
    let mut iteration = 0;

    while improved && iteration < max_iterations {
        improved = false;
        iteration += 1;

        for i in 0..n - 1 {
            for j in i + 2..n {
                if i == 0 && j == n - 1 {
                    continue;
                }
                let old_cost = distance(points[tour[i]], points[tour[i + 1]])
                    + distance(points[tour[j]], points[tour[(j + 1) % n]]);
                let new_cost = distance(points[tour[i]], points[tour[j]])
                    + distance(points[tour[i + 1]], points[tour[(j + 1) % n]]);
                if new_cost < old_cost {
                    tour[i + 1..=j].reverse();
                    improved = true;
                }
            }
        }
    }
}
