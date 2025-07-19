use argmin::core::{CostFunction, Error, Executor};
use argmin::solver::brent::BrentOpt;
use utils::{var};


const EPSILON: f64 = 1e-12;

#[derive(Clone)]
struct BoxCoxMLE {
    data: Vec<f64>,
}

impl BoxCoxMLE {
    fn new(data: Vec<f64>) -> Self {
        Self { data }
    }

    fn transform(&self, lambda: f64) -> Vec<f64> {
        if lambda <= EPSILON {
            self.data.iter().map(|x| x.ln()).collect()
        } else {
            self.data.iter().map(|x| (x.powf(lambda) - 1.0) / lambda).collect()
        }
    }
}

impl CostFunction for BoxCoxMLE {
    type Param = f64;
    type Output = f64;
    fn cost(&self, p: &Self::Param) -> Result<Self::Output, Error> {
        let lambda = *p;

        let z =self.transform(lambda);
        let n = z.len() as f64;

        let sigma2 = var(&z);

        let sum_y_ln = self.data.iter().map(|x| x.ln()).sum::<f64>();
        let result = (n / 2.0) * sigma2.ln() - (lambda - 1.0) * sum_y_ln;

        Ok(result)
    }
}

pub fn calc_lambda(data: Vec<f64>, max_iters: u64) -> Result<Vec<Vec<f64>>, Error> {
    let problem = BoxCoxMLE::new(data.to_vec());
    let problem_cloned = problem.clone();

    let left_border = -5.0;
    let right_border = 5.0;

    let solver = BrentOpt::new(left_border, right_border);

    let res = match Executor::new(problem_cloned, solver)
        .configure(|state| state.max_iters(max_iters))
        .run() {
        Ok(res) => res,
        Err(e) => return Err(e),
    };

    let best_param = match res.state.best_param {
        Some(best_param) => best_param,
        None => return Err(Error::msg("No best_param for box cox transformation")),
    };

    let best_cost = res.state.best_cost;
    let transformed_data = problem.transform(best_param);

    Ok(vec![
        vec![best_param, best_cost], transformed_data,
    ])
}

pub fn box_cox(data: &[f64], lambda: f64) -> Vec<f64> {
    let result = BoxCoxMLE::new(data.to_vec());
    result.transform(lambda)
}

pub fn z_score(data: &[f64], mean: f64, std_dev: f64) -> Vec<f64> {
    let z_scores = data.iter()
        .map(|x| (x-mean) / std_dev)
        .collect::<Vec<f64>>();

    z_scores
}

pub fn reverse_box_cox(z: &[f64], lambda: f64) -> Vec<f64> {
    z.iter()
        .map(|zi| {
            if lambda <= EPSILON {
                zi.exp()
            } else {
                (lambda * zi + 1.0).powf(1.0 / lambda)
            }
        }).collect::<Vec<f64>>()
}

pub fn reverse_z_score(z: &[f64], mu: f64, std_dev: f64) -> Vec<f64> {
    z.iter().map(|zi| zi * std_dev + mu).collect::<Vec<f64>>()
}