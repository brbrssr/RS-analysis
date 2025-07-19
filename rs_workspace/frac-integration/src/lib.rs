pub fn get_weights(d: f64, epsilon: f64) -> Vec<f64> {
    let mut w = vec![1.0];
    let mut k = 1;

    loop {
        let w_k = -w[k-1] * (d - (k as f64 - 1.)) / (k as f64);
        if w_k <= epsilon { break; }
        w.push(w_k);
        k += 1;
    }

    w
}

fn get_inv_weight(d: f64, epsilon: f64) -> Vec<f64> {
    let mut w = vec![1.0];
    let mut k = 1;
    loop {
        let w_k = w[k-1] * (d + (k as f64 - 1.)) / (k as f64);
        if w_k <= epsilon { break; }
        w.push(w_k);
        k += 1;
    }
    
    w
}

pub fn frac_diff(series: &[f64], d: f64, eplison: f64) -> Vec<f64> {
    let weights = get_weights(d, eplison);
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

pub fn reverse_frac_diff(series: &[f64], d: f64, eplison: f64) -> Vec<f64> {
    let weights = get_inv_weight(d, eplison);

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