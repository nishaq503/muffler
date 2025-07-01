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
//! Undipervised denoising of time-series data.

pub(crate) mod data;
pub mod models;

use models::Classical;

use ndarray::prelude::*;
use smartcore::{api::SupervisedEstimator, linalg::basic::matrix::DenseMatrix};

/// Denoise a set of time-series samples.
///
/// # Parameters
///
/// * `samples`: The time-series samples to denoise.
/// * `window_size`: The number of elements in each window.
///
/// # Errors
///
/// * `window_size` is not an even number.
/// * `window_size` does not evenly divide the length of the time-series.
pub fn denoise<M, Mp, P>(
    samples: &Array2<f32>,
    window_size: usize,
    stride: usize,
    parameters: P,
) -> Result<Array2<f32>, String>
where
    M: Classical<Mp, P>,
    Mp: SupervisedEstimator<DenseMatrix<f32>, Vec<f32>, P> + Send + Sync,
    P: Clone + Send + Sync,
{
    let model = M::train(samples, window_size, stride, parameters)?;
    model.denoise(samples, stride)
}
