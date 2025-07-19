use statrs::distribution::{StudentsT, ContinuousCDF};
use utils::{mean, var, median};
use std::f64;


fn t_ppf(prob: f64, df: f64) -> f64 {
    let t_dist = StudentsT::new(0.0, 1.0, df).unwrap();
    t_dist.inverse_cdf(prob)
}

fn mad(x: &[f64]) -> f64 {
    let med = median(x);
    let abs_dev = x.iter().map(|x|  (x - med).abs()).collect::<Vec<f64>>();
    median(&abs_dev)
}

fn seasonal_mean(x: &[f64], freq: usize) -> Vec<f64> {
    let mut means = Vec::with_capacity(freq);
    for i in 0..freq { 
        let values = x.iter().enumerate()
            .filter(|(idx, _)| idx % freq == i)
            .map(|(_, &val)| val)
            .collect::<Vec<f64>>();
        
        let mean = if values.is_empty() {
            f64::NAN
        } else {
            values.iter().sum::<f64>() / values.len() as f64
        };
        means.push(mean);
    }
    
    means
}

fn ts_s_md_decomposition(x: &[f64], freq: usize) -> Vec<f64> {
    let nobs =  x.len();
    
    let period_averages = seasonal_mean(&x, freq);
    let mut seasonal = Vec::with_capacity(nobs);
    while seasonal.len() < nobs {
        seasonal.extend_from_slice(&period_averages);
    }
    seasonal.truncate(nobs);
    
    let med_value = median(&x);
    let median_vec = vec![med_value; nobs];
    
    let residual = x.iter()
        .zip(seasonal.iter())
        .zip(median_vec.iter())
        .map(|((&obs, &s), &m)| obs - s - m)
        .collect::<Vec<f64>>();
    
    residual
}

fn esd_test_statistics(x: &[f64], hybrid: Option<bool>) -> (f64, f64) {
    let hybrid =  hybrid.unwrap_or(true);
    
    if hybrid {
        (median(x), mad(x))
    } else {
        (mean(x), var(x).sqrt())
    }
}

pub fn esd_test(
    x: &[f64], 
    freq: usize, 
    alpha: Option<f64>, 
    ub: Option<f64>, 
    hybrid: Option<bool>
) -> Vec<usize> {
    let mut ub = ub.unwrap_or(0.499);
    let alpha = alpha.unwrap_or(0.95);
    let hybrid = hybrid.unwrap_or(true);

    if ub > 0.4999 {
        ub = 0.499;
    }
    
    let nobs = x.len();
    let k = (ub * nobs as f64).floor().max(1.0) as usize;
    
    let residuals = ts_s_md_decomposition(x, freq);
    let mut res: Vec<Option<f64>> = residuals.into_iter().map(Some).collect();
    let mut anomalies = Vec::new();

    for i in 1..=k {
        let unmasked: Vec<f64> = res.iter()
            .filter_map(|&val| val)
            .collect();
        if unmasked.is_empty() {
            break;
        }

        let (location, dispersion) = esd_test_statistics(&unmasked, Some(hybrid));

        let tmp: Vec<f64> = res.iter().map(|opt| {
            match opt {
                Some(val) => (val - location).abs() / dispersion,
                None => -f64::INFINITY,
            }
        }).collect();

        let idx_opt = tmp.iter().enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .map(|(index, _)| index);

        if let Some(idx) = idx_opt {
            let test_statistic = tmp[idx];

            let n = res.iter().filter(|v| v.is_some()).count();

            if n <= i + 1 {
                break;
            }
            let df = (n - i - 1) as f64;
            let t_val = t_ppf(alpha, df);
            let critical_value = ((n - i) as f64 * t_val)
                / (((n - i - 1) as f64 + t_val.powi(2)) * (n - i - 1) as f64).sqrt();

            if test_statistic > critical_value {
                anomalies.push(idx);
            }

            res[idx] = None;
        }
    }

    anomalies
}