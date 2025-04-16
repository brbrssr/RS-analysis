use argmin::core::{CostFunction, Error, Executor};
use argmin::solver::brent::BrentOpt;

#[derive(Clone)]
struct BoxCoxLikelihood {
    data: Vec<f64>,
}

impl BoxCoxLikelihood {
    pub fn new(data: Vec<f64>) -> Self {
        Self { data }
    }

    fn transform(&self, lambda: f64) -> Vec<f64> {
        if lambda == 0.0 {
            self.data.iter().map(|y| y.ln()).collect()
        } else {
            self.data.iter().map(|y| (y.powf(lambda) - 1f64) / lambda).collect()
        }
    }
}

/*
    Optimize the negative logarithm of likelihood because the function requires argmax and trait implements argmin.

    F: -ell(lambda, mu, sigma2) = n/2 * Ln[2 * PI * sigma2] + 1/(2 * sigma2) Sum[z - mu)^2] ...
    ... - (lambda - 1) Sum[Ln[data]]

    where z : transformed data
 */
impl CostFunction for BoxCoxLikelihood {
    type Param = f64;
    type Output = f64;
    fn cost(&self, param: &Self::Param) -> Result<Self::Output, Error> {
        let lambda = if *param < 1e-12 {
            0.0
        } else {
            *param
        };

        let z = self.transform(lambda);
        let n = z.len() as f64;

        let mu = z.iter().sum::<f64>() / n;
        let sigma2 = z.iter().map(|z| (z - mu).powf(2.0)).sum::<f64>() / n;

        let sum_y_ln = self.data.iter().map(|y| y.ln()).sum::<f64>();

        let result = (n / 2.0) * sigma2.ln() - (lambda - 1.0) * sum_y_ln;

        Ok(result)
    }
}
pub fn box_cox_z_score(data: &[f64], max_iter: u64) -> Result<Vec<Vec<f64>>, Error> {
    let problem = BoxCoxLikelihood::new(data.to_vec());
    let problem_clone = problem.clone();

    let left_border = -5.0;
    let right_border = 5.0;

    let solver = BrentOpt::new(left_border, right_border);

    let res = match Executor::new(problem_clone, solver)
        .configure(|state| state.max_iters(max_iter))
        .run() {
        Ok(x) => {
            x
        },
        Err(e) => {
            return Err(e);
        }
    };

    let best_param = match res.state.best_param {
        Some(param) => param,
        None => {
            return Err(Error::msg("No best_param"));
        }
    };

    let best_cost = res.state.best_cost;


    let transformed_data = problem.transform(best_param);
    

    let mean = transformed_data.iter().sum::<f64>() / transformed_data.len() as f64;
    let variance = transformed_data.iter()
        .map(|&x| (x - mean).powi(2))
        .sum::<f64>() / transformed_data.len() as f64;
    let std_dev = variance.sqrt();


    let z_scores: Vec<f64> = transformed_data.iter()
        .map(|&x| (x - mean) / std_dev)
        .collect();


    Ok(vec![
        vec![best_param, best_cost],
        z_scores,
    ])

}