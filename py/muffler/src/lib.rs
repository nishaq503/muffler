use pyo3::prelude::*;

use muffler_python::{denoise_decision_tree, denoise_linear_regression};

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn _core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    pyo3_log::init();

    m.add_function(wrap_pyfunction!(denoise_decision_tree, m)?)?;
    m.add_function(wrap_pyfunction!(denoise_linear_regression, m)?)?;
    Ok(())
}
