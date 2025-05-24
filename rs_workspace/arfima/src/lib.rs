use std::ffi::{c_char, CStr};
use std::fs;
use box_cox::{box_cox, z_score, reverse_box_cox_z_score};
use s_h_esd::esd_test;
use common::{file_clean, get_os, rust_string_to_c, write_data};
use serde_json::json;
use config_manager::ConfigManager;
use pacf_acf::{durbin_yw,acf,significant_lags};
use akaike_optim::grid;

fn get_weight(d: f64, border: usize) -> Vec<f64> {
    let mut w = vec![1.0];
    let mut k = 1;
    loop {
        let w_k = -w[k-1] * (d - (k as f64 - 1.)) / (k as f64);
        if k > border { break; }
        w.push(w_k);
        k += 1;
    }
    w
}

fn get_inv_weight(d: f64, border: usize) -> Vec<f64> {
    let mut v = vec![1.0];
    let mut k = 1;
    loop {
        let vk = v[k-1] * (d + (k as f64 - 1.)) / (k as f64);
        if k > border{ break; }
        v.push(vk);
        k += 1;
    }
    v
}

fn frac_diff(series: &[f64], d: f64, border: usize) -> Vec<f64> {
    let weights = get_weight(d, border);

    let k = weights.len() - 1;
    let mut result = Vec::with_capacity(series.len().saturating_sub(k));
    for t in k..series.len() {
        let mut sum = 0.0;
        for j in 0..=k {
            sum += weights[j] * series[t - j];
        }
        result.push(sum);
    }
    result
}

fn reverse_frac_diff(train: &[f64], preds: &[f64], d: f64, border: usize) -> Vec<f64> {
    let mut full_z = Vec::with_capacity(train.len() + preds.len());
    full_z.extend_from_slice(train);
    full_z.extend_from_slice(preds);

    let inv_w = get_inv_weight(d, border);
    let sum_w: f64 = inv_w.iter().sum();
    let weights: Vec<f64> = inv_w.iter().map(|&v| v / sum_w).collect();
    let max_lag = weights.len();

    let mut full_x = Vec::with_capacity(full_z.len());
    for t in 0..full_z.len() {
        let mut acc = 0.0;
        let max_j = t.min(max_lag - 1);
        for j in 0..=max_j {
            acc += weights[j] * full_z[t - j];
        }
        full_x.push(acc);
    }


    full_x[train.len()..].to_vec()
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

    let series: Vec<f64> = match serde_json::from_str::<Vec<f64>>(&json_data) {
        Ok(vec) => vec,
        Err(e) => {
            let msg = format!("Error parsing JSON in arfima preparation: {}", e);
            return rust_string_to_c(&msg);
        }
    };

    if series.is_empty() {
        return rust_string_to_c("Error: empty data series");
    }

    let (mut train, _test) = train_test_split(&series);

    let anomalies = esd_test(&series, freq, Some(alpha), Some(ub), Some(hybrid));

    for &i in &anomalies {
        if let (Some(&left), Some(&right)) = (
            train.get(i.wrapping_sub(1)),
            train.get(i + 1)
        ) {
            train[i] = (left + right) / 2.0;
        }
    }

    let (lambda, cost, data) = match box_cox(&train, max_iters) {
        Ok(v) => (v[0][0], v[0][1], v[1].clone()),
        Err(e) => return rust_string_to_c(format!("[Error] {}", e).as_str()),
    };

    let rv_mu = common::mean(&data);
    let rv_std_dev = common::variance(&data).sqrt();

    let z_data = z_score(&data);

    match file_clean(path_test.clone()) {
        Ok(()) => {}
        Err(e) => return rust_string_to_c(format!("Error: cleaning file: {}", e).as_str()),
    }

    match file_clean("./data/preds.json".to_string().clone()) {
        Ok(()) => {}
        Err(e) => return rust_string_to_c(format!("Error: cleaning file: {}", e).as_str()),
    }
    match file_clean("./data/confidence_intervals.json".to_string().clone()) {
        Ok(()) => {}
        Err(e) => return rust_string_to_c(format!("Error: cleaning file: {}", e).as_str()),
    }

    let _config = match ConfigManager::get_config(){
        Ok(config) => config,
        Err(e) => return rust_string_to_c(format!("{}", e).as_str()),
    };

    let d = _config.herst - 0.5;
    let horizon = 100;
    let border = (series.len() as f64 * 0.1).ceil() as usize;

    let intagrate_data = frac_diff(&z_data, d, border);

    let thr = 1.96 / (intagrate_data.len() as f64).sqrt();

    let pacf = durbin_yw(&intagrate_data, nlags).pacf;
    let acf = acf(&intagrate_data, nlags);

    let p0 = significant_lags(&pacf,thr);
    let q0 = significant_lags(&acf,thr);


    println!("p0: {:?} \n q0: {:?}", p0, q0);
    let (params, psi) = match grid(&intagrate_data, p0, q0, max_iters_grid) {
        Ok(v) => v,
        Err(e) => return rust_string_to_c(format!("[Error] Error fitting ARIMA: {}", e).as_str()),
    };

    let p = params[0];
    let q = params[1];
    let sigma2= estimate_noise_variance(&intagrate_data, &psi, p, q);

    let preds = forecast(&intagrate_data, &psi, p, q, horizon);

    let no_frac_diff = reverse_frac_diff(&intagrate_data, &preds, d, border);
    let result = reverse_box_cox_z_score(&no_frac_diff, rv_mu, rv_std_dev, lambda);
    
    println!("params: {:?} \n psi: {:?}", params, psi);

    let imp = impulse(
        horizon,
        &psi[1..(p+1)],
        &psi[(p+1)..(p+1+q)],
        d,
        border,
    );

    let (down_conf_intervals, top_conf_intervals): (Vec<f64>, Vec<f64>) = confidence_intervals(
        &preds,
        &imp,
        sigma2,
        1.96,
    );

    let no_diff_down_ci = reverse_frac_diff(&intagrate_data,&down_conf_intervals,d,border);
    let rev_down_ci = reverse_box_cox_z_score(&no_diff_down_ci,rv_mu,rv_std_dev,lambda);
    let rev_down_ci = rev_down_ci.iter().map(|&x| x * rv_std_dev + rv_mu).collect::<Vec<f64>>();

    let no_diff_top_ci = reverse_frac_diff(&intagrate_data,&top_conf_intervals,d,border);
    let rev_top_ci = reverse_box_cox_z_score(&no_diff_top_ci,rv_mu,rv_std_dev,lambda);
    let rev_top_ci = rev_top_ci.iter().map(|&x| x * rv_std_dev + rv_mu).collect::<Vec<f64>>();

    let ci = rev_down_ci.iter().zip(rev_top_ci.iter()).map(|(&d, &t)| (d,t)).collect::<Vec<(f64,f64)>>();

    let preds_json = json![&result];

    match write_data(preds_json, "./data/preds.json".to_string().clone()) {
        Ok(()) => {}
        Err(e) => return rust_string_to_c(format!("Error: writing data: {}", e).as_str()),
    }

    let json_data = json!(&intagrate_data);

    match write_data(json_data, path_test.clone()) {
        Ok(()) => {}
        Err(e) => return rust_string_to_c(format!("Error: writing data: {}", e).as_str()),
    }

    let conf_intervals_json = json![&ci];

    match write_data(conf_intervals_json, "./data/confidence_intervals.json".to_string().clone()) {
        Ok(()) => {}
        Err(e) => return rust_string_to_c(format!("Error: writing data: {}", e).as_str()),
    }

    rust_string_to_c(format!("lambda: {}, cost: {}", lambda, cost).as_str());
    rust_string_to_c("Where is my mind?")
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
    let split_idx = ((n as f64) * 0.99).round() as usize;

    let train = data[..split_idx].to_vec();
    let valid = data[split_idx..].to_vec();

    (train, valid)
}

pub fn impulse(horizon: usize, phi: &[f64], theta: &[f64], d: f64, border: usize) -> Vec<f64> {
    let p = phi.len();
    let q = theta.len();
    let h = horizon;

    let mut w = get_weight(d, border);
    if w.len() < h + 1 {
        w.resize(h + 1, 0.0);
    }

    let mut pi = vec![0.0; h + 1];
    pi[0] = 1.0;
    for m in 1..=h {
        let end = p.min(m);
        let sum: f64 = phi.iter()
            .take(end)
            .enumerate()
            .map(|(i, &ph)| ph * pi[m - i - 1])
            .sum();
        pi[m] = sum;
    }

    let mut u = Vec::with_capacity(h + 1);
    for m in 0..=h {
        let mut sum = w[m];
        let end = q.min(m);
        for i in 1..=end {
            sum += theta[i - 1] * w[m - i];
        }
        u.push(sum);
    }

    let mut psi = vec![0.0; h + 1];
    for j in 0..=h {
        let acc: f64 = (0..=j).map(|m| u[m] * pi[j - m]).sum();
        psi[j] = acc;
    }
    psi
}

pub fn estimate_noise_variance(z: &[f64], psi: &[f64], p: usize, q: usize) -> f64 {
    let alpha = psi[0];
    let phi   = &psi[1..=p];
    let theta = &psi[p+1..];
    let t_max = z.len();

    let mut eps = Vec::with_capacity(t_max);
    for t in 0..t_max {
        let mut e = z[t] - alpha;
        // AR
        for i in 0..p {
            if t > i {
                e -= phi[i] * z[t-1-i];
            }
        }
        // MA
        for j in 0..q {
            if t > j {
                e -= theta[j] * eps[t-1-j];
            }
        }
        eps.push(e);
    }

    let ssr: f64 = eps.iter().map(|&e| e * e).sum();
    ssr / (t_max as f64)
}

fn confidence_intervals(
    preds: &[f64],
    impulse: &[f64],
    sigma2: f64,
    z: f64,
) -> (Vec<f64>, Vec<f64>) {
    let sigma = sigma2.sqrt();
    let mut ci_top = Vec::with_capacity(preds.len());
    let mut ci_down = Vec::with_capacity(preds.len());
    let mut acc_var = 0.0;

    for (h, &yhat) in preds.iter().enumerate() {
        if h < impulse.len() {
            acc_var += impulse[h] * impulse[h];
        }
        let stderr = sigma * acc_var.sqrt();
        let lower = yhat - z * stderr;
        let upper = yhat + z * stderr;
        ci_top.push(upper);
        ci_down.push(lower);
    }

    (ci_down, ci_top)
}