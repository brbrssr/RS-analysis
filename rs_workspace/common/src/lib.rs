use serde_json::Value;
use std::ffi::CString;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::os::raw::c_char;
use std::env::consts::OS;

/*
    Function for file and memory
*/

pub fn file_clean(path: String) -> Result<(), String> {
    let mut file = File::create(&path)
        .map_err(|e| format!("Error opening file {}: {}", path, e))?;
    file.write_all(b"")
        .map_err(|e| format!("Error cleaning file {}: {}", path, e))?;
    Ok(())
}

pub fn rust_string_to_c(s: &str) -> *mut c_char {
    CString::new(s).unwrap_or_default().into_raw()
}

pub fn write_data(json_data: Value, path: String) -> Result<(), String> {
    let content = fs::read_to_string(&path).unwrap_or_default();
    let mut data_array: Vec<Value> = if content.trim().is_empty() {
        Vec::new()
    } else {
        serde_json::from_str(&content)
            .map_err(|e| format!("Error parsing JSON {}: {}", path, e))?
    };

    match json_data {
        Value::Array(arr) => data_array.extend(arr),
        other => data_array.push(other),
    }

    let json_string = serde_json::to_string_pretty(&data_array)
        .map_err(|e| format!("Error serializing JSON: {}", e))?;
    fs::write(&path, json_string)
        .map_err(|e| format!("Error writing to {}: {}", path, e))?;
    Ok(())
}

pub fn get_os(filename: &str) -> Result<String, String> {
    let path = match OS {
        "windows" => format!(".\\data\\{}", filename),
        "linux" => format!("./data/{}", filename),
        _ => return Err("Unsupported OS".to_string()),
    };
    Ok(path)
}

/*
    Mathematical functions
*/

pub fn root_mean_squared_error(y_real: &[f64], y_pred: &[f64]) -> f64 {
    let n = y_real.len().min(y_pred.len()) as f64;
    if n == 0.0 {
        return 0.0;
    }
    let mse: f64 = y_real.iter().zip(y_pred.iter())
        .map(|(real, pred)| (real - pred).powi(2))
        .sum();
    (mse / n).sqrt()
}

pub fn mean(x: &[f64]) -> f64 {
    let n = x.len();
    if n == 0 {
        return 0.0;
    }
    let sum: f64 = x.iter().sum();
    sum / n as f64
}

pub fn variance(x: &[f64]) -> f64 {
    let n = x.len();
    if n == 0 {
        return 0.0;
    }
    let m = mean(x);
    x.iter().map(|&xi| (xi - m).powi(2)).sum::<f64>() / n as f64
}

pub fn median(x: &[f64]) -> f64 {
    let mut temp = x.to_vec();
    let n = temp.len();
    if n == 0 {
        return 0.0;
    }
    temp.sort_by(|a, b| a.partial_cmp(b).unwrap());
    if n % 2 == 0 {
        let mid = n / 2;
        (temp[mid - 1] + temp[mid]) / 2.0
    } else {
        temp[n / 2]
    }
}

pub fn autocov(series: &[f64], nlags: usize) -> Vec<f64> {
    let n = series.len();
    if n == 0 {
        return Vec::new();
    }
    let max_lag = nlags.min(n - 1);
    let mean_series = mean(series);
    let mut gamma = Vec::with_capacity(max_lag + 1);
    for lag in 0..=max_lag {
        let cov = (0..(n - lag))
            .map(|i| (series[i] - mean_series) * (series[i + lag] - mean_series))
            .sum::<f64>()
            / n as f64;
        gamma.push(cov);
    }
    gamma
}
