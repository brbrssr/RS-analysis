use chrono::Utc;
use common::{file_clean, rust_string_to_c, write_data,get_os};
use reqwest;
use serde_json::{Value, json};
use std::ffi::CStr;
use std::os::raw::c_char;
use tokio;

pub fn get_price_series(
    pair: *const c_char,
    interval: *const c_char,
    date: *const c_char,
) -> *mut c_char {
    let symbol = unsafe { CStr::from_ptr(pair).to_string_lossy().into_owned() };
    let interval_str = unsafe { CStr::from_ptr(interval).to_string_lossy().into_owned() };
    let date_str = unsafe { CStr::from_ptr(date).to_string_lossy().into_owned() };

    let start_time = match chrono::DateTime::parse_from_rfc3339(&date_str) {
        Ok(dt) => dt.timestamp_millis(),
        Err(e) => return rust_string_to_c(format!("Error: invalid date format: {}", e).as_str()),
    };

    let path: String = match get_os("price_series.json") {
        Ok(p) => p,
        Err(e) => return rust_string_to_c(e.as_str()),
    };

    let (num_str, unit) = interval_str.split_at(interval_str.len() - 1);
    let num: i64 = match num_str.parse::<i64>() {
        Ok(value) => value,
        Err(e) => {
            return rust_string_to_c(format!("Error: failed to parse interval: {}", e).as_str());
        }
    };

    let millis: i64 = match unit {
        "s" => num * 1000,
        "m" => num * 60 * 1000,
        "h" => num * 3600 * 1000,
        "d" => num * 24 * 3600 * 1000,
        "w" => num * 7 * 24 * 3600 * 1000,
        _ => return rust_string_to_c("Error: failed to convert interval"),
    };

    let end_time = Utc::now().timestamp_millis();

    match file_clean(path.clone()) {
        Ok(()) => {}
        Err(e) => {
            return rust_string_to_c(
                format!("Error: couldn't clear the file: [price_series.json]: {}", e).as_str(),
            );
        }
    };

    let rt = match tokio::runtime::Runtime::new() {
        Ok(runtime) => runtime,
        Err(e) => {
            return rust_string_to_c(format!("Error: failed to create runtime: {}", e).as_str());
        }
    };

    let result = rt.block_on(load_price_series_parallel(
        symbol.clone(),
        interval_str.clone(),
        start_time,
        end_time,
        path.clone(),
        millis,
    ));

    match result {
        Ok(_) => rust_string_to_c("Time series loaded successfully"),
        Err(e) => rust_string_to_c(&e),
    }
}

async fn load_price_series_parallel(
    pair: String,
    interval: String,
    start_point: i64,
    end_time: i64,
    path: String,
    millis: i64,
) -> Result<(), String> {
    let mut tasks = Vec::new();
    let mut iter_start_time = start_point;

    while iter_start_time < end_time {
        let batch_start_time = iter_start_time;
        let task = tokio::spawn({
            let pair = pair.clone();
            let interval = interval.clone();
            let path = path.clone();
            async move { load_price_series(pair, interval, batch_start_time.to_string(), path).await }
        });
        tasks.push(task);
        iter_start_time += millis * 1500;
    }

    for task in tasks {
        match task.await {
            Ok(result) => {
                if let Err(e) = result {
                    return Err(e);
                }
            }
            Err(e) => return Err(format!("Task panicked: {:?}", e)),
        }
    }

    Ok(())
}

async fn load_price_series(
    pair: String,
    interval: String,
    start_point: String,
    path: String,
) -> Result<(), String> {
    let url = format!(
        "https://api.binance.com/api/v3/klines?symbol={}&interval={}&startTime={}&limit=1500",
        pair, interval, start_point
    );

    let response = reqwest::get(&url)
        .await
        .map_err(|e| format!("Error: failed request: {}", e))?;

    let data: Value = response
        .json()
        .await
        .map_err(|e| format!("Error: failed parse JSON: {}", e))?;

    let filter_data: Vec<f64> = match data.as_array() {
        Some(array) => array
            .iter()
            .filter_map(|candle| {
                let price = candle.get(4)?.as_str()?.parse::<f64>().ok()?;
                Some(price)
            })
            .collect(),
        None => return Err("Error: data isn't array".to_string()),
    };

    let json_data = json!(filter_data);
    match write_data(json_data, path) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Error: failed to write data: {}", e)),
    }
}
