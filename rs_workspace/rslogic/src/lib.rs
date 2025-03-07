use price_series;
use rs_series;
use std::os::raw::c_char;

#[unsafe(no_mangle)]
pub extern "C" fn get_price(
    symbol: *const c_char,
    interval: *const c_char,
    date: *const c_char,
    os: *const c_char,
) -> *mut c_char {
    price_series::get_price_series(symbol, interval, date, os)
}

#[unsafe(no_mangle)]
pub extern "C" fn free_heap(ptr: *mut c_char) {
    price_series::free_rust_heap(ptr);
}

#[unsafe(no_mangle)]
pub extern "C" fn get_rs(
    os: *const c_char,
    min_window: *const c_char,
    max_window: *const c_char,
) -> *mut c_char {
    rs_series::get_rs_series(os, min_window, max_window)
}
