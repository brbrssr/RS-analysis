use reqwest;
use serde::Serialize;
use serde_json::{Value, json};
use std::ffi::{CStr, CString};
use std::fs::File;
use std::io::Write;
use std::os::raw::c_char;

#[derive(Serialize)]
struct CandleData {
    time: i64,
    price: f64,
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn free_rust_heap(s: *mut c_char) {
    if s.is_null() {
        return;
    }
    unsafe {
        let _ = CString::from_raw(s);
    }
}

fn rust_string_to_c(s: &str) -> *mut c_char {
    CString::new(s).map_or(std::ptr::null_mut(), |c_string| c_string.into_raw())
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_price_series(
    pair: *const c_char,
    interval: *const c_char,
    date: *const c_char,
) -> *mut c_char {
    let symbol = unsafe { CStr::from_ptr(pair).to_string_lossy().into_owned() };
    let interval = unsafe { CStr::from_ptr(interval).to_string_lossy().into_owned() };
    let date = unsafe { CStr::from_ptr(date).to_string_lossy().into_owned() };

    let start_time = match chrono::DateTime::parse_from_rfc3339(&date) {
        Ok(dt) => dt.timestamp_millis(),
        Err(_) => return rust_string_to_c("Error: invalid data format"),
    };

    let url = format!(
        "https://api.binance.com/api/v3/klines?symbol={}&interval={}&startTime={}",
        symbol, interval, start_time
    );

    let response = match reqwest::blocking::get(&url) {
        Ok(resp) => resp,
        Err(_) => return rust_string_to_c("Error: failed request"),
    };

    let data: Value = match response.json() {
        Ok(json) => json,
        Err(_) => return rust_string_to_c("Error: failed parse JSON."),
    };

    let filter_data: Vec<CandleData> = match data.as_array() {
        Some(array) => array
            .iter()
            .filter_map(|candle| {
                let time = candle.get(0)?.as_i64()?;

                let price_str = candle.get(4)?.as_str()?;
                let price = price_str.parse::<f64>().ok()?;
                Some(CandleData { time, price })
            })
            .collect(),
        None => return rust_string_to_c("Error: data isn't array"),
    };

    let json_data = json!(filter_data);

    let mut file = match File::create("price_series.json") {
        Ok(f) => f,
        Err(_) => return rust_string_to_c("Error: failed to create a file"),
    };

    if write!(
        file,
        "{}",
        serde_json::to_string_pretty(&json_data).unwrap()
    )
    .is_err()
    {
        return rust_string_to_c("Error: failed to write data");
    }

    rust_string_to_c("Success")
}
