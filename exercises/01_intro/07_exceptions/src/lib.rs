use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use pyo3::types::PyInt;

#[pyfunction]
// TODO: Implement a function that returns a list containing the first `n` numbers in Fibonacci's sequence.
//  It must raise a `TypeError` if `n` is not an integer or if it is less than 0.
fn fibonacci(n: Bound<'_, PyInt>) -> PyResult<Vec<u64>> {
    let n: usize = n
        .extract()
        .map_err(|_| PyTypeError::new_err("n must be an integer"))?;
    if n == 0 {
        return Ok(vec![]);
    }
    if n == 1 {
        return Ok(vec![0]);
    }
    let mut result = vec![0, 1];
    for i in 2..n {
        result.push(result[i - 1] + result[i - 2]);
    }
    Ok(result)
}

#[pymodule]
fn exceptions(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(fibonacci, m)?)?;
    Ok(())
}
