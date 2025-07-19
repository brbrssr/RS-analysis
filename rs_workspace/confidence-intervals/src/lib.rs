use frac_integration::get_weights;


pub fn estimate_noise_variance(
    data: &[f64],
    psi: &[f64],
    p: usize,
    q: usize,
) -> f64 {
    let alpha = psi[0];
    let phi = &psi[1..=p];
    let theta = &psi[p+1..];
    let length = data.len();

    let mut eps: Vec<f64> = Vec::with_capacity(length);
    for t in 0..length {
        let mut e = data[t] - alpha;

        //AR component
        for i in 0..p {
            if t > i {
                e -= phi[i] * data[t - 1 - i];
            }
        }

        // MA component 
        for i in 0..q {
            if t > i {
                e -= theta[i] * eps[t-1-i];
            }
        }
        eps.push(e);
    }

    let ssr: f64 = eps.iter().map(|&e| e * e).sum();
    ssr / length as f64
}

pub fn impulse(
    horizon: usize,
    psi: &[f64],
    p: usize,
    d: f64,
    q: usize,
    epsilon: f64,
) -> Vec<f64> {
    let phi = &psi[1..=p];
    let theta = &psi[p+1..];    

    let mut w = get_weights(d, epsilon);
    if w.len() < horizon + 1 {
        w.resize(horizon+1, 0.0);
    }

    let mut pi = vec![0.0; horizon+1];
    pi[0] = 1.0;
    for i in 0..=horizon {
        let end = p.min(i);
        let sum: f64 = phi.iter()
            .take(end)
            .enumerate()
            .map(|(j, &phi)| phi * pi [i - j - 1])
            .sum();
    pi[i] = sum;
    }

   let mut u = Vec::with_capacity(horizon+1);
   for i in 0..=horizon {
    let mut sum = w[i];
    let end = q.min(i);
    for j in 1..=end {
        sum += theta[j-1] * w[i-j];
    }
    u.push(sum);
   } 

   let mut imp = vec![0.0; horizon+1];
   for i in 0..=horizon {
        let acc: f64 = (0..=i).map(|j| u[j] * pi[i-j]).sum();
        imp[i] = acc;
   }

    imp
}

pub fn confidence_intervals(
    preds: &[f64],
    impulse: &[f64],
    sigma: f64,
    q: f64,
) -> (Vec<f64>, Vec<f64>) {
    let mut ci_top = Vec::with_capacity(preds.len());
    let mut ci_down = Vec::with_capacity(preds.len());
    let mut acc = 0.0;

    for (h, y) in preds.iter().enumerate() {
        if h < impulse.len() {
            acc += impulse[h] * impulse[h];
        }

        let err = sigma * acc.sqrt();
        let lower = y - q * err;
        let upper = y + q * err;
        ci_top.push(upper);
        ci_down.push(lower);
    }

    (ci_down, ci_top)
}