use common::{file_clean, rust_string_to_c, write_data};
use serde::Serialize;
use serde_json::json;
use std::f64;
use std::ffi::CStr;
use std::fs;
use std::os::raw::c_char;

#[derive(Serialize)]
struct RsSeries {
    window: usize,
    rs: f64,
}

fn linear_regression(x: &[f64], y: &[f64]) -> (f64, f64) {
    let n = x.len() as f64;
    let sum_x: f64 = x.iter().sum();
    let sum_y: f64 = y.iter().sum();
    let sum_xy: f64 = x.iter().zip(y.iter()).map(|(a, b)| a * b).sum();
    let sum_x2: f64 = x.iter().map(|a| a * a).sum();

    let slope = (n * sum_xy - sum_x * sum_y) / (n * sum_x2 - sum_x.powi(2));
    let intercept = sum_y / n - slope * (sum_x / n);

    (slope, intercept)
}

fn rs_analysis(series: &[f64], min_window: usize, n_iter: usize) -> (Vec<usize>, Vec<f64>) {
    let n = series.len();
    let max_window = n / 2;
    let num_points = n_iter;
    let log_min = (min_window as f64).log10();
    let log_max = (max_window as f64).log10();

    let mut window_sizes = Vec::with_capacity(num_points);
    let mut rs_values = Vec::with_capacity(num_points);

    for i in 0..num_points {
        let exponent = log_min + (log_max - log_min) * (i as f64) / ((num_points - 1) as f64);

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

            let s = (segment.iter().map(|&x| (x - mean).powi(2)).sum::<f64>()
                / (segment.len() as f64))
                .sqrt();

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

pub fn get_rs_series(
    os: *const c_char,
    min_window: *const c_char,
    n_iter: *const c_char,
) -> *mut c_char {
    let os = unsafe { CStr::from_ptr(os).to_string_lossy().into_owned() };
    let min_window = unsafe { CStr::from_ptr(min_window).to_string_lossy().into_owned() };
    let n_iter = unsafe { CStr::from_ptr(n_iter).to_string_lossy().into_owned() };

    let min_window = match min_window.parse::<usize>() {
        Ok(w) => w,
        Err(e) => return rust_string_to_c(format!("Error: parsing min_window: {}", e).as_str()),
    };

    let n_iter = match n_iter.parse::<usize>() {
        Ok(n) => n,
        Err(e) => return rust_string_to_c(format!("Error: parsing n_iter: {}", e).as_str()),
    };

    let path = match os.as_str() {
        "Windows" => ".\\data\\price_series.json".to_string(),
        "Linux" => "./data/price_series.json".to_string(),
        _ => return rust_string_to_c("Error: incorrect operating system"),
    };

    let json_data = match fs::read_to_string(path) {
        Ok(data) => data,
        Err(e) => return rust_string_to_c(format!("Error: opening file: {}", e).as_str()),
    };

    let series: Vec<f64> = match serde_json::from_str::<Vec<serde_json::Value>>(&json_data) {
        Ok(value) => value
            .iter()
            .filter_map(|v| v.get("price")?.as_f64())
            .collect(),
        Err(e) => return rust_string_to_c(format!("Error: parsing JSON: {}", e).as_str()),
    };

    if series.is_empty() {
        return rust_string_to_c("Error: empty data series");
    }

    let (window_sizes, rs_series) = rs_analysis(&series, min_window, n_iter);

    let log_window_sizes: Vec<f64> = window_sizes.iter().map(|&w| (w as f64).log10()).collect();

    let log_rs_series: Vec<f64> = rs_series.iter().map(|&v| v.log10()).collect();

    let (slope, _intercept) = linear_regression(&log_window_sizes, &log_rs_series);
    let hurst_exponent = slope;

    let path = match os.as_str() {
        "Windows" => ".\\data\\rs_series.json".to_string(),
        "Linux" => "./data/rs_series.json".to_string(),
        _ => return rust_string_to_c("Error: incorrect operating system"),
    };

    let glued_data: Vec<RsSeries> = window_sizes
        .iter()
        .zip(rs_series.iter())
        .map(|(&w, &r)| RsSeries { window: w, rs: r })
        .collect();

    let json_data = json!(glued_data);

    match file_clean(path.clone()) {
        Ok(()) => {}
        Err(e) => return rust_string_to_c(format!("Error: cleaning file: {}", e).as_str()),
    }

    match write_data(json_data, path.clone()) {
        Ok(()) => {}
        Err(e) => return rust_string_to_c(format!("Error: writing data: {}", e).as_str()),
    }

    rust_string_to_c(format!("{:.3}H{:.3}C", hurst_exponent, _intercept).as_str())
}
