use pyo3::prelude::*;

#[pyfunction]
// Modify this function to release the GIL while computing the nth prime number.
fn nth_prime(python: Python<'_>, n: u64) -> u64 {
    python.allow_threads(|| {
        let mut count = 0;
        let mut num = 2; // Start checking primes from 2
        while count < n {
            if is_prime(num) {
                count += 1;
            }
            num += 1;
        }
        num - 1 // Subtract 1 because we increment after finding the nth prime
    })
}

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    for i in 2..=(n as f64).sqrt() as u64 {
        if n % i == 0 {
            return false;
        }
    }
    true
}

#[pymodule]
fn release(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(nth_prime, m)?)?;
    Ok(())
}
