#![deny(clippy::correctness)]
#![warn(
    missing_docs,
    clippy::all,
    clippy::suspicious,
    clippy::style,
    clippy::complexity,
    clippy::perf,
    clippy::pedantic,
    clippy::nursery,
    clippy::missing_docs_in_private_items,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::cast_lossless
)]
//! Python bindings for the Muffler crate.

use ::muffler::{
    denoise,
    models::{DTModel, LRModel},
};
use numpy::{IntoPyArray, PyArray2, PyReadonlyArray2};
use pyo3::{exceptions::PyValueError, prelude::*};
use smartcore::{
    linear::linear_regression::LinearRegressionParameters,
    tree::decision_tree_regressor::DecisionTreeRegressorParameters,
};

/// Denoise a set of time-series samples using linear regression.
///
/// # Parameters
///
/// * `samples`: The time-series samples to denoise.
/// * `window_size`: The number of elements in each window.
/// * `stride`: The number of elements to skip between windows.
///
/// # Errors
///
/// * Shape mismatch. Should not happen.
///
/// # Returns
///
/// The denoised samples.
#[allow(clippy::needless_pass_by_value)]
#[pyfunction]
pub fn denoise_linear_regression<'py>(
    py: Python<'py>,
    samples: PyReadonlyArray2<'py, f32>,
    window_size: usize,
    stride: usize,
) -> PyResult<Bound<'py, PyArray2<f32>>> {
    let samples = samples.as_array().to_owned();
    denoise::<LRModel, _, _>(&samples, window_size, stride, LinearRegressionParameters::default())
        .map(|x| x.into_pyarray(py))
        .map_err(PyValueError::new_err)
}

/// Denoise a set of time-series samples using decision trees.
///
/// # Parameters
///
/// * `samples`: The time-series samples to denoise.
/// * `window_size`: The number of elements in each window.
/// * `stride`: The number of elements to skip between windows.
///
/// # Errors
///
/// * Shape mismatch. Should not happen.
///
/// # Returns
///
/// The denoised samples.
#[allow(clippy::needless_pass_by_value)]
#[pyfunction]
pub fn denoise_decision_tree<'py>(
    py: Python<'py>,
    samples: PyReadonlyArray2<'py, f32>,
    window_size: usize,
    stride: usize,
) -> PyResult<Bound<'py, PyArray2<f32>>> {
    let samples = samples.as_array().to_owned();
    let params = DecisionTreeRegressorParameters {
        max_depth: Some(8),
        min_samples_leaf: 1,
        min_samples_split: 2,
        seed: None,
    };
    denoise::<DTModel, _, _>(&samples, window_size, stride, params)
        .map(|x| x.into_pyarray(py))
        .map_err(PyValueError::new_err)
}
