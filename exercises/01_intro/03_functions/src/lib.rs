//! Expose a function named `no_op` from Rust to get the tests in `test_sample.py` to pass.
use pyo3::prelude::*;

#[pymodule]
fn functions(m: &Bound<'_, PyModule>) -> PyResult<()> {
    Ok(())
}
