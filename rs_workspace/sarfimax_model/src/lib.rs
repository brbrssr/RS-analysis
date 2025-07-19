use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyModule;
use numpy::{PyArray, PyArray1};
use s_h_esd::esd_test;
use box_cox::{box_cox, reverse_box_cox, z_score, reverse_z_score, calc_lambda};
use rs_analysis::calc_herst;
use frac_integration::{frac_diff,reverse_frac_diff};
use utils::{mean, var};
use pacf_acf::{durbin_yw, acf, significant_lags};
use arma::arma_optim;
use forecast::forecast;
use confidence_intervals::{estimate_noise_variance, impulse, confidence_intervals};


#[pyclass]
pub struct SARFIMAX {
    // Parameters of preparation
    freq: usize,
    alpha: Option<f64>,
    ub: Option<f64>,
    hybrid: Option<bool>,
    max_iter_box_cox: u64,
    min_window: usize,
    n_points: usize,
    nlags: usize,
    max_iters_optim: u64,
    optimizer: String,

    // Parameters of data
    mean: f64,
    std_dev: f64,
    lambda: f64,
    noise: f64,

    // Parameters of model [ARFIMA]
    p: usize,
    d: f64,
    q: usize,
    psi: Vec<f64>, // alpha = psi[0], phi = psi[1:=p] theta = psi[p+1:]
}


#[pymethods]
impl SARFIMAX {
    #[new]
    fn new(
        freq: usize,
        alpha: Option<f64>,
        ub: Option<f64>,
        hybrid: Option<bool>,
        max_iter_box_cox: u64,
        min_window: usize,
        n_points: usize,
        nlags: usize,
        max_iters_optim: u64,
        optimizer: String,
    ) -> Self {
        Self {
            freq,
            alpha,
            ub,
            hybrid,
            max_iter_box_cox,
            min_window,
            n_points,
            nlags,
            max_iters_optim,
            optimizer,

            mean: 0.0,
            std_dev: 0.0,
            lambda: 0.0,
            noise: 0.0,

            p: 0,
            d: 0.0,
            q: 0,
            psi: vec![],
        }
    }

    fn fit(&mut self, input: Vec<f64>) -> PyResult<()> {
        let mut data = input.clone();
        if data.is_empty() {
            return Err(PyValueError::new_err("Data shouldn't be empty"));
        }

        /*
            Remove anomalies from input data
         */
        let anomalies: Vec<usize> = esd_test(&data, self.freq, self.alpha, self.ub, self.hybrid);
        for anomaly in anomalies {
            if let (Some(&left),  Some(&right)) = (
                data.get(anomaly.wrapping_sub(1)), data.get(anomaly+1)
            ) {
                data[anomaly] = (left + right) / 2.0;
            }
        }

        /*
            Transformation of data [Box-Cox + z_score]
         */
        let (lambda, _cost, data) = match calc_lambda(data, self.max_iter_box_cox) {
            Ok(v) => (v[0][0], v[0][1], v[1].clone()),
            Err(e) => return Err(PyValueError::new_err(format!("{}", e))),
        };

        self.mean = mean(&data);
        self.std_dev = var(&data).sqrt();
        self.lambda = lambda;

        let data = box_cox(&data, self.lambda);
        let data = z_score(&data, self.mean, self.std_dev);

        /*
            Fractional difference
         */
        let (herst, _intercept) = match calc_herst(&data, self.min_window, self.n_points) {
            Ok(v) => v,
            Err(e) => return Err(PyValueError::new_err(format!("{}", e))),
        };

        self.d = herst - 0.5;
        let data = frac_diff(&data, self.d, 1e-5);

        /*
            Optimization of model
         */
        let thr = 1.96 / (data.len() as f64).sqrt();

        let pacf = durbin_yw(&data, self.nlags).pacf;
        let acf = acf(&data, self.nlags);

        let p0 = significant_lags(&pacf,thr);
        let q0 = significant_lags(&acf,thr);

        let (params, psi) = match arma_optim(&data, p0, q0, self.max_iters_optim, &self.optimizer) {
            Ok(v) => v,
            Err(e) => return Err(PyValueError::new_err(format!("{}", e))),
        };

        self.p = params[0];
        self.q = params[1];
        self.psi = psi;

        /*
            Compute estimate noise
         */

        self.noise = estimate_noise_variance(&data, &self.psi, self.p, self.q);

        Ok(())
    }

    fn forecast<'py>(
        &self,
        py: Python<'py>,
        data: Vec<f64>,
        horizon: usize
    ) -> PyResult<Bound<'py, PyArray1<f64>>> {
        let data = box_cox(&data, self.lambda);
        let data = z_score(&data, self.mean, self.std_dev);
        let data = frac_diff(&data, self.d, 1e-5);

        let preds = forecast(
            &data,
            &self.psi,
            self.p,
            self.q,
            horizon,
        );

        let preds = reverse_z_score(&preds, self.mean, self.std_dev);
        let preds = reverse_box_cox(&preds, self.lambda);
        let preds = reverse_frac_diff(&preds, self.d, 1e-1);

        Ok(PyArray::from_vec(py, preds))
    }

    fn confidence_intervals(
        &self,
        preds: Vec<f64>,
        horizon: usize,
        epsilon: f64,
        quantile: f64,
    ) -> PyResult<(Vec<f64>, Vec<f64>)> {
        let imp = impulse(horizon, &self.psi, self.p, self.d, self.q, epsilon);
        let (down_ci, top_ci) = confidence_intervals(&preds, &imp, self.noise.sqrt(), quantile);

        /* 
            reverse transform for down ci
         */
        let down_ci = reverse_z_score(&down_ci, self.mean, self.std_dev);
        let down_ci = reverse_box_cox(&down_ci, self.lambda);
        let down_ci = reverse_frac_diff(&down_ci, self.d, 1e-1);

        /*
            revese transform for top ci
         */
        let top_ci = reverse_z_score(&top_ci, self.mean, self.std_dev);
        let top_ci = reverse_box_cox(&top_ci, self.lambda);
        let top_ci = reverse_frac_diff(&top_ci, self.d, 1e-1);
        
       Ok((down_ci, top_ci))
    }

    fn get_mean(&self) ->PyResult<f64> {
        Ok(self.mean.clone())
    }

    fn get_std_dev(&self) ->  PyResult<f64> {
        Ok(self.std_dev)
    }

    fn get_lambda(&self) ->  PyResult<f64> {
        Ok(self.lambda)
    }

    fn get_noise(&self) -> PyResult<f64> {
        Ok(self.noise)
    }

    fn get_params<'py>(&self, py: Python<'py>) ->  PyResult<Bound<'py, PyArray1<f64>>> {
        let p = vec![self.p as f64, self.d, self.q as f64];
        Ok(PyArray::from_vec(py, p))
    }

    fn get_psi<'py>(&self, py: Python<'py>) ->  PyResult<Bound<'py, PyArray1<f64>>> {
        Ok(PyArray::from_vec(py, self.psi.clone()))
    }
}


#[pymodule]
fn sarfimax_model(py: Python<'_>, m: Bound<'_, PyModule>) -> PyResult<()> {
    m.add("sarfimax", py.get_type::<SARFIMAX>())?;
    Ok(())
}