//! Expose a function named `no_op` from Rust to get the tests in `test_sample.py` to pass.
use pyo3::prelude::*;

#[pyfunction]
fn no_op() {}

#[pymodule]
fn functions(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(no_op, m)?)?;
    Ok(())
}
