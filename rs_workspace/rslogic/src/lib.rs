use price;
use rs;
use std::ffi::CString;
use std::os::raw::c_char;
use arfima;

#[unsafe(no_mangle)]
pub extern "C" fn get_price(
    symbol: *const c_char,
    interval: *const c_char,
    date: *const c_char,
) -> *mut c_char {
    price::get_price_series(symbol, interval, date)
}

#[unsafe(no_mangle)]
pub extern "C" fn free_heap(s: *mut c_char) {
    if s.is_null() {
        return;
    }
    unsafe {
        let _ = CString::from_raw(s);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn get_rs(
    min_window: *const c_char,
    n_iter: *const c_char,
) -> *mut c_char {
    rs::get_rs_series(min_window, n_iter)
}

#[unsafe(no_mangle)]
pub extern "C" fn get_arfima(
    freq: *const std::ffi::c_char,
    alpha: *const std::ffi::c_char,
    ub: *const std::ffi::c_char,
    hybrid: *const std::ffi::c_char,
    max_iters: *const std::ffi::c_char,
    nlags: *const c_char,
    max_iters_grid:  *const c_char,
) -> *mut c_char {
    arfima::arfima(freq, alpha, ub, hybrid, max_iters,nlags,max_iters_grid)
}