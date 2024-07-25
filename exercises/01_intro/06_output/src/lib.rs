use pyo3::prelude::*;

#[pyfunction]
// TODO: Implement a function that returns a list containing the first `n` numbers in Fibonacci's sequence.
fn fibonacci(/* n: TODO */) /* -> TODO */ {
    todo!()
}

#[pymodule]
fn output(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(fibonacci, m)?)?;
    Ok(())
}
