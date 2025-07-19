use std::f64;

fn linear_regression(x: &[f64], y: &[f64]) -> (f64, f64) {
    let n = x.len() as f64;
    let sum_x = x.iter().sum::<f64>();
    let sum_y = y.iter().sum::<f64>();
    let sum_xy = x.iter()
        .zip(y.iter())
        .map(|(xi,yi)| xi * yi)
        .sum::<f64>();
    let sum_x2 = x.iter().map(|xi| xi * xi).sum::<f64>();

    let slope = (n * sum_xy - sum_x * sum_y) / (n * sum_x2 - sum_x.powi(2));
    let intercept = sum_y / n - slope * (sum_x / n);
    (slope, intercept)
}


fn rs_analysis(
    series: &[f64],
    min_window: usize,
    n_iter: usize
) -> (Vec<usize>, Vec<f64>) {
    let n = series.len();
    let max_window = n / 2;
    let num_points = n_iter;
    let log_min = (min_window as f64).log10();
    let log_max = (max_window as f64).log10();

    let mut window_sizes = Vec::with_capacity(num_points);
    let mut rs_values = Vec::with_capacity(num_points);

    for i in 0..num_points {
        let exponent = log_min + (log_max - log_min) * (i as f64) /
            ((num_points - 1) as f64);

        let mut window = 10f64.powf(exponent).round() as usize;

        window = window.max(min_window).min(n);
        window_sizes.push(window);

        let segments = n / window;
        let mut rs_segment_values = Vec::new();

        for j in 0..segments {
            let start = j * window;
            let end = start + window;
            let segment = &series[start..end];

            let mean: f64 = segment.iter().sum::<f64>() / (segment.len() as f64);

            let mut cumulative = Vec::with_capacity(segment.len());
            let mut sum = 0.0;
            for &value in segment {
                sum += value - mean;
                cumulative.push(sum);
            }

            let r = cumulative.iter().cloned().fold(f64::NEG_INFINITY, f64::max)
                - cumulative.iter().cloned().fold(f64::INFINITY, f64::min);

            let s = (
                segment.iter()
                .map(|&x| (x - mean).powi(2)).sum::<f64>()
                / (segment.len() as f64)
            ).sqrt();

            if s > 0.0 {
                rs_segment_values.push(r / s);
            }
        }

        let rs_avg = if !rs_segment_values.is_empty() {
            rs_segment_values.iter().sum::<f64>() / (rs_segment_values.len() as f64)
        } else {
            f64::NAN
        };
        rs_values.push(rs_avg);
    }

    (window_sizes, rs_values)
}


pub fn calc_herst(
    series: &[f64],
    min_window: usize,
    n_iter: usize,
) -> Result<(f64, f64),  String> {
    if series.is_empty() {
        "Error[rs-analysis]: empty data series";
    }

    let (window_sizes, rs_series) = rs_analysis(&series, min_window, n_iter);

    let log_window_sizes: Vec<f64> = window_sizes.iter().map(|&w| (w as f64).log10()).collect();
    let log_rs_series: Vec<f64> = rs_series.iter().map(|&v| v.log10()).collect();

    let (slope, intercept) = linear_regression(&log_window_sizes, &log_rs_series);

    Ok((slope, intercept))
}