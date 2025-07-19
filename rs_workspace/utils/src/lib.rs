pub fn mean_squared_error(y_real: &[f64], y_pred: &[f64]) -> f64 {
    if  y_real.len() != y_pred.len() || y_real.len() == 0 {
        return 0.0;
    }

    let n = y_real.len() as f64;
    let mse = y_real.iter().zip(y_pred).map(|(r,p)| (r-p).powf(2.0)).sum::<f64>();
    mse / n
}

pub fn mean(x: &[f64]) -> f64 {
    if  x.len() == 0 {
        return 0.0;
    }
    x.iter().sum::<f64>() / x.len() as f64
}

pub fn var(x: &[f64]) -> f64 {
    if  x.len() == 0 {
        return 0.0;
    }
    let mu = mean(x);
    x.iter().map(|x| (x-mu).powf(2.0)).sum::<f64>() / x.len() as f64
}

pub fn median(x: &[f64]) -> f64 {
    if  x.len() == 0 {
        return 0.0;
    }
    let mut res = x.to_vec();
    res.sort_by(|x,y| x.partial_cmp(y).unwrap());
    let n =  res.len();
    if n % 2 == 0 {
        let mid =  n / 2;
        (res[mid-1] + res[mid]) / 2.0
    } else {
        res[n / 2]
    }
}

pub fn autocov(x: &[f64], nlags: usize) -> Vec<f64> {
    let n =  x.len();
    if n == 0 { 
        return vec![];
    }
    let max_lag = nlags.min(n-1);
    let mean = mean(x);
    let mut gamma= Vec::new();
    
    for lag in 0..max_lag {
        let cov = (0..(n-lag))
            .map(|i| (x[i] - mean) * (x[lag+i] - mean))
            .sum::<f64>()
            / n as f64;
        gamma.push(cov);
    }
    gamma
}
