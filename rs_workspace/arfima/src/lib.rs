use std::ffi::{c_char, CStr};
use std::fs;
use box_cox::{box_cox, z_score};
use s_h_esd::esd_test;
use common::{file_clean, get_os, rust_string_to_c, write_data};
use serde_json::json;
use config_manager::ConfigManager;
use pacf_acf::{durbin_yw,acf,significant_lags};
use akaike_optim::grid;


fn get_weight(d: f64, thresh: f64) -> Vec<f64> {
    let mut w: Vec<f64> = vec![1.0];
    let mut k = 1;

    loop {
        let w_k = - w[w.len() - 1] * (d - (k-1) as f64) / k as f64;
        if w_k.abs() < thresh {
            break;
        }
        w.push(w_k);
        k += 1;
    }

    w
}

fn frac_diff(series: &[f64], d: f64, thresh: f64) -> Vec<f64> {
    let w = get_weight(d, thresh);
    let k = w.len().saturating_sub(1).min(series.len().saturating_sub(1));
    let mut result = Vec::new();

    for t in k..series.len() {
        let mut val = 0.0;
        for j in 0..=k{
            val += w[j] * series[t-j];
        }
        result.push(val);
    }

    result
}

pub fn arfima(
    freq: *const c_char,
    alpha: *const c_char,
    ub: *const c_char,
    hybrid: *const c_char,
    max_iters: *const c_char,
    nlags: *const c_char,
    max_iters_grid:  *const c_char,
) -> *mut c_char {
    let freq = unsafe { CStr::from_ptr(freq).to_string_lossy().into_owned() };
    let alpha = unsafe { CStr::from_ptr(alpha).to_string_lossy().into_owned() };
    let ub = unsafe { CStr::from_ptr(ub).to_string_lossy().into_owned() };
    let hybrid = unsafe { CStr::from_ptr(hybrid).to_string_lossy().into_owned() };
    let max_iters = unsafe { CStr::from_ptr(max_iters).to_string_lossy().into_owned() };
    let nlags = unsafe { CStr::from_ptr(nlags).to_string_lossy().into_owned() };
    let max_iters_grid = unsafe { CStr::from_ptr(max_iters_grid).to_string_lossy().into_owned() };
    
    let freq = match freq.parse::<usize>() {
        Ok(v) => v,
        Err(e) => return rust_string_to_c(format!("[Error] Failed to parse frequency: \n {}", e).as_str()),
    };
    let alpha = match alpha.parse::<f64>() {
        Ok(v) => v,
        Err(e) => return rust_string_to_c(format!("[Error] Failed to parse alpha: \n {}", e).as_str()),
    };

    let ub = match ub.parse::<f64>() {
        Ok(v) => v,
        Err(e) => return rust_string_to_c(format!("[Error] Failed to parse ub: \n {}", e).as_str()),
    };

    let hybrid = match hybrid.parse::<bool>() {
        Ok(v) => v,
        Err(e) => return rust_string_to_c(format!("[Error] Failed to parse hybrid: \n {}", e).as_str()),
    };

    let max_iters = match max_iters.parse::<u64>() {
        Ok(v) => v,
        Err(e) => return rust_string_to_c(format!("[Error] Failed to parse max_iters: \n {}", e).as_str()),
    };

    let nlags = match nlags.parse::<usize>() {
        Ok(v) => v,
        Err(e) => return rust_string_to_c(format!("[Error] Failed to parse nlags: \n {}", e).as_str()),
    };

    let max_iters_grid = match max_iters_grid.parse::<u64>() {
        Ok(v) => v,
        Err(e) => return rust_string_to_c(format!("[Error] Failed to parse max_iters_grid: \n {}", e).as_str()),
    };

    let path_series: String = match get_os("price_series.json") {
        Ok(p) => p,
        Err(e) => return rust_string_to_c(e.as_str()),
    };

    let path_test: String = match get_os("test.json") {
        Ok(p) => p,
        Err(e) => return rust_string_to_c(e.as_str()),
    };

    let json_data = match fs::read_to_string(path_series) {
        Ok(data) => data,
        Err(e) => return rust_string_to_c(format!("Error: opening file: {}", e).as_str()),
    };

    let mut series: Vec<f64> = match serde_json::from_str::<Vec<f64>>(&json_data) {
        Ok(vec) => vec,
        Err(e) => {
            let msg = format!("Error parsing JSON in arfima preparation: {}", e);
            return rust_string_to_c(&msg);
        }
    };

    if series.is_empty() {
        return rust_string_to_c("Error: empty data series");
    }

    let anomalies = esd_test(&series, freq, Some(alpha), Some(ub), Some(hybrid));

    for &i in &anomalies {
        if let (Some(&left), Some(&right)) = (
            series.get(i.wrapping_sub(1)),
            series.get(i + 1)
        ) {
            series[i] = (left + right) / 2.0;
        }
    }

    let (lambda, cost, data) = match box_cox(&series, max_iters) {
        Ok(v) => (v[0][0], v[0][1], v[1].clone()),
        Err(e) => return rust_string_to_c(format!("[Error] {}", e).as_str()),
    };

    let z_data = z_score(&data);
    
    match file_clean(path_test.clone()) {
        Ok(()) => {}
        Err(e) => return rust_string_to_c(format!("Error: cleaning file: {}", e).as_str()),
    }

    match file_clean("./data/preds.json".to_string().clone()) {
        Ok(()) => {}
        Err(e) => return rust_string_to_c(format!("Error: cleaning file: {}", e).as_str()),
    }

    let config = match ConfigManager::get_config(){
        Ok(config) => config,
        Err(e) => return rust_string_to_c(format!("{}", e).as_str()),
    };
    
    let intagrate_data = frac_diff(&z_data, config.herst - 0.5, 1e-4);
    
    let thr = 1.96 / (intagrate_data.len() as f64).sqrt();

    let pacf = durbin_yw(&intagrate_data, nlags).pacf;
    let acf = acf(&intagrate_data, nlags);

    let p0 = significant_lags(&pacf,thr);
    let q0 = significant_lags(&acf,thr);
    
    println!("p0: {:?} \n q0: {:?}", p0, q0);

    let (train, valid) = train_test_split(&intagrate_data);
    
    let (params, psi) = match grid(&train, p0, q0, max_iters_grid) {
        Ok(v) => v,
        Err(e) => return rust_string_to_c(format!("[Error] Error fitting ARIMA: {}", e).as_str()),
    };

    let preds = forecast(&train, &psi,params[0], params[1], valid.len());
    
    println!("params: {:?} \n psi: {:?}", params, psi);
    
    let preds_json = json![&preds];

    match write_data(preds_json, "./data/preds.json".to_string().clone()) {
        Ok(()) => {}
        Err(e) => return rust_string_to_c(format!("Error: writing data: {}", e).as_str()),
    }

    let json_data = json!(&intagrate_data);

    match write_data(json_data, path_test.clone()) {
        Ok(()) => {}
        Err(e) => return rust_string_to_c(format!("Error: writing data: {}", e).as_str()),
    }

    rust_string_to_c(format!("lambda: {}, cost: {}", lambda, cost).as_str())
}

fn forecast(
    z: &[f64],
    psi: &[f64],
    p: usize,
    q: usize,
    horizon: usize,
) -> Vec<f64> {
    let alpha = psi[0];
    let phi   = &psi[1..=p];
    let theta = &psi[p+1..];

    let mut eps_hat = Vec::with_capacity(z.len());
    for t in 0..z.len() {

        let mut e = z[t] - alpha;
        for i in 0..p {
            if t > i {
                e -= phi[i] * z[t-1-i];
            }
        }
        for j in 0..q {
            if t > j {
                e -= theta[j] * eps_hat[t-1-j];
            }
        }
        eps_hat.push(e);
    }

    let mut z_hat = z.to_vec();

    for _h in 1..=horizon {
        let t = z_hat.len();
        let mut val = alpha;
        for i in 0..p {
            val += phi[i] * z_hat[t-1-i];
        }
        for j in 0..q {
            val += theta[j] * eps_hat[t-1-j];
        }
        z_hat.push(val);
        eps_hat.push(0.0);
    }

    z_hat[z.len()..].to_vec()
}

fn train_test_split(data: &[f64]) -> (Vec<f64>, Vec<f64>) {
    let n = data.len();
    let split_idx = ((n as f64) * 0.7).round() as usize;

    let train = data[..split_idx].to_vec();
    let valid = data[split_idx..].to_vec();

    (train, valid)
}
