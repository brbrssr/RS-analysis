use serde_json::Value;
use std::ffi::CString;
use std::fs;
use std::fs::File;
use std::io::{self, Write};
use std::os::raw::c_char;
use std::env::consts::OS;

/*
    Function for file and memory
 */

pub fn file_clean(path: String) -> io::Result<()> {
    let mut file = File::create(path)?;

    file.write_all(b"")?;

    Ok(())
}

pub fn rust_string_to_c(s: &str) -> *mut c_char {
    CString::new(s).unwrap_or_default().into_raw()
}

pub fn write_data(json_data: Value, path: String) -> Result<(), String> {
    let mut data_array = if let Ok(content) = fs::read_to_string(&path) {
        if content.trim().is_empty() {
            Vec::new()
        } else {
            serde_json::from_str::<Vec<Value>>(&content)
                .map_err(|e| format!("Ошибка при парсинге JSON: {}", e))?
        }
    } else {
        Vec::new()
    };

    match json_data {
        Value::Array(arr) => data_array.extend(arr),
        other => data_array.push(other),
    }

    let json_string = serde_json::to_string_pretty(&data_array)
        .map_err(|e| format!("Ошибка при сериализации JSON: {}", e))?;

    fs::write(path, json_string).map_err(|e| format!("Ошибка при записи в файл: {}", e))?;

    Ok(())
}

pub fn get_os(filename: &str) -> Result<String, String>{
    let path: String = match OS { 
        "windows" => format!(".\\data\\{}", filename).to_string(),
        "linux" => format!("./data/{}", filename).to_string(),
        _ => return Err("Error: pathfinding".to_string()),
    };
    Ok(path)
}

/*
    Mathematical functions
 */

pub fn mean_squared_error(y_real: &[f64], y_pred: &[f64]) -> f64 {
    let n = y_real.len() as f64;
    let mse: f64 = y_real.iter()
    .zip(y_pred.iter())
    .map(|(real, pred)| (real - pred).powi(2))
    .sum();
     
     mse / n
}

pub fn mean(x: &[f64]) -> f64 {
    let sum: f64 = x.iter().sum();
    sum / x.len() as f64
}

pub fn std_dev(x: &[f64]) -> f64 {
    let mean_value = mean(x);
    let variance: f64 = x.iter()
        .map(|&xi| (xi - mean_value).powi(2))
        .sum::<f64>() / x.len() as f64;
    variance.sqrt()
}

pub fn median(x: &[f64]) -> f64 {
    let mut temp = x.to_vec();
    temp.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = temp.len();
    if n % 2 == 0 {
        (temp[n / 2 - 1] + temp[n / 2]) / 2.0
    } else {
        temp[n / 2]
    }
}