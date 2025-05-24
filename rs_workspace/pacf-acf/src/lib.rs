use common::{autocov, variance};
use std::f64;

pub fn acf(series: &[f64], nlags: usize) -> Vec<f64> {
    let var = variance(series);
    let acf:Vec<f64> = autocov(series, nlags).iter().map(|&cov| cov / var).collect();

    acf
}

pub fn significant_lags(vals: &[f64], threshold: f64) -> Vec<usize> {
    vals.iter()
        .enumerate()
        .filter_map(|(idx, &v)| {
            if idx >= 1 && v.abs() > threshold {
                Some(idx)
            } else {
                None
            }
        })
        .collect()
}


pub struct YwResult {
    pub pacf: Vec<f64>,
    pub phi: Vec<Vec<f64>>,
}


pub fn durbin_yw(series: &[f64], nlags: usize) -> YwResult {

    let n = series.len();
    let mean = series.iter().sum::<f64>() / n as f64;
    let mut gamma = vec![0.0; nlags + 1];
    for k in 0..=nlags {
        let mut sum = 0.0;
        for t in k..n {
            sum += (series[t] - mean) * (series[t - k] - mean);
        }
        gamma[k] = sum / n as f64;
    }

    let mut pacf = vec![0.0; nlags + 1];
    let mut phi = vec![vec![0.0; nlags + 1]; nlags + 1];
    let mut sigma2 = vec![0.0; nlags + 1];
    sigma2[0] = gamma[0];
    pacf[0] = 1.0;

    for lag in 1..=nlags {

        let mut acc = 0.0;
        for j in 1..lag {
            acc += phi[lag - 1][j] * gamma[lag - j];
        }
        let kappa = (gamma[lag] - acc) / sigma2[lag - 1];
        pacf[lag] = kappa;
        phi[lag][lag] = kappa;

        for j in 1..lag {
            phi[lag][j] = phi[lag - 1][j] - kappa * phi[lag - 1][lag - j];
        }

        sigma2[lag] = sigma2[lag - 1] * (1.0 - kappa * kappa);
    }

    YwResult { pacf, phi }
}