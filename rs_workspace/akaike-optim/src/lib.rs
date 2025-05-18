use argmin::solver::quasinewton::LBFGS;
use argmin::solver::linesearch::BacktrackingLineSearch;
use pacf_acf::durbin_yw;
use argmin::core::{Gradient, CostFunction, Executor, IterState};
use argmin::core::Error;
use argmin::solver::linesearch::condition::ArmijoCondition;


#[derive(Clone)]
struct ARMA {
    z:  Vec<f64>,
    p: usize,
    q:  usize,
}

impl ARMA {
    fn new(z: Vec<f64>, p: usize, q: usize)  -> ARMA {Self {z,p,q}}
}

impl CostFunction for ARMA {
    type Param = Vec<f64>;
    type Output = f64;

    fn cost(&self, psi: &Self::Param) -> Result<Self::Output, Error> {
        if psi.iter().any(|&x| !x.is_finite()) {
            return Ok(1e12);
        }

        let alpha: f64 = psi[0];
        let phi: Vec<f64> = psi[1..=self.p].iter().map(|&x| 0.99 * x.tanh()).collect();
        let theta: Vec<f64> = psi[self.p+1..].iter().map(|&x| 0.99 * x.tanh()).collect();
        let t_max = self.z.len();

        let mut residuals: Vec<f64> = Vec::new();
        let mut past_eps = vec![0.0; self.q.max(1)];

        for t in 0..t_max {
            let mut eps_t = self.z[t] - alpha;

            // AR component
            for i in 0..self.p {
                if t > i {
                    eps_t -= phi[i] * self.z[t - i - 1];
                }
            }

            // MA component
            for j in 0..self.q {
                if t > j {
                    eps_t -= theta[j] * past_eps[t - j - 1];
                }
            }

            residuals.push(eps_t);
            if t < past_eps.len() {
                past_eps[t] = eps_t;
            } else {
                past_eps.push(eps_t);
            }
        }

        if residuals.iter().any(|&e| !e.is_finite()) {
            return Ok(1e12);
        }

        let ssr = residuals.iter().map(|&x| x.powf(2.0)).sum::<f64>();

        let sigma2: f64 = if ssr > 0.0 {
            ssr / t_max as f64
        } else  { 1.0 };


        let log_likelihood = -0.5 * t_max as f64 * ( (2.0 * std::f64::consts::PI).ln() + sigma2.ln() )
            - ssr / (2.0 * sigma2);

        Ok(-log_likelihood)
    }
}

impl Gradient for ARMA {
    type Param = Vec<f64>;
    type Gradient = Vec<f64>;

    fn gradient(&self, psi: &Self::Param) -> Result<Self::Param, Error> {
        if psi.iter().any(|&x| !x.is_finite()) {
            return Ok(vec![0.0; psi.len()]);
        }

        let alpha = psi[0];
        let raw_phi = &psi[1..=self.p];
        let raw_theta = &psi[self.p+1..];
        let c = 0.99;

        let phi: Vec<f64> = raw_phi.iter().map(|&r| c * r.tanh()).collect();
        let theta: Vec<f64> = raw_theta.iter().map(|&r| c * r.tanh()).collect();


        let t_max = self.z.len();
        let mut eps = Vec::with_capacity(t_max);
        for t in 0..t_max {
            let mut e = self.z[t] - alpha;
            for i in 0..self.p {
                if t > i { e -= phi[i] * self.z[t-1-i]; }
            }
            for j in 0..self.q {
                if t > j { e -= theta[j] * eps[t-1-j]; }
            }

            if !e.is_finite() { return Ok(vec![0.0; psi.len()]); }
            eps.push(e);
        }
        let ssr: f64 = eps.iter().map(|&e| e*e).sum();

        let t = t_max as f64;
        let sigma2 = (ssr / t).max(1e-12);
        let inv2 = 1.0 / sigma2;


        let mut grad = vec![0.0; 1 + self.p + self.q];

        grad[0] = -inv2 * eps.iter().sum::<f64>();

        for i in 0..self.p {
            let acc = (i+1..t_max).map(|t| eps[t] * self.z[t-1-i]).sum::<f64>();
            grad[1 + i] = -inv2 * acc;
        }

        for j in 0..self.q {
            let acc = (j+1..t_max).map(|t| eps[t] * eps[t-1-j]).sum::<f64>();
            grad[1 + self.p + j] = -inv2 * acc;
        }

        let mut grad_psi = vec![0.0; psi.len()];
        grad_psi[0] = grad[0];

        for i in 0..self.p {
            let d_tanh = c * (1.0 - raw_phi[i].tanh().powi(2));
            grad_psi[1 + i] = grad[1 + i] * d_tanh;
        }

        for j in 0..self.q {
            let d_tanh = c * (1.0 - raw_theta[j].tanh().powi(2));
            grad_psi[1 + self.p + j] = grad[1 + self.p + j] * d_tanh;
        }

        Ok(grad_psi)
    }
}

pub fn grid(data: &[f64], p_vec: Vec<usize>, q_vec: Vec<usize>, max_iters: u64) -> Result<(Vec<usize>, Vec<f64>), Error> {

    let t =  data.len() as f64;
    println!("t: {}",t);

    let mut best_aic = std::f64::INFINITY;
    let mut best_params = Vec::new();
    let mut best_psi =  Vec::new();
    
    for p in p_vec.iter() {
        for q in q_vec.iter()  {

            println!("Now: [p: {}, q: {}]", p, q);

            let problem = ARMA::new(data.to_vec(), *p, *q);

            let condition = ArmijoCondition::new(1e-4)?;
            let ls = BacktrackingLineSearch::new(condition);

            let solver = LBFGS::new(ls,5);

            let res = durbin_yw(&data, *p);
            let phi_init: Vec<f64> = res.phi[*p][1..=*p].to_vec();
            let theta_init_value = 0.01;
            let mut psi: Vec<f64> = Vec::with_capacity(1 + p + q);

            psi.push(0.0);
            psi.extend(phi_init.iter().cloned());
            psi.extend((0..*q).map(|_| theta_init_value));

            let executor = Executor::new(problem, solver)
                .configure(|state: IterState<Vec<f64>, Vec<f64>, (), (), (), f64>| {
                    state
                        .param(psi.clone())
                        .max_iters(max_iters)
                })
                .run()?;

            let mut debug_flag = false;
            let k = (p + q + 1) as f64;
            let aic = if t / k < 40.0 {
                let base = executor.state.get_best_cost() * 2.0 + 2.0 * k;
                base + 2.0 * k * (k + 1.0) / (t - k - 1.0)
            } else {
                debug_flag = true;
                executor.state.get_best_cost() * 2.0 + 2.0 * k
            };

            if debug_flag {
                println!("AIC {}", aic);
            } else { println!("AIC_c {}", aic);}


            if aic < best_aic {
                best_aic = aic;
                best_psi = executor.state.best_param.unwrap();
                best_params = vec![*p,*q];
            }
        }
    }
    let alpha = best_psi[0];
    best_psi = best_psi.iter().map(|&p| p.tanh() * 0.99).collect::<Vec<f64>>();
    best_psi[0] = alpha;
    Ok((best_params, best_psi))
}