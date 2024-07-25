use pyo3::prelude::*;

#[pyclass(frozen)]
struct Rectangle {
    width: u32,
    length: u32,
}

#[pymethods]
impl Rectangle {
    #[new]
    fn new(width: u32, length: u32) -> Self {
        Self { width, length }
    }
}

#[pyfunction]
/// Compute the area of a rectangle while allowing Python to run other threads.
/// Fill in the body of the function.
/// Modify `Rectangle`'s definition if necessary.
///
/// # Constraints
///
/// Do NOT remove the `allow_threads` call. The computation must be done inside
/// the closure passed to `allow_threads`.
fn compute_area<'py>(python: Python<'py>, shape: Bound<'py, Rectangle>) -> u32 {
    let shape = shape.unbind();
    python.allow_threads(|| {
        let shape = shape.get();
        shape.width * shape.length
    })
}

#[pymodule]
fn immutable(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(compute_area, m)?)?;
    m.add_class::<Rectangle>()?;
    Ok(())
}
