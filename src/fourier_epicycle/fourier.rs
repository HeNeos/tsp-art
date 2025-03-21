use rustfft::{FftPlanner, num_complex::Complex};
use std::f32::consts::PI;

pub struct Epicycle {
    pub radius: f32,
    pub freq: i32,
    pub phase: f32,
}

pub fn compute_fourier_series(
    points: &[(f32, f32)],
    num_epicycles: usize,
) -> (Complex<f32>, Vec<Epicycle>) {
    let n_points: usize = points.len();
    let mut buffer: Vec<Complex<f32>> = points
        .iter()
        .map(|&(x, y)| Complex { re: x, im: y })
        .collect();
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(n_points);
    fft.process(&mut buffer);

    let c_0 = buffer[0] / n_points as f32;

    let mut coeffs: Vec<(i32, f32, f32)> = (1..n_points)
        .map(|k| {
            let freq = if k <= n_points / 2 {
                k as i32
            } else {
                k as i32 - n_points as i32
            };
            let coeff = buffer[k] / n_points as f32;
            let radius = coeff.norm();
            let phase = coeff.arg();
            (freq, radius, phase)
        })
        .collect();

    coeffs.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    let num_epicycles = num_epicycles.min(coeffs.len());
    let selected_coeffs = &coeffs[0..num_epicycles];

    let epicycles: Vec<Epicycle> = selected_coeffs
        .iter()
        .map(|&(freq, radius, phase)| Epicycle {
            radius,
            freq,
            phase,
        })
        .collect();

    (c_0, epicycles)
}

pub fn compute_position(c_0: Complex<f32>, epicycles: &[Epicycle], t: f32) -> (f32, f32) {
    let mut x = c_0.re;
    let mut y = c_0.im;
    for epicycle in epicycles {
        let angle = 2.0 * PI * epicycle.freq as f32 * t + epicycle.phase;
        x += epicycle.radius * angle.cos();
        y += epicycle.radius * angle.sin();
    }
    (x, y)
}
