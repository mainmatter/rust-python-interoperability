use pyo3::{
    prelude::*,
    types::{PyDict, PyList},
};

#[pyfunction]
// You're given a Python list of non-negative numbers.
// You need to return a Python dictionary where the keys are the numbers in the list and the values
// are the unique prime factors of each number, sorted in ascending order.
//
// # Resources
//
// You can use `factors_uniq` from the `primes` crate to compute the prime factors of a number.
//
// # Constraints
//
// Don't hold the GIL while computing the prime factors
//
// # Fun additional challenge
//
// Can you use multiple threads to parallelize the computation?
// Consider using `rayon` to make it easier.
fn compute_prime_factors<'python>(
    python: Python<'python>,
    numbers: Bound<'python, PyList>,
) -> PyResult<Bound<'python, PyDict>> {
    todo!()
}

#[pymodule]
fn minimize(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(compute_prime_factors, m)?)?;
    Ok(())
}
